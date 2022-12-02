/* カーネル及びタイマ割り込みからのトラップ処理 */
.global kerneltrap
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

