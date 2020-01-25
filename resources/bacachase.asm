	org 105
__START_PROGRAM:
	di
	push ix
	push iy
	exx
	push hl
	exx
	ld hl, 0
	add hl, sp
	ld (__CALL_BACK__), hl
	ei
#line 0
		ld sp, $dff0
#line 1
	jp __LABEL__ed01b
__LABEL__typeface:
__LABEL__patterns:
__LABEL__udg01:
#line 6
		defb $92,$FF,$FF,$FC,$F8,$F8,$B8,$A8
		defb $49,$FF,$FF,$3F,$1F,$1F,$1D,$15
		defb $A8,$B8,$F8,$F8,$FC,$FF,$FF,$92
#line 9
__LABEL__charmap01:
#line 13
		defb $00,$00,$00,$00,$00,$00,$00,$00
		defb $00,$1c,$1c,$1c,$1c,$00,$1c,$00
		defb $00,$76,$76,$00,$00,$00,$00,$00
		defb $00,$3b,$7f,$3b,$3b,$7f,$3b,$00
		defb $00,$0c,$7f,$70,$7f,$03,$7f,$0c
		defb $00,$73,$07,$0e,$1c,$38,$73,$00
		defb $00,$7e,$76,$3c,$77,$76,$7f,$00
		defb $00,$0c,$38,$00,$00,$00,$00,$00
		defb $00,$0e,$38,$38,$38,$38,$0e,$00
		defb $00,$38,$0e,$0e,$0e,$0e,$38,$00
		defb $00,$00,$73,$0c,$7f,$0c,$73,$00
		defb $00,$00,$1c,$1c,$7f,$1c,$1c,$00
		defb $00,$00,$00,$00,$00,$00,$1c,$7c
		defb $00,$00,$00,$00,$7c,$00,$00,$00
		defb $00,$00,$00,$00,$00,$00,$38,$00
		defb $00,$03,$07,$0e,$1c,$38,$70,$00
		defb $00,$7f,$73,$73,$73,$73,$7f,$00
		defb $00,$3e,$0e,$0e,$0e,$0e,$0e,$00
		defb $00,$7f,$73,$03,$7f,$70,$7f,$00
		defb $00,$7f,$73,$0f,$03,$73,$7f,$00
		defb $00,$73,$73,$73,$7f,$03,$03,$00
		defb $00,$7f,$70,$7f,$03,$73,$7f,$00
		defb $00,$7f,$70,$7f,$73,$73,$7f,$00
		defb $00,$7f,$03,$03,$3f,$38,$38,$00
		defb $00,$7f,$73,$0c,$73,$73,$7f,$00
		defb $00,$7f,$73,$73,$7f,$03,$7f,$00
		defb $00,$00,$00,$1c,$00,$00,$1c,$00
		defb $00,$00,$00,$1c,$00,$00,$1c,$7c
		defb $00,$00,$1c,$38,$70,$38,$1c,$00
		defb $00,$00,$00,$7e,$00,$7e,$00,$00
		defb $00,$00,$70,$38,$1c,$38,$70,$00
		defb $00,$7f,$73,$03,$0f,$00,$0c,$00
		defb $00,$7f,$73,$7f,$7f,$70,$7f,$00
		defb $00,$7f,$73,$73,$7f,$73,$73,$00
		defb $00,$7f,$73,$7c,$73,$73,$7f,$00
		defb $00,$7f,$73,$70,$70,$73,$7f,$00
		defb $00,$7c,$73,$73,$73,$73,$7c,$00
		defb $00,$7f,$70,$7f,$70,$70,$7f,$00
		defb $00,$7f,$70,$7f,$70,$70,$70,$00
		defb $00,$7f,$73,$70,$77,$73,$7f,$00
		defb $00,$73,$73,$7f,$73,$73,$73,$00
		defb $00,$3f,$0e,$0e,$0e,$0e,$3f,$00
		defb $00,$0f,$03,$03,$03,$73,$7f,$00
		defb $00,$73,$73,$7c,$73,$73,$73,$00
		defb $00,$70,$70,$70,$70,$70,$7f,$00
		defb $00,$73,$7f,$7f,$7f,$73,$73,$00
		defb $00,$73,$7f,$77,$73,$73,$73,$00
		defb $00,$7f,$73,$73,$73,$73,$7f,$00
		defb $00,$7f,$73,$73,$7f,$70,$70,$00
		defb $00,$7f,$73,$73,$77,$73,$7f,$00
		defb $00,$7f,$73,$73,$7c,$73,$73,$00
		defb $00,$7f,$70,$7f,$03,$03,$7f,$00
		defb $00,$ff,$1c,$1c,$1c,$1c,$1c,$00
		defb $00,$73,$73,$73,$73,$73,$7f,$00
		defb $00,$73,$73,$73,$73,$7f,$0c,$00
		defb $00,$73,$73,$7f,$7f,$7f,$73,$00
		defb $00,$73,$73,$0c,$73,$73,$73,$00
		defb $00,$73,$73,$7f,$0c,$0c,$0c,$00
		defb $00,$7f,$03,$7f,$70,$70,$7f,$00
		defb $00,$3e,$38,$38,$38,$38,$3e,$00
		defb $00,$70,$38,$1c,$0e,$07,$03,$00
		defb $00,$3e,$0e,$0e,$0e,$0e,$3e,$00
		defb $00,$1e,$77,$00,$00,$00,$00,$00
		defb %11111110
		defb %11000001
		defb %11011101
		defb %11001101
		defb %11001101
		defb %11011101
		defb %11000001
		defb %11111111
#line 84
__LABEL__ed01b:
	ld a, 1
	ld (_debug), a
	ld hl, 85
	ld (_seed), hl
	xor a
	ld (_d), a
__LABEL__start01:
	xor a
	push af
	xor a
	push af
	call _smsvdp
	ld a, 226
	push af
	ld a, 1
	push af
	call _smsvdp
	ld a, 6
	push af
	ld a, 2
	push af
	call _smsvdp
	ld a, 128
	push af
	ld a, 3
	push af
	call _smsvdp
	xor a
	push af
	ld a, 4
	push af
	call _smsvdp
	ld a, 54
	push af
	ld a, 5
	push af
	call _smsvdp
	ld a, 7
	push af
	ld a, 6
	push af
	call _smsvdp
	ld a, 49
	push af
	ld a, 7
	push af
	call _smsvdp
	ld hl, 16384
	push hl
	ld hl, 0
	push hl
	ld hl, 0
	push hl
	call _smsfilvrm
	ld hl, 2048
	push hl
	ld hl, __LABEL__charmap01
	push hl
	ld hl, 256
	push hl
	call _smsldirvm
	ld hl, 24
	push hl
	ld hl, __LABEL__udg01
	push hl
	ld hl, 0
	push hl
	call _smsldirvm
	ld hl, 24
	push hl
	ld hl, __LABEL__udg01
	push hl
	ld hl, 64
	push hl
	call _smsldirvm
	ld hl, 24
	push hl
	ld hl, __LABEL__udg01
	push hl
	ld hl, 128
	push hl
	call _smsldirvm
	ld hl, 24
	push hl
	ld hl, __LABEL__udg01
	push hl
	ld hl, 192
	push hl
	call _smsldirvm
	ld hl, 32
	push hl
	ld hl, 241
	push hl
	ld hl, 8192
	push hl
	call _smsfilvrm
	ld hl, 136
	push hl
	ld hl, 8192
	push hl
	call _smsvpoke
	ld hl, 3
	push hl
	ld hl, 210
	push hl
	ld hl, 8193
	push hl
	call _smsfilvrm
	ld hl, 28
	push hl
	ld hl, 66
	push hl
	ld hl, 8196
	push hl
	call _smsfilvrm
	ld hl, 12
	push hl
	ld hl, __LABEL__attr02
	push hl
	ld hl, 8192
	push hl
	call _smsldirvm
	call __LABEL__startup
__LABEL0:
	call __LABEL__title
	call __LABEL__gamestart
__LABEL__playloop:
	call __LABEL__stagestart
	call __LABEL__stageplay
	ld hl, 0
	ld de, (_stam)
	call __LTI16
	or a
	jp z, __LABEL3
	jp __LABEL__playloop
__LABEL3:
	jp __LABEL0
__LABEL1:
__LABEL__startup:
	ld hl, 0
	ld (_hiscore), hl
	ret
__LABEL__title:
#line 168
		halt
#line 169
	ld hl, 136
	push hl
	ld hl, 8192
	push hl
	call _smsvpoke
#line 172
		halt
