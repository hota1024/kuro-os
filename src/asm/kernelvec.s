/* カーネル及びタイマ割り込みからのトラップ処理 */
.section .text
.global kernel_trap
.global kernelvec
.align 4
kernelvec:
		/* レジスタを退避させる空間の作成 */
		addi sp, sp, -256

		/* レジスタを退避 */
		sd ra, 0(sp)
		sd sp, 8(sp)
		sd gp, 16(sp)
		sd tp, 24(sp)
		sd t0, 32(sp)
		sd t1, 40(sp)
		sd t2, 48(sp)
		sd s0, 56(sp)
		sd s1, 64(sp)
		sd a0, 72(sp)
		sd a1, 80(sp)
		sd a2, 88(sp)
		sd a3, 96(sp)
		sd a4, 104(sp)
		sd a5, 112(sp)
		sd a6, 120(sp)
		sd a7, 128(sp)
		sd s2, 136(sp)
		sd s3, 144(sp)
		sd s4, 152(sp)
		sd s5, 160(sp)
		sd s6, 168(sp)
		sd s7, 176(sp)
		sd s8, 184(sp)
		sd s9, 192(sp)
		sd s10, 200(sp)
		sd s11, 208(sp)
		sd t3, 216(sp)
		sd t4, 224(sp)
		sd t5, 232(sp)
		sd t6, 240(sp)

		/* カーネルのトラップ処理を呼び出す */
		call kernel_trap

		/* レジスタを復元 */
		sd ra, 0(sp)
		sd sp, 8(sp)
		sd gp, 16(sp)
		sd tp, 24(sp)
		sd t0, 32(sp)
		sd t1, 40(sp)
		sd t2, 48(sp)
		sd s0, 56(sp)
		sd s1, 64(sp)
		sd a0, 72(sp)
		sd a1, 80(sp)
		sd a2, 88(sp)
		sd a3, 96(sp)
		sd a4, 104(sp)
		sd a5, 112(sp)
		sd a6, 120(sp)
		sd a7, 128(sp)
		sd s2, 136(sp)
		sd s3, 144(sp)
		sd s4, 152(sp)
		sd s5, 160(sp)
		sd s6, 168(sp)
		sd s7, 176(sp)
		sd s8, 184(sp)
		sd s9, 192(sp)
		sd s10, 200(sp)
		sd s11, 208(sp)
		sd t3, 216(sp)
		sd t4, 224(sp)
		sd t5, 232(sp)
		sd t6, 240(sp)
.section .text
.global timervec
.align 4
timervec:
		/* start.rs の write_mscratch で設定した mscratch を取得 */
		csrrw a0, mscratch, a0

		/*
		scratch[0, 4, 8, 16] : レジスタの退避空間
		*/
		sd a1, 0(a0)
		sd a2, 8(a0)
		sd a3, 16(a0)

		/*
		scratch[24]          : mtimecmp のアドレス
		scratch[32]          : interval (割り込みの間隔)

		mtimecmp に interval を足して次の割り込みをスケジュールする。
		*/
		ld  a1, 24(a0)
		ld  a2, 32(a0)
		ld  a3, 0(a1)
		add a3, a3, a2
		sd  a3, 0(a1)

		/*  */
		li   a1, 2
		csrw sip, a1

		ld    a3, 16(a0)
		ld    a2, 8(a0)
		ld    a1, 0(a0)
		csrrw a0, mscratch, a0

		mret