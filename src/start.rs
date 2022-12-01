use crate::{
    consts::cpu,
    riscv::{clint, medeleg, mepc, mhartid, mideleg, mret::mret, mstatus, pmp, satp, sie},
};

#[no_mangle]
pub fn start() -> ! {
    // crate::rust_main::rust_main();

    // MPP を Supervisorモード に設定
    mstatus::set_mpp(mstatus::MPP::Supervisor);

    /* この時点では、まだ Machineモード */

    // mepc に rust_main のアドレスを設定(マシンモード限定)
    // タイマー割り込みが発生した後に rust_main を実行するように設定
    // as usize で関数のポインタが取れる...?
    mepc::write_mepc(crate::rust_main::rust_main as usize);

    // 仮想アドレスへの変換を無効化(なぜ...?)
    // TODO: なぜここで無効化するのか調べる
    satp::write_satp(0);

    // 例外と割り込みイベントの発生時に Supervisorモード に遷移するように設定(RISC-V のデフォルトでは Machine モードに遷移する)
    // mideleg と medeleg の各ビットはどの例外や割り込みイベントを Supervisorモード に移譲するかを設定する。
    // (0xffff = 0b1111_1111_1111_1111 → すべての例外と割り込みを指定)
    mideleg::write_mideleg(0xffff);
    medeleg::write_medeleg(0xffff);

    // 外部割り込み, タイマー割り込み, ソフトウェア割り込みを有効化
    let mut sie = sie::read_sie();
    sie |= sie::SIE::External as usize;
    sie |= sie::SIE::Timer as usize;
    sie |= sie::SIE::Software as usize;
    sie::write_sie(sie);

    // PMP(物理メモリ保護) の設定
    // PMP は Machine モードで設定し Supervisor や User モードで作用する。
    // 0xf  = 0b00001111
    // Lock = 0  ← PMP をロックする(Machine モードでも変更不可能になる)
    // WIRI = 0  ← 謎
    // A    = 01 ← アドレスマッチング方式を TOR (Top of Range - 0 から pmpaddr0 に適応) に設定
    // X    = 1  ← 実行権限を有効化
    // W    = 1  ← 書き込み権限を有効化
    // R    = 1  ← 読み込み権限を有効化
    pmp::write_pmpcfg0(0xf);
    // TOR で設定したので 0 ~ 0x3fffffffffffff までのアドレスに対して PMP を適用する。
    pmp::write_pmpaddr0(0x3fffffffffffff);

    timerinit();

    // Supervisor モードに遷移(MPP で指定したモード)して rust_main を実行する。
    mret();

    loop {}
}

static mut TIMER_SCRATCH: [usize; cpu::NCPU * 5] = [0; cpu::NCPU * 5];

fn timerinit() {
    let id = mhartid::read_mhartid();

    let interval = 1000000;
    crate::console::println!("mtimecmp: {:?}", clint::read_mtimecmp(id));
    clint::add_mtimecmp(id, interval);
    crate::console::println!("mtimecmp: {:?}", clint::read_mtimecmp(id));
}