#line 173
	ld hl, 3
	push hl
	ld hl, 210
	push hl
	ld hl, 8193
	push hl
	call _smsfilvrm
#line 176
		halt
#line 177
	ld hl, 28
	push hl
	ld hl, 66
	push hl
	ld hl, 8196
	push hl
	call _smsfilvrm
#line 180
		halt
#line 181
	ld hl, 192
	push hl
	ld hl, 0
	push hl
	ld hl, 6144
	push hl
	call _smsfilvrm
	ld hl, 576
	push hl
	ld hl, 32
	push hl
	ld hl, 6336
	push hl
	call _smsfilvrm
	xor a
	ld (_c), a
	jp __LABEL4
__LABEL7:
	ld hl, (_seed)
	ld (_seedo), hl
	ld hl, (_seed)
	call _smsrnd
	ld (_seed), hl
	ld (_seedo), hl
	ld hl, (_seed)
	call _smsrnd
	ld (_seed), hl
	ld a, 9
	push af
	ld de, 7
	ld hl, (_seed)
	call __BAND16
	ld de, 15
	add hl, de
	push hl
	ld de, 31
	ld hl, (_seed)
	call __BAND16
	push hl
	call _sg1000putchar
__LABEL8:
	ld a, (_c)
	inc a
	ld (_c), a
__LABEL4:
	ld a, 15
	ld hl, (_c - 1)
	cp h
	jp nc, __LABEL7
__LABEL6:
	ld hl, __LABEL__text01aend - __LABEL__text01a
	push hl
	ld hl, __LABEL__text01a
	push hl
	ld hl, 18
	push hl
	ld hl, 2
	push hl
	call _sg1000writetext
	ld hl, __LABEL__text01bend - __LABEL__text01b
	push hl
	ld hl, __LABEL__text01b
	push hl
	ld hl, 19
	push hl
	ld hl, 2
	push hl
	call _sg1000writetext
	ld hl, __LABEL__text01cend - __LABEL__text01c
	push hl
	ld hl, __LABEL__text01c
	push hl
	ld hl, 20
	push hl
	ld hl, 2
	push hl
	call _sg1000writetext
__LABEL__ttljstklp01:
	call _smsjoypad1
	push af
	call _smsjoypad2
	ld h, a
	pop af
	or h
	push af
	ld h, 63
	pop af
	and h
	ld (_boolbf), a
	sub 1
	jp nc, __LABEL10
	jp __LABEL__ttljstklp01
__LABEL10:
	ret
__LABEL__gamestart:
	ld hl, 0
	ld (_score), hl
	ld hl, 1
	ld (_stg), hl
	ld hl, 99
	ld (_stam), hl
	ret
__LABEL__stagestart:
	ld hl, 64
	push hl
	ld hl, 32
	push hl
	ld hl, 6144
	push hl
	call _smsfilvrm
	ld hl, 704
	push hl
	ld hl, 0
	push hl
	ld hl, 6208
	push hl
	call _smsfilvrm
	ld hl, 12
	push hl
	ld hl, __LABEL__attr02
	push hl
	ld hl, 8192
	push hl
	call _smsldirvm
	ld hl, (_stg)
	add hl, hl
	add hl, hl
	ld de, 4
	add hl, de
	ld (_enam), hl
	ld (_cnt), hl
	call __LABEL__displayscore
	call __LABEL__displaystamina
	ld a, 1
	ld (_d), a
	xor a
	ld (_kprs), a
	ld hl, (_seed)
	call _smsrnd
	ld (_seed), hl
	ld de, 31
	ld hl, (_seed)
	call __BAND16
	ld a, l
	ld (_x), a
	ld hl, (_seed)
	call _smsrnd
	ld (_seed), hl
	ld de, 22
	call __MODU16
	inc hl
	inc hl
	ld a, l
	ld (_y), a
	ld hl, 0
	ld (_i), hl
	jp __LABEL11
__LABEL14:
	ld hl, (_seed)
	call _smsrnd
	ld (_seed), hl
	ld a, l
	ld (_seedb), a
	ld h, 31
	ld a, (_seedb)
	and h
	push af
	ld hl, (_i)
	push hl
	call _setxin
	ld hl, (_seed)
	call _smsrnd
	ld (_seed), hl
	ld a, l
	ld (_seedb), a
	ld h, 22
	call __MODU8_FAST
	add a, 2
	push af
	ld hl, (_i)
	push hl
	call _setyin
	ld hl, (_i)
	push hl
	call _getxin
	call __FTOU32REG
	ld a, l
	ld (_xintp), a
	xor a
	push af
	ld hl, (_i)
	push hl
	call _setdin
	ld a, (_xintp)
	push af
	ld a, (_x)
	pop hl
	call __LTI8
	or a
	jp z, __LABEL17
	ld a, 1
	push af
	ld hl, (_i)
	push hl
	call _setdin
__LABEL17:
	xor a
	push af
	ld hl, (_i)
	push hl
	call _setstin
__LABEL15:
	ld hl, (_i)
	inc hl
	ld (_i), hl
__LABEL11:
	ld hl, 255
	ld de, (_i)
	call __LTI16
	or a
	jp z, __LABEL14
__LABEL13:
	ld hl, 0
	ld (_i), hl
	jp __LABEL18
__LABEL21:
	ld hl, (_i)
	push hl
	call _getxin
	call __FTOU32REG
	ld a, l
	ld (_xintp), a
	ld hl, (_i)
	push hl
	call _getyin
	call __FTOU32REG
	ld a, l
	ld (_yintp), a
	ld hl, (_i)
	push hl
	call _getdin
	call __FTOU32REG
	ld a, l
	ld (_dintp), a
	add a, 16
	push af
	ld a, (_yintp)
	ld l, a
	ld h, 0
	push hl
	ld a, (_xintp)
	ld l, a
	ld h, 0
	push hl
	call _sg1000putchar
__LABEL22:
	ld hl, (_i)
	inc hl
	ld (_i), hl
__LABEL18:
	ld hl, (_enam)
	dec hl
	ld de, (_i)
	call __LTI16
	or a
	jp z, __LABEL21
__LABEL20:
	ld a, (_d)
	add a, 8
	push af
	ld a, (_y)
	ld l, a
	add a, a
	sbc a, a
	ld h, a
	push hl
	ld a, (_x)
	ld l, a
	add a, a
	sbc a, a
	ld h, a
	push hl
	call _sg1000putchar
	ret
__LABEL__stageplay:
__LABEL__lp01:
__LABEL__checkjoystick:
	call _smsjoypad1
	push af
	call _smsjoypad2
	ld h, a
	pop af
	or h
	push af
	ld h, 63
	pop af
	and h
	ld (_jacum), a
	ld h, 1
	ld a, (_jacum)
	and h
	push af
	ld a, (_kprs)
	sub 1
	sbc a, a
	ld h, a
	pop af
	call __AND8
	or a
	jp z, __LABEL24
	xor a
	push af
	ld a, (_y)
	ld l, a
	add a, a
	sbc a, a
	ld h, a
	push hl
	ld a, (_x)
	ld l, a
	add a, a
	sbc a, a
	ld h, a
	push hl
	call _sg1000putchar
	ld a, (_y)
	dec a
	ld (_y), a
	ld a, 1
	ld (_kprs), a
	ld h, 3
	ld a, (_y)
	call __LTI8
	or a
	jp z, __LABEL26
	ld a, 2
	ld (_y), a
__LABEL26:
	ld a, (_d)
	add a, 8
	push af
	ld a, (_y)
	ld l, a
	add a, a
	sbc a, a
	ld h, a
	push hl
	ld a, (_x)
	ld l, a
	add a, a
	sbc a, a
	ld h, a
	push hl
	call _sg1000putchar
__LABEL24:
	ld h, 2
	ld a, (_jacum)
	and h
	push af
	ld a, (_kprs)
	sub 1
	sbc a, a
	ld h, a
	pop af
	call __AND8
	or a
	jp z, __LABEL28
	xor a
	push af
	ld a, (_y)
	ld l, a
	add a, a
	sbc a, a
	ld h, a
	push hl
	ld a, (_x)
	ld l, a
	add a, a
	sbc a, a
	ld h, a
	push hl
	call _sg1000putchar
	ld a, (_y)
	inc a
	ld (_y), a
	ld a, 1
	ld (_kprs), a
	ld a, 22
	ld hl, (_y - 1)
	call __LTI8
	or a
	jp z, __LABEL30
	ld a, 23
	ld (_y), a
__LABEL30:
	ld a, (_d)
	add a, 8
	push af
	ld a, (_y)
	ld l, a
	add a, a
	sbc a, a
	ld h, a
	push hl
	ld a, (_x)
	ld l, a
	add a, a
	sbc a, a
	ld h, a
	push hl
	call _sg1000putchar
