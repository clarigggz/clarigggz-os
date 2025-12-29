# SPDX-License-Identifier: MPL-2.0

.altmacro
.macro SAVE_GP n
    sd x\n, \n*8(sp)
.endm
.macro RESTORE_GP n
    ld x\n, \n*8(sp)
.endm

.section .text
.globl __alltraps
.align 2
__alltraps:
    csrrw sp, sscratch, sp
    # now sp points to kernel stack, sscratch points to user stack
    # allocate a TrapContext on kernel stack
    addi sp, sp, -34*8
    # save general-purpose registers
    sd x1, 1*8(sp)
    # skip sp(x2), we will save it later
    sd x3, 3*8(sp)
    .set n, 4
    .rept 28
        SAVE_GP %n
        .set n, n+1
    .endr
    # we can read from sscratch and save it
    csrr t0, sscratch
    sd t0, 2*8(sp)
    # save sstatus and sepc
    csrr t0, sstatus
    csrr t1, sepc
    sd t0, 32*8(sp)
    sd t1, 33*8(sp)
    # set input argument for trap_handler(context: &mut TrapContext)
    mv a0, sp
    call trap_handler

.globl __restore
__restore:
    # now sp points to TrapContext in kernel stack
    # restore sstatus and sepc
    ld t0, 32*8(sp)
    ld t1, 33*8(sp)
    csrw sstatus, t0
    csrw sepc, t1
    # restore general-purpose registers
    ld x1, 1*8(sp)
    ld x3, 3*8(sp)
    .set n, 4
    .rept 28
        RESTORE_GP %n
        .set n, n+1
    .endr
    # restore user stack
    ld t0, 2*8(sp)
    csrw sscratch, t0
    addi sp, sp, 34*8
    csrrw sp, sscratch, sp
    sret