__LABEL28:
	ld h, 4
	ld a, (_jacum)
	and h
	push af
	ld a, (_kprs)
	sub 1
	sbc a, a
	ld h, a
	pop af
	call __AND8
	or a
	jp z, __LABEL32
	xor a
	push af
	ld a, (_y)
	ld l, a
	add a, a
	sbc a, a
	ld h, a
	push hl
	ld a, (_x)
	ld l, a
	add a, a
	sbc a, a
	ld h, a
	push hl
	call _sg1000putchar
	ld a, (_x)
	dec a
	ld (_x), a
	ld a, 1
	ld (_kprs), a
	ld a, 1
	ld (_d), a
	ld h, 1
	ld a, (_x)
	call __LTI8
	or a
	jp z, __LABEL34
	xor a
	ld (_x), a
__LABEL34:
	ld a, (_d)
	add a, 8
	push af
	ld a, (_y)
	ld l, a
	add a, a
	sbc a, a
	ld h, a
	push hl
	ld a, (_x)
	ld l, a
	add a, a
	sbc a, a
	ld h, a
	push hl
	call _sg1000putchar
__LABEL32:
	ld h, 8
	ld a, (_jacum)
	and h
	push af
	ld a, (_kprs)
	sub 1
	sbc a, a
	ld h, a
	pop af
	call __AND8
	or a
	jp z, __LABEL36
	xor a
	push af
	ld a, (_y)
	ld l, a
	add a, a
	sbc a, a
	ld h, a
	push hl
	ld a, (_x)
	ld l, a
	add a, a
	sbc a, a
	ld h, a
	push hl
	call _sg1000putchar
	ld a, (_x)
	inc a
	ld (_x), a
	ld a, 1
	ld (_kprs), a
	xor a
	ld (_d), a
	ld a, 30
	ld hl, (_x - 1)
	call __LTI8
	or a
	jp z, __LABEL38
	ld a, 31
	ld (_x), a
__LABEL38:
	ld a, (_d)
	add a, 8
	push af
	ld a, (_y)
	ld l, a
	add a, a
	sbc a, a
	ld h, a
	push hl
	ld a, (_x)
	ld l, a
	add a, a
	sbc a, a
	ld h, a
	push hl
	call _sg1000putchar
__LABEL36:
	ld h, 48
	ld a, (_jacum)
	and h
	push af
	ld a, (_kprs)
	sub 1
	sbc a, a
	ld h, a
	pop af
	call __AND8
	or a
	jp z, __LABEL40
	xor a
	push af
	ld a, (_y)
	ld l, a
	add a, a
	sbc a, a
	ld h, a
	push hl
	ld a, (_x)
	ld l, a
	add a, a
	sbc a, a
	ld h, a
	push hl
	call _sg1000putchar
	ld hl, (_seed)
	call _smsrnd
	ld (_seed), hl
	ld de, 31
	ld hl, (_seed)
	call __BAND16
	ld a, l
	ld (_x), a
	ld hl, (_seed)
	call _smsrnd
	ld (_seed), hl
	ld de, 22
	call __MODU16
	inc hl
	inc hl
	ld a, l
	ld (_y), a
	ld a, 1
	ld (_kprs), a
	ld a, (_d)
	add a, 8
	push af
	ld a, (_y)
	ld l, a
	add a, a
	sbc a, a
	ld h, a
	push hl
	ld a, (_x)
	ld l, a
	add a, a
	sbc a, a
	ld h, a
	push hl
	call _sg1000putchar
__LABEL40:
	ld a, (_jacum)
	sub 1
	sbc a, a
	push af
	ld a, (_kprs)
	dec a
	sub 1
	sbc a, a
	ld h, a
	pop af
	call __AND8
	or a
	jp z, __LABEL42
	xor a
	ld (_kprs), a
	ld hl, 0
	ld (_i), hl
	jp __LABEL43
__LABEL46:
	ld hl, (_i)
	push hl
	call _getstin
	call __FTOU32REG
	ld a, l
	ld (_stintp), a
	sub 1
	jp nc, __LABEL49
	ld hl, (_i)
	push hl
	call _getxin
	call __FTOU32REG
	ld a, l
	ld (_xintp), a
	ld hl, (_i)
	push hl
	call _getyin
	call __FTOU32REG
	ld a, l
	ld (_yintp), a
	xor a
	push af
	ld a, (_yintp)
	ld l, a
	ld h, 0
	push hl
	ld a, (_xintp)
	ld l, a
	ld h, 0
	push hl
	call _sg1000putchar
	ld a, (_yintp)
	push af
	ld a, (_y)
	pop hl
	call __LTI8
	or a
	jp z, __LABEL51
	ld a, (_yintp)
	dec a
	push af
	ld hl, (_i)
	push hl
	call _setyin
__LABEL51:
	ld a, (_yintp)
	push af
	ld hl, (_y - 1)
	pop af
	call __LTI8
	or a
	jp z, __LABEL53
	ld a, (_yintp)
	inc a
	push af
	ld hl, (_i)
	push hl
	call _setyin
__LABEL53:
	ld a, (_xintp)
	push af
	ld a, (_x)
	pop hl
	call __LTI8
	or a
	jp z, __LABEL55
	ld a, (_xintp)
	dec a
	push af
	ld hl, (_i)
	push hl
	call _setxin
__LABEL55:
	ld a, (_xintp)
	push af
	ld hl, (_x - 1)
	pop af
	call __LTI8
	or a
	jp z, __LABEL57
	ld a, (_xintp)
	inc a
	push af
	ld hl, (_i)
	push hl
	call _setxin
__LABEL57:
__LABEL49:
	ld hl, 0
	ld (_j), hl
	jp __LABEL58
__LABEL61:
	ld hl, (_i)
	push hl
	call _getxin
	call __FTOU32REG
	ld a, l
	ld (_xintp), a
	ld hl, (_i)
	push hl
	call _getyin
	call __FTOU32REG
	ld a, l
	ld (_yintp), a
	ld hl, (_j)
	push hl
	call _getxin
	call __FTOU32REG
	ld a, l
	ld (_xintq), a
	ld hl, (_j)
	push hl
	call _getyin
	call __FTOU32REG
	ld a, l
	ld (_yintq), a
	ld hl, (_yintq - 1)
	ld a, (_yintp)
	sub h
	sub 1
	sbc a, a
	push af
	ld hl, (_xintq - 1)
	ld a, (_xintp)
	sub h
	sub 1
	sbc a, a
	ld h, a
	pop af
	call __AND8
	or a
	jp z, __LABEL64
	ld a, 1
	push af
	ld hl, (_i)
	push hl
	call _setstin
	ld a, 1
	push af
	ld hl, (_j)
	push hl
	call _setstin
	ld hl, (_score)
	ld de, 10
	add hl, de
	ld (_score), hl
__LABEL64:
__LABEL62:
	ld hl, (_j)
	inc hl
	ld (_j), hl
__LABEL58:
	ld hl, (_i)
	dec hl
	ld de, (_j)
	call __LTI16
	or a
	jp z, __LABEL61
__LABEL60:
	ld de, (_score)
	ld hl, (_hiscore)
	call __LTI16
	or a
	jp z, __LABEL66
	ld hl, (_score)
	ld (_hiscore), hl
__LABEL66:
	call __LABEL__displayscore
	ld hl, (_i)
	push hl
	call _getxin
	call __FTOU32REG
	ld a, l
	ld (_xintp), a
	push af
	ld a, (_x)
	pop hl
	call __LTI8
	or a
	jp z, __LABEL68
	ld a, 1
	push af
	ld hl, (_i)
	push hl
	call _setdin
__LABEL68:
	ld a, (_xintp)
	push af
	ld hl, (_x - 1)
	pop af
	call __LTI8
	or a
	jp z, __LABEL70
	xor a
	push af
	ld hl, (_i)
	push hl
	call _setdin
__LABEL70:
	ld hl, (_i)
	push hl
	call _getxin
	call __FTOU32REG
	ld a, l
	ld (_xintp), a
	ld hl, (_i)
	push hl
	call _getyin
	call __FTOU32REG
	ld a, l
	ld (_yintp), a
	ld hl, (_i)
	push hl
	call _getdin
	call __FTOU32REG
	ld a, l
	ld (_dintp), a
	add a, 16
	push af
	ld a, (_yintp)
	ld l, a
	ld h, 0
	push hl
	ld a, (_xintp)
	ld l, a
	ld h, 0
	push hl
	call _sg1000putchar
	ld hl, (_i)
	push hl
	call _getstin
	call __FTOU32REG
	ld a, l
	ld (_stintp), a
	dec a
	sub 1
	jp nc, __LABEL72
	ld hl, (_i)
	push hl
	call _getxin
	call __FTOU32REG
	ld a, l
	ld (_xintp), a
	ld hl, (_i)
	push hl
	call _getyin
	call __FTOU32REG
	ld a, l
	ld (_yintp), a
	ld a, 26
	push af
	ld a, (_yintp)
	ld l, a
	ld h, 0
	push hl
	ld a, (_xintp)
	ld l, a
	ld h, 0
	push hl
	call _sg1000putchar
__LABEL72:
__LABEL47:
	ld hl, (_i)
	inc hl
	ld (_i), hl
__LABEL43:
	ld hl, (_enam)
	dec hl
	ld de, (_i)
	call __LTI16
	or a
	jp z, __LABEL46
__LABEL45:
	ld hl, 0
	ld (_cnt), hl
	ld hl, 0
	ld (_i), hl
	jp __LABEL73
__LABEL76:
	ld hl, (_i)
	push hl
	call _getstin
	call __FTOU32REG
	ld a, l
	ld (_stintp), a
	sub 1
	jp nc, __LABEL79
	ld hl, (_cnt)
	inc hl
	ld (_cnt), hl
__LABEL79:
	ld hl, (_i)
	push hl
	call _getxin
	call __FTOU32REG
	ld a, l
	ld (_xintp), a
	ld hl, (_i)
	push hl
	call _getyin
	call __FTOU32REG
	ld a, l
	ld (_yintp), a
	ld a, (_xintp)
	ld h, a
	ld a, (_x)
	sub h
	sub 1
	sbc a, a
	push af
	ld a, (_yintp)
	ld h, a
	ld a, (_y)
	sub h
	sub 1
	sbc a, a
	ld h, a
	pop af
	call __AND8
	or a
	jp z, __LABEL81
	ld hl, (_stam)
	dec hl
	ld (_stam), hl
__LABEL81:
__LABEL77:
	ld hl, (_i)
	inc hl
	ld (_i), hl
__LABEL73:
	ld hl, (_enam)
	dec hl
	ld de, (_i)
	call __LTI16
	or a
	jp z, __LABEL76
__LABEL75:
	ld a, (_d)
	add a, 8
	push af
	ld a, (_y)
	ld l, a
	add a, a
	sbc a, a
	ld h, a
	push hl
	ld a, (_x)
	ld l, a
	add a, a
	sbc a, a
	ld h, a
	push hl
	call _sg1000putchar
	call __LABEL__displaystamina
__LABEL42:
	ld hl, 0
	ld de, (_cnt)
	call __LTI16
	or a
	jp z, __LABEL83
	jp __LABEL__lp01
__LABEL83:
	ld hl, (_stg)
	inc hl
	ld (_stg), hl
	call __LABEL__displayscore
	call __LABEL__displaystamina
	ret
__LABEL__displayscore:
	ld hl, __LABEL__text01dend - __LABEL__text01d - 2
	push hl
	ld hl, __LABEL__text01d + 2
	push hl
	ld hl, 0
	push hl
	ld hl, 1
	push hl
	call _sg1000writetext
	ld hl, (_score)
	push hl
	ld hl, 7
	push hl
	call _zprintdecimal5
	ld hl, __LABEL__text01dend - __LABEL__text01d
	push hl
	ld hl, __LABEL__text01d
	push hl
	ld hl, 0
	push hl
	ld hl, 13
	push hl
	call _sg1000writetext
	ld hl, (_hiscore)
	push hl
	ld hl, 21
	push hl
	call _zprintdecimal5
	ret
__LABEL__displaystamina:
	ld hl, __LABEL__text01eend - __LABEL__text01e
	push hl
	ld hl, __LABEL__text01e
	push hl
	ld hl, 1
	push hl
	ld hl, 1
	push hl
	call _sg1000writetext
	ld hl, (_cnt)
	push hl
	ld hl, 41
	push hl
	call _zprintdecimal2
	ld hl, __LABEL__text01fend - __LABEL__text01f
	push hl
	ld hl, __LABEL__text01f
	push hl
	ld hl, 1
	push hl
	ld hl, 12
	push hl
	call _sg1000writetext
	ld hl, (_stam)
	push hl
	ld hl, 52
	push hl
	call _zprintdecimal2
	ld hl, __LABEL__text01gend - __LABEL__text01g
	push hl
	ld hl, __LABEL__text01g
	push hl
	ld hl, 1
	push hl
	ld hl, 23
	push hl
	call _sg1000writetext
	ld hl, (_stg)
	push hl
	ld hl, 61
	push hl
	call _zprintdecimal2
	ret
__LABEL84:
	jp __LABEL84
__LABEL85:
__LABEL__text01:
__LABEL__text01a:
#line 396
		defb "BACACHASE"
#line 397
__LABEL__text01aend:
__LABEL__text01b:
#line 401
		defb 95
		defb " PAULO SILVA, '16, '11"
#line 403
__LABEL__text01bend:
__LABEL__text01c:
#line 407
		defb "PUSH ANY KEY"
#line 408
__LABEL__text01cend:
__LABEL__text01d:
#line 412
	defb "HISCORE:"
#line 413
__LABEL__text01dend:
__LABEL__text01e:
#line 417
	defb "ZOMBIES:"
#line 418
__LABEL__text01eend:
__LABEL__text01f:
#line 422
	defb "STAMINA:"
#line 423
__LABEL__text01fend:
__LABEL__text01g:
#line 427
	defb "STAGE:"
#line 428
__LABEL__text01gend:
__LABEL__attr01:
#line 433
		defb $88,$D2,$D2,$D2
		defb $42,$42,$42,$42
		defb $42,$42,$42,$42
#line 436
__LABEL__attr02:
#line 440
		defb $22,$42,$A2,$72
		defb $71,$71,$F1,$F1
		defb $71,$71,$71,$71
#line 443
	ld hl, 0
	ld b, h
	ld c, l
__END_PROGRAM:
	di
	ld hl, (__CALL_BACK__)
	ld sp, hl
	exx
	pop hl
	exx
	pop iy
	pop ix
	ei
	ret
__CALL_BACK__:
	DEFW 0
_smsvdp:
	push ix
	ld ix, 0
	add ix, sp
#line 1
		ld a, (ix+7)
		out ($bf),a
		ld a, (ix+5)
		or $80
		out ($bf),a
#line 6
_smsvdp__leave:
	ld sp, ix
	pop ix
	exx
	pop hl
	pop bc
	ex (sp), hl
	exx
	ret
_smsfilvrm:
	push ix
	ld ix, 0
	add ix, sp
#line 1
		ld l, (ix+4)
		ld a,l
		out ($bf),a
		ld h, (ix+5)
		ld a,h
		or $40
		out ($bf),a
		ld b, (ix+9)
		ld c, (ix+8)
smsfilvrmloop:
		ld a, (ix+6)
		out ($be),a
		dec bc
		ld a,b
		or c
		jp nz,smsfilvrmloop
#line 17
_smsfilvrm__leave:
	ld sp, ix
	pop ix
	exx
	pop hl
	pop bc
	pop bc
	ex (sp), hl
	exx
	ret
_smsldirvm:
	push ix
	ld ix, 0
	add ix, sp
#line 1
		ld d, (ix+5)
		ld e, (ix+4)
		ld h, (ix+7)
		ld l, (ix+6)
		ld b, (ix+9)
		ld c, (ix+8)
		ld a,e
		out ($bf),a
		ld a,d
		or $40
		out ($bf),a
smsldirvmloop:
		ld a, (hl)
		out ($be),a
		inc hl
		dec bc
		ld a,b
		or c
		jp nz,smsldirvmloop
#line 20
_smsldirvm__leave:
	ld sp, ix
	pop ix
	exx
	pop hl
	pop bc
	pop bc
	ex (sp), hl
	exx
	ret
_smsvpoke:
	push ix
	ld ix, 0
	add ix, sp
#line 1
		ld l, (ix+4)
		ld a,l
		out ($bf),a
		ld h, (ix+5)
		ld a,h
		or $40
		out ($bf),a
		ld a, (ix+6)
		out ($be),a
#line 10
_smsvpoke__leave:
	ld sp, ix
	pop ix
	exx
	pop hl
	pop bc
	ex (sp), hl
	exx
	ret
_smsrnd:
#line 2
		ld  d, h
		ld  e, l
		ld  a, d
		ld  h, e
		ld  l, 253
		or  a
		sbc  hl, de
		sbc  a, 0
		sbc  hl, de
		ld  d, 0
		sbc  a, d
		ld  e, a
		sbc  hl, de
		jr  nc, smsrndloop
		inc  hl
smsrndloop:
		ret
#line 19
_smsrnd__leave:
	ret
_smsjoypad1:
#line 1
		in  a, ($dc)
		cpl
#line 3
_smsjoypad1__leave:
	ret
_smsjoypad2:
#line 8
		in  a, ($dc)
		cpl
		rla
		rla
		rla
		and  $03
		ld  l, a
		in  a, ($dd)
		cpl
		add  a, a
		add  a, a
		or  l
#line 20
_smsjoypad2__leave:
	ret
_smsdelay:
	push ix
	ld ix, 0
	add ix, sp
#line 1
		ld b, (ix+5)
		ld c, (ix+4)
smsdelayloop:
		dec bc
		ld a,b
		or c
		jp nz,smsdelayloop
#line 8
_smsdelay__leave:
	ld sp, ix
	pop ix
	exx
	pop hl
	ex (sp), hl
	exx
	ret
_cvsoundio:
	push ix
	ld ix, 0
	add ix, sp
	ld a, (ix+5)
	ld bc, 255
	out (c), a
_cvsoundio__leave:
	ld sp, ix
	pop ix
	exx
	pop hl
	ex (sp), hl
	exx
	ret
_sg1000putchar:
	push ix
	ld ix, 0
	add ix, sp
	ld a, (ix+9)
	ld l, a
	ld h, 0
	push hl
	ld l, (ix+4)
	ld h, (ix+5)
	ld de, 6144
	add hl, de
	push hl
	ld l, (ix+6)
	ld h, (ix+7)
	ld de, 32
	call __MUL16_FAST
	ex de, hl
	pop hl
	add hl, de
	push hl
	call _smsvpoke
_sg1000putchar__leave:
	ld sp, ix
	pop ix
	exx
	pop hl
	pop bc
	pop bc
	ex (sp), hl
	exx
	ret
_sg1000writetext:
	push ix
	ld ix, 0
	add ix, sp
	ld hl, 0
	ld (_sg1000writetext_tq2), hl
	jp __LABEL86
__LABEL89:
	ld l, (ix+8)
	ld h, (ix+9)
	ex de, hl
	ld hl, (_sg1000writetext_tq2)
	add hl, de
	ld b, h
	ld c, l
	ld a, (bc)
	push af
	ld l, (ix+6)
	ld h, (ix+7)
	push hl
	ld l, (ix+4)
	ld h, (ix+5)
	ex de, hl
	ld hl, (_sg1000writetext_tq2)
	add hl, de
	push hl
	call _sg1000putchar
__LABEL90:
	ld hl, (_sg1000writetext_tq2)
	inc hl
	ld (_sg1000writetext_tq2), hl
__LABEL86:
	ld l, (ix+10)
	ld h, (ix+11)
	dec hl
	ld de, (_sg1000writetext_tq2)
	or a
	sbc hl, de
	jp nc, __LABEL89
__LABEL88:
_sg1000writetext__leave:
	ld sp, ix
	pop ix
	exx
	pop hl
	pop bc
	pop bc
	pop bc
	ex (sp), hl
	exx
	ret
_zprintdecimal5:
	push ix
	ld ix, 0
	add ix, sp
	ld l, (ix+6)
	ld h, (ix+7)
	ld de, 10000
	call __DIVU16
	ld de, 0
	push de
	push hl
	ld de, 0
	ld hl, 10
	call __SWAP32
	call __MODI32
	ld bc, 0
	push bc
	ld bc, 48
	push bc
	pop bc
	add hl, bc
	ex de, hl
	pop bc
	adc hl, bc
	ex de, hl
	push hl
	ld l, (ix+4)
	ld h, (ix+5)
	ld de, 6144
	add hl, de
	push hl
	call _smsvpoke
	ld l, (ix+6)
	ld h, (ix+7)
	ld de, 1000
	call __DIVU16
	ld de, 0
	push de
	push hl
	ld de, 0
	ld hl, 10
	call __SWAP32
	call __MODI32
	ld bc, 0
	push bc
	ld bc, 48
	push bc
	pop bc
	add hl, bc
	ex de, hl
	pop bc
	adc hl, bc
	ex de, hl
	push hl
	ld l, (ix+4)
	ld h, (ix+5)
	ld de, 6144
	add hl, de
	inc hl
	push hl
	call _smsvpoke
	ld l, (ix+6)
	ld h, (ix+7)
	ld de, 100
	call __DIVU16
	ld de, 0
	push de
	push hl
	ld de, 0
	ld hl, 10
	call __SWAP32
	call __MODI32
	ld bc, 0
	push bc
	ld bc, 48
	push bc
	pop bc
	add hl, bc
	ex de, hl
	pop bc
	adc hl, bc
	ex de, hl
	push hl
	ld l, (ix+4)
	ld h, (ix+5)
	ld de, 6144
	add hl, de
	inc hl
	inc hl
	push hl
	call _smsvpoke
	ld l, (ix+6)
	ld h, (ix+7)
	ld de, 10
	call __DIVU16
	ld de, 0
	push de
	push hl
	ld de, 0
	ld hl, 10
	call __SWAP32
	call __MODI32
	ld bc, 0
	push bc
	ld bc, 48
	push bc
	pop bc
	add hl, bc
	ex de, hl
	pop bc
	adc hl, bc
	ex de, hl
	push hl
	ld l, (ix+4)
	ld h, (ix+5)
	ld de, 6144
	add hl, de
	inc hl
	inc hl
	inc hl
	push hl
	call _smsvpoke
	ld l, (ix+6)
	ld h, (ix+7)
	ld de, 10
	call __MODU16
	ld de, 48
	add hl, de
	push hl
	ld l, (ix+4)
	ld h, (ix+5)
	ld de, 6144
	add hl, de
	ld de, 4
	add hl, de
	push hl
	call _smsvpoke
_zprintdecimal5__leave:
	ld sp, ix
	pop ix
	exx
	pop hl
	pop bc
	ex (sp), hl
	exx
	ret
_zprintdecimal4:
	push ix
	ld ix, 0
	add ix, sp
	ld l, (ix+6)
	ld h, (ix+7)
	ld de, 1000
	call __DIVU16
	ld de, 0
	push de
	push hl
	ld de, 0
	ld hl, 10
	call __SWAP32
	call __MODI32
	ld bc, 0
	push bc
	ld bc, 48
	push bc
	pop bc
	add hl, bc
	ex de, hl
	pop bc
	adc hl, bc
	ex de, hl
	push hl
	ld l, (ix+4)
	ld h, (ix+5)
	ld de, 6144
	add hl, de
	push hl
	call _smsvpoke
	ld l, (ix+6)
	ld h, (ix+7)
	ld de, 100
	call __DIVU16
	ld de, 0
	push de
	push hl
	ld de, 0
	ld hl, 10
	call __SWAP32
	call __MODI32
	ld bc, 0
	push bc
	ld bc, 48
	push bc
	pop bc
	add hl, bc
	ex de, hl
	pop bc
	adc hl, bc
	ex de, hl
	push hl
	ld l, (ix+4)
	ld h, (ix+5)
	ld de, 6144
	add hl, de
	inc hl
	push hl
	call _smsvpoke
	ld l, (ix+6)
	ld h, (ix+7)
	ld de, 10
	call __DIVU16
	ld de, 0
	push de
	push hl
	ld de, 0
	ld hl, 10
	call __SWAP32
	call __MODI32
	ld bc, 0
	push bc
	ld bc, 48
	push bc
	pop bc
	add hl, bc
	ex de, hl
	pop bc
	adc hl, bc
	ex de, hl
	push hl
	ld l, (ix+4)
	ld h, (ix+5)
	ld de, 6144
	add hl, de
	inc hl
	inc hl
	push hl
	call _smsvpoke
	ld l, (ix+6)
	ld h, (ix+7)
	ld de, 10
	call __MODU16
	ld de, 48
	add hl, de
	push hl
	ld l, (ix+4)
	ld h, (ix+5)
	ld de, 6144
	add hl, de
	inc hl
	inc hl
	inc hl
	push hl
	call _smsvpoke
_zprintdecimal4__leave:
	ld sp, ix
	pop ix
	exx
	pop hl
	pop bc
	ex (sp), hl
	exx
	ret
_zprintdecimal2:
	push ix
	ld ix, 0
	add ix, sp
	ld l, (ix+6)
	ld h, (ix+7)
	ld de, 10
	call __DIVU16
	ld de, 0
	push de
	push hl
	ld de, 0
	ld hl, 10
	call __SWAP32
	call __MODI32
	ld bc, 0
	push bc
	ld bc, 48
	push bc
	pop bc
	add hl, bc
	ex de, hl
	pop bc
	adc hl, bc
	ex de, hl
	push hl
	ld l, (ix+4)
	ld h, (ix+5)
	ld de, 6144
	add hl, de
	push hl
	call _smsvpoke
	ld l, (ix+6)
	ld h, (ix+7)
	ld de, 10
	call __MODU16
	ld de, 48
	add hl, de
	push hl
	ld l, (ix+4)
	ld h, (ix+5)
	ld de, 6144
	add hl, de
	inc hl
	push hl
	call _smsvpoke
_zprintdecimal2__leave:
	ld sp, ix
	pop ix
	exx
	pop hl
	pop bc
	ex (sp), hl
	exx
	ret
_getxin:
	push ix
	ld ix, 0
	add ix, sp
	ld l, (ix+4)
	ld h, (ix+5)
	ld de, 49408
	add hl, de
	ld b, h
	ld c, l
	ld a, (bc)
	push af
	ld h, 127
	pop af
	and h
	call __U8TOFREG
_getxin__leave:
	ld sp, ix
	pop ix
	exx
	pop hl
	ex (sp), hl
	exx
	ret
_setxin:
	push ix
	ld ix, 0
	add ix, sp
	ld l, (ix+4)
	ld h, (ix+5)
	ld de, 49408
	add hl, de
	push hl
	ld l, (ix+4)
	ld h, (ix+5)
	ld de, 49408
	add hl, de
	ld b, h
	ld c, l
	ld a, (bc)
	push af
	ld h, 128
	pop af
	and h
	push af
	ld a, (ix+7)
	push af
	ld h, 127
	pop af
	and h
	ld h, a
	pop af
	or h
	pop hl
	ld (hl), a
_setxin__leave:
	ld sp, ix
	pop ix
	exx
	pop hl
	pop bc
	ex (sp), hl
	exx
	ret
_getyin:
	push ix
	ld ix, 0
	add ix, sp
	ld l, (ix+4)
	ld h, (ix+5)
	ld de, 51200
	add hl, de
	ld b, h
	ld c, l
	ld a, (bc)
	push af
	ld h, 127
	pop af
	and h
	call __U8TOFREG
_getyin__leave:
	ld sp, ix
	pop ix
	exx
	pop hl
	ex (sp), hl
	exx
	ret
_setyin:
	push ix
	ld ix, 0
	add ix, sp
	ld l, (ix+4)
	ld h, (ix+5)
	ld de, 51200
	add hl, de
	push hl
	ld l, (ix+4)
	ld h, (ix+5)
	ld de, 51200
	add hl, de
	ld b, h
	ld c, l
	ld a, (bc)
	push af
	ld h, 128
	pop af
	and h
	push af
	ld a, (ix+7)
	push af
	ld h, 127
	pop af
	and h
	ld h, a
	pop af
	or h
	pop hl
	ld (hl), a
_setyin__leave:
	ld sp, ix
	pop ix
	exx
	pop hl
	pop bc
	ex (sp), hl
	exx
	ret
_getstin:
	push ix
	ld ix, 0
	add ix, sp
	ld l, (ix+4)
	ld h, (ix+5)
	ld de, 49408
	add hl, de
	ld b, h
	ld c, l
	ld a, (bc)
	push af
	ld h, 128
	pop af
	and h
	ld h, 128
	call __DIVU8_FAST
	call __U8TOFREG
_getstin__leave:
	ld sp, ix
	pop ix
	exx
	pop hl
	ex (sp), hl
	exx
	ret
_setstin:
	push ix
	ld ix, 0
	add ix, sp
	ld l, (ix+4)
	ld h, (ix+5)
	ld de, 49408
	add hl, de
	push hl
	ld l, (ix+4)
	ld h, (ix+5)
	ld de, 49408
	add hl, de
	ld b, h
	ld c, l
	ld a, (bc)
	push af
	ld h, 127
	pop af
	and h
	push af
	ld a, (ix+7)
	push af
	ld h, 1
	pop af
	and h
	ld h, 128
	call __MUL8_FAST
	ld h, a
	pop af
	or h
	pop hl
	ld (hl), a
_setstin__leave:
	ld sp, ix
	pop ix
	exx
	pop hl
	pop bc
	ex (sp), hl
	exx
	ret
_getdin:
	push ix
	ld ix, 0
	add ix, sp
	ld l, (ix+4)
	ld h, (ix+5)
	ld de, 51200
	add hl, de
	ld b, h
	ld c, l
	ld a, (bc)
	push af
	ld h, 128
	pop af
	and h
	ld h, 128
	call __DIVU8_FAST
	call __U8TOFREG
_getdin__leave:
	ld sp, ix
	pop ix
	exx
	pop hl
	ex (sp), hl
	exx
	ret
_setdin:
	push ix
	ld ix, 0
	add ix, sp
	ld l, (ix+4)
	ld h, (ix+5)
	ld de, 51200
	add hl, de
	push hl
	ld l, (ix+4)
	ld h, (ix+5)
	ld de, 51200
	add hl, de
	ld b, h
	ld c, l
	ld a, (bc)
	push af
	ld h, 127
	pop af
	and h
	push af
	ld a, (ix+7)
	push af
	ld h, 1
	pop af
	and h
	ld h, 128
	call __MUL8_FAST
	ld h, a
	pop af
	or h
	pop hl
	ld (hl), a
_setdin__leave:
	ld sp, ix
	pop ix
	exx
	pop hl
	pop bc
	ex (sp), hl
	exx
	ret
#line 1 "mul16.asm"
__MUL16:	; Mutiplies HL with the last value stored into de stack
				; Works for both signed and unsigned
	
			PROC
	
			LOCAL __MUL16LOOP
	        LOCAL __MUL16NOADD
			
			ex de, hl
			pop hl		; Return address
			ex (sp), hl ; CALLEE caller convention
	
;;__MUL16_FAST:	; __FASTCALL ENTRY: HL = 1st operand, DE = 2nd Operand
	;;		ld c, h
	;;		ld a, l	 ; C,A => 1st Operand
	;;
	;;		ld hl, 0 ; Accumulator
	;;		ld b, 16
	;;
;;__MUL16LOOP:
	;;		sra c	; C,A >> 1  (Arithmetic)
	;;		rra
	;;
	;;		jr nc, __MUL16NOADD
	;;		add hl, de
	;;
;;__MUL16NOADD:
	;;		sla e
	;;		rl d
	;;			
	;;		djnz __MUL16LOOP
	
__MUL16_FAST:
	        ld b, 16
	        ld a, d
	        ld c, e
	        ex de, hl
	        ld hl, 0
	
__MUL16LOOP:
	        add hl, hl  ; hl << 1
	        sla c
	        rla         ; a,c << 1
	        jp nc, __MUL16NOADD
	        add hl, de
	
__MUL16NOADD:
	        djnz __MUL16LOOP
	
			ret	; Result in hl (16 lower bits)
	
			ENDP
	
#line 2242 "bacachase.bas"
#line 1 "mul8.asm"
__MUL8:		; Performs 8bit x 8bit multiplication
		PROC
	
		;LOCAL __MUL8A
		LOCAL __MUL8LOOP
		LOCAL __MUL8B
				; 1st operand (byte) in A, 2nd operand into the stack (AF)
		pop hl	; return address
		ex (sp), hl ; CALLE convention
	
;;__MUL8_FAST: ; __FASTCALL__ entry
	;;	ld e, a
	;;	ld d, 0
	;;	ld l, d
	;;	
	;;	sla h	
	;;	jr nc, __MUL8A
	;;	ld l, e
	;;
;;__MUL8A:
	;;
	;;	ld b, 7
;;__MUL8LOOP:
	;;	add hl, hl
	;;	jr nc, __MUL8B
	;;
	;;	add hl, de
	;;
;;__MUL8B:
	;;	djnz __MUL8LOOP
	;;
	;;	ld a, l ; result = A and HL  (Truncate to lower 8 bits)
	
__MUL8_FAST: ; __FASTCALL__ entry, a = a * h (8 bit mul) and Carry
	
	    ld b, 8
	    ld l, a
	    xor a
	
__MUL8LOOP:
	    add a, a ; a *= 2
	    sla l
	    jp nc, __MUL8B
	    add a, h
	
__MUL8B:
	    djnz __MUL8LOOP
		
		ret		; result = HL
		ENDP
	
#line 2243 "bacachase.bas"
#line 1 "u32tofreg.asm"
#line 1 "neg32.asm"
__ABS32:
		bit 7, d
		ret z
	
__NEG32: ; Negates DEHL (Two's complement)
		ld a, l
		cpl
		ld l, a
	
		ld a, h
		cpl
		ld h, a
	
		ld a, e
		cpl
		ld e, a
		
		ld a, d
		cpl
		ld d, a
	
		inc l
		ret nz
	
		inc h
		ret nz
	
		inc de
		ret
	
#line 2 "u32tofreg.asm"
__I8TOFREG:
		ld l, a
		rlca
		sbc a, a	; A = SGN(A)
		ld h, a
		ld e, a
		ld d, a
	
__I32TOFREG:	; Converts a 32bit signed integer (stored in DEHL)
					; to a Floating Point Number returned in (A ED CB)
	
		ld a, d
		or a		; Test sign
	
		jp p, __U32TOFREG	; It was positive, proceed as 32bit unsigned
	
		call __NEG32		; Convert it to positive
		call __U32TOFREG	; Convert it to Floating point
	
		set 7, e			; Put the sign bit (negative) in the 31bit of mantissa
		ret
	
__U8TOFREG:
					; Converts an unsigned 8 bit (A) to Floating point
		ld l, a
		ld h, 0
		ld e, h
		ld d, h
	
__U32TOFREG:	; Converts an unsigned 32 bit integer (DEHL)
					; to a Floating point number returned in A ED CB
	
	    PROC
	
	    LOCAL __U32TOFREG_END
	
		ld a, d
		or e
		or h
		or l
	    ld b, d
		ld c, e		; Returns 00 0000 0000 if ZERO
		ret z
	
		push de
		push hl
	
		exx
		pop de  ; Loads integer into B'C' D'E' 
		pop bc
		exx
	
		ld l, 128	; Exponent
		ld bc, 0	; DEBC = 0
		ld d, b
		ld e, c
	
__U32TOFREG_LOOP: ; Also an entry point for __F16TOFREG
		exx
		ld a, d 	; B'C'D'E' == 0 ?
		or e
		or b
		or c
		jp z, __U32TOFREG_END	; We are done
	
		srl b ; Shift B'C' D'E' >> 1, output bit stays in Carry
		rr c
		rr d
		rr e
		exx
	
		rr e ; Shift EDCB >> 1, inserting the carry on the left
		rr d
		rr c
		rr b
	
		inc l	; Increment exponent
		jp __U32TOFREG_LOOP
	
	
__U32TOFREG_END:
		exx
	    ld a, l     ; Puts the exponent in a
		res 7, e	; Sets the sign bit to 0 (positive)
	
		ret
	    ENDP
	
#line 2244 "bacachase.bas"
#line 1 "lti8.asm"
	
__LTI8: ; Test 8 bit values A < H
        ; Returns result in A: 0 = False, !0 = True
	        sub h
	
__LTI:  ; Signed CMP
	        PROC
	        LOCAL __PE
	
	        ld a, 0  ; Sets default to false
__LTI2:
	        jp pe, __PE
	        ; Overflow flag NOT set
	        ret p
	        dec a ; TRUE
	
__PE:   ; Overflow set
	        ret m
	        dec a ; TRUE
	        ret
	        
	        ENDP
#line 2245 "bacachase.bas"
#line 1 "div32.asm"
	
	
				 ; ---------------------------------------------------------
__DIVU32:    ; 32 bit unsigned division
	             ; DEHL = Dividend, Stack Top = Divisor
	             ; OPERANDS P = Dividend, Q = Divisor => OPERATION => P / Q
				 ;
				 ; Changes A, BC DE HL B'C' D'E' H'L'
				 ; ---------------------------------------------------------
	        exx
	        pop hl   ; return address
	        pop de   ; low part
	        ex (sp), hl ; CALLEE Convention ; H'L'D'E' => Dividend
	
__DIVU32START: ; Performs D'E'H'L' / HLDE
	        ; Now switch to DIVIDEND = B'C'BC / DIVISOR = D'E'DE (A / B)
	        push de ; push Lowpart(Q)
			ex de, hl	; DE = HL
	        ld hl, 0
	        exx
	        ld b, h
	        ld c, l
	        pop hl
	        push de
	        ex de, hl
	        ld hl, 0        ; H'L'HL = 0
	        exx
	        pop bc          ; Pop HightPart(B) => B = B'C'BC
	        exx
	
	        ld a, 32 ; Loop count
	
__DIV32LOOP:
	        sll c  ; B'C'BC << 1 ; Output most left bit to carry
	        rl  b
	        exx
	        rl c
	        rl b
	        exx
	
	        adc hl, hl
	        exx
	        adc hl, hl
	        exx
	
	        sbc hl,de
	        exx
	        sbc hl,de
	        exx
	        jp nc, __DIV32NOADD	; use JP inside a loop for being faster
	
	        add hl, de
	        exx
	        adc hl, de
	        exx
	        dec bc
	
__DIV32NOADD:
	        dec a
	        jp nz, __DIV32LOOP	; use JP inside a loop for being faster
	        ; At this point, quotient is stored in B'C'BC and the reminder in H'L'HL
	
	        push hl
	        exx
	        pop de
	        ex de, hl ; D'E'H'L' = 32 bits modulus
	        push bc
	        exx
	        pop de    ; DE = B'C'
	        ld h, b
	        ld l, c   ; DEHL = quotient D'E'H'L' = Modulus
	
	        ret     ; DEHL = quotient, D'E'H'L' = Modulus
	
	
	
__MODU32:    ; 32 bit modulus for 32bit unsigned division
	             ; DEHL = Dividend, Stack Top = Divisor (DE, HL)
	
	        exx
	        pop hl   ; return address
	        pop de   ; low part
	        ex (sp), hl ; CALLEE Convention ; H'L'D'E' => Dividend
	
	        call __DIVU32START	; At return, modulus is at D'E'H'L'
	
__MODU32START:
	
			exx
			push de
			push hl
	
			exx 
			pop hl
			pop de
	
			ret
	
	
__DIVI32:    ; 32 bit signed division
	             ; DEHL = Dividend, Stack Top = Divisor
	             ; A = Dividend, B = Divisor => A / B
	        exx
	        pop hl   ; return address
	        pop de   ; low part
	        ex (sp), hl ; CALLEE Convention ; H'L'D'E' => Dividend
	
__DIVI32START:
			exx
			ld a, d	 ; Save sign
			ex af, af'
			bit 7, d ; Negative?
			call nz, __NEG32 ; Negates DEHL
	
			exx		; Now works with H'L'D'E'
			ex af, af'
			xor h
			ex af, af'  ; Stores sign of the result for later
	
			bit 7, h ; Negative?
			ex de, hl ; HLDE = DEHL
			call nz, __NEG32
			ex de, hl 
	
			call __DIVU32START
			ex af, af' ; Recovers sign
			and 128	   ; positive?
			ret z
	
			jp __NEG32 ; Negates DEHL and returns from there
			
			
__MODI32:	; 32bits signed division modulus
			exx
	        pop hl   ; return address
	        pop de   ; low part
	        ex (sp), hl ; CALLEE Convention ; H'L'D'E' => Dividend
	
			call __DIVI32START
			jp __MODU32START		
	
#line 2246 "bacachase.bas"
#line 1 "and8.asm"
	; FASTCALL boolean and 8 version.
	; result in Accumulator (0 False, not 0 True)
; __FASTCALL__ version (operands: A, H)
	; Performs 8bit and 8bit and returns the boolean
	
__AND8:
		or a
		ret z
		ld a, h
		ret 
	
#line 2247 "bacachase.bas"
#line 1 "div8.asm"
				; --------------------------------
__DIVU8:	; 8 bit unsigned integer division 
				; Divides (Top of stack, High Byte) / A
		pop hl	; --------------------------------
		ex (sp), hl	; CALLEE
	
__DIVU8_FAST:	; Does A / H
		ld l, h
		ld h, a		; At this point do H / L
	
		ld b, 8
		xor a		; A = 0, Carry Flag = 0
		
__DIV8LOOP:
		sla	h		
		rla			
		cp	l		
		jr	c, __DIV8NOSUB
		sub	l		
		inc	h		
	
__DIV8NOSUB:	
		djnz __DIV8LOOP
	
		ld	l, a		; save remainder
		ld	a, h		; 
		
		ret			; a = Quotient, 
	
	
					; --------------------------------
__DIVI8:		; 8 bit signed integer division Divides (Top of stack) / A
		pop hl		; --------------------------------
		ex (sp), hl
	
__DIVI8_FAST:
		ld e, a		; store operands for later
		ld c, h
	
		or a		; negative?
		jp p, __DIV8A
		neg			; Make it positive
	
__DIV8A:
		ex af, af'
		ld a, h
		or a
		jp p, __DIV8B
		neg
		ld h, a		; make it positive
	
__DIV8B:
		ex af, af'
	
		call __DIVU8_FAST
	
		ld a, c
		xor l		; bit 7 of A = 1 if result is negative
	
		ld a, h		; Quotient
		ret p		; return if positive	
	
		neg
		ret
		
	
__MODU8:		; 8 bit module. REturns A mod (Top of stack) (unsigned operands)
		pop hl
		ex (sp), hl	; CALLEE
	
__MODU8_FAST:	; __FASTCALL__ entry
		call __DIVU8_FAST
		ld a, l		; Remainder
	
		ret		; a = Modulus
	
	
__MODI8:		; 8 bit module. REturns A mod (Top of stack) (For singed operands)
		pop hl
		ex (sp), hl	; CALLEE
	
__MODI8_FAST:	; __FASTCALL__ entry
		call __DIVI8_FAST
		ld a, l		; remainder
	
		ret		; a = Modulus
	
#line 2248 "bacachase.bas"
#line 1 "lti16.asm"
	
	
	
__LTI16: ; Test 8 bit values HL < DE
        ; Returns result in A: 0 = False, !0 = True
	        xor a
	        sbc hl, de
	        jp __LTI2
	
#line 2249 "bacachase.bas"
#line 1 "ftou32reg.asm"
	
	
__FTOU32REG:	; Converts a Float to (un)signed 32 bit integer (NOTE: It's ALWAYS 32 bit signed)
					; Input FP number in A EDCB (A exponent, EDCB mantissa)
				; Output: DEHL 32 bit number (signed)
		PROC
	
		LOCAL __IS_FLOAT
	
		or a
		jr nz, __IS_FLOAT 
		; Here if it is a ZX ROM Integer
	
		ld h, c
		ld l, d
	ld a, e	 ; Takes sign: FF = -, 0 = +
		ld de, 0
		inc a
		jp z, __NEG32	; Negates if negative
		ret
	
__IS_FLOAT:  ; Jumps here if it is a true floating point number
		ld h, e	
		push hl  ; Stores it for later (Contains Sign in H)
	
		push de
		push bc
	
		exx
		pop de   ; Loads mantissa into C'B' E'D' 
		pop bc	 ; 
	
		set 7, c ; Highest mantissa bit is always 1
		exx
	
		ld hl, 0 ; DEHL = 0
		ld d, h
		ld e, l
	
		;ld a, c  ; Get exponent
		sub 128  ; Exponent -= 128
		jr z, __FTOU32REG_END	; If it was <= 128, we are done (Integers must be > 128)
		jr c, __FTOU32REG_END	; It was decimal (0.xxx). We are done (return 0)
	
		ld b, a  ; Loop counter = exponent - 128
	
__FTOU32REG_LOOP:
		exx 	 ; Shift C'B' E'D' << 1, output bit stays in Carry
		sla d
		rl e
		rl b
		rl c
	
	    exx		 ; Shift DEHL << 1, inserting the carry on the right
		rl l
		rl h
		rl e
		rl d
	
		djnz __FTOU32REG_LOOP
	
__FTOU32REG_END:
		pop af   ; Take the sign bit
		or a	 ; Sets SGN bit to 1 if negative
		jp m, __NEG32 ; Negates DEHL
		
		ret
	
		ENDP
	
	
__FTOU8:	; Converts float in C ED LH to Unsigned byte in A
		call __FTOU32REG
		ld a, l
		ret
	
#line 2250 "bacachase.bas"
#line 1 "band16.asm"
; vim:ts=4:et:
	; FASTCALL bitwise and16 version.
	; result in hl 
; __FASTCALL__ version (operands: A, H)
	; Performs 16bit or 16bit and returns the boolean
; Input: HL, DE
; Output: HL <- HL AND DE
	
__BAND16:
		ld a, h
		and d
	    ld h, a
	
	    ld a, l
	    and e
	    ld l, a
	
	    ret 
	
#line 2251 "bacachase.bas"
#line 1 "div16.asm"
	; 16 bit division and modulo functions 
	; for both signed and unsigned values
	
#line 1 "neg16.asm"
	; Negates HL value (16 bit)
__ABS16:
		bit 7, h
		ret z
	
__NEGHL:
		ld a, l			; HL = -HL
		cpl
		ld l, a
		ld a, h
		cpl
		ld h, a
		inc hl
		ret
	
#line 5 "div16.asm"
	
__DIVU16:    ; 16 bit unsigned division
	             ; HL = Dividend, Stack Top = Divisor
	
		;   -- OBSOLETE ; Now uses FASTCALL convention
		;   ex de, hl
	    ;	pop hl      ; Return address
	    ;	ex (sp), hl ; CALLEE Convention
	
__DIVU16_FAST:
	    ld a, h
	    ld c, l
	    ld hl, 0
	    ld b, 16
	
__DIV16LOOP:
	    sll c
	    rla
	    adc hl,hl
	    sbc hl,de
	    jr  nc, __DIV16NOADD
	    add hl,de
	    dec c
	
__DIV16NOADD:
	    djnz __DIV16LOOP
	
	    ex de, hl
	    ld h, a
	    ld l, c
	
	    ret     ; HL = quotient, DE = Mudulus
	
	
	
__MODU16:    ; 16 bit modulus
	             ; HL = Dividend, Stack Top = Divisor
	
	    ;ex de, hl
	    ;pop hl
	    ;ex (sp), hl ; CALLEE Convention
	
	    call __DIVU16_FAST
	    ex de, hl	; hl = reminder (modulus)
					; de = quotient
	
	    ret
	
	
__DIVI16:	; 16 bit signed division
		;	--- The following is OBSOLETE ---
		;	ex de, hl
		;	pop hl
		;	ex (sp), hl 	; CALLEE Convention
	
__DIVI16_FAST:
		ld a, d
		xor h
		ex af, af'		; BIT 7 of a contains result
	
		bit 7, d		; DE is negative?
		jr z, __DIVI16A	
	
		ld a, e			; DE = -DE
		cpl
		ld e, a
		ld a, d
		cpl
		ld d, a
		inc de
	
__DIVI16A:
		bit 7, h		; HL is negative?
		call nz, __NEGHL
	
__DIVI16B:
		call __DIVU16_FAST
		ex af, af'
	
		or a	
		ret p	; return if positive
	    jp __NEGHL
	
		
__MODI16:    ; 16 bit modulus
	             ; HL = Dividend, Stack Top = Divisor
	
	    ;ex de, hl
	    ;pop hl
	    ;ex (sp), hl ; CALLEE Convention
	
	    call __DIVI16_FAST
	    ex de, hl	; hl = reminder (modulus)
					; de = quotient
	
	    ret
	
#line 2252 "bacachase.bas"
#line 1 "swap32.asm"
	; Exchanges current DE HL with the
	; ones in the stack
	
__SWAP32:
		pop bc ; Return address
	
		exx
		pop hl	; exx'
		pop de
	
		exx
		push de ; exx
		push hl
	
		exx		; exx '
		push de
		push hl
		
		exx		; exx
		pop hl
		pop de
	
		push bc
	
		ret
	
#line 2253 "bacachase.bas"
	
ZXBASIC_USER_DATA:
	_seed EQU 49168
	_i EQU 49170
	_j EQU 49172
	_enam EQU 49174
	_cnt EQU 49176
	_stam EQU 49178
	_stg EQU 49180
	_score EQU 49182
	_hiscore EQU 49184
	_seedo EQU 49186
	_x EQU 49188
	_y EQU 49189
	_d EQU 49190
	_kprs EQU 49191
	_debug EQU 49192
	_c EQU 49193
	_boolbf EQU 49194
	_seedb EQU 49195
	_xintp EQU 49196
	_yintp EQU 49197
	_stintp EQU 49198
	_dintp EQU 49199
	_jacum EQU 49200
	_xintq EQU 49201
	_yintq EQU 49202
	_sg1000writetext_tq2 EQU 49152
	; Defines DATA END --> HEAP size is 0
ZXBASIC_USER_DATA_END EQU ZXBASIC_MEM_HEAP
	; Defines USER DATA Length in bytes
ZXBASIC_USER_DATA_LEN EQU ZXBASIC_USER_DATA_END - ZXBASIC_USER_DATA
	END
