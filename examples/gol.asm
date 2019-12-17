#def    x_size  80
#def    y_size  43

#def    world_size x_size * y_size

#def    cell 79
#def    space 46

#def    OW  999999          ; Overwritten pointers

        jit [true], init    ; Entry point

; ------------------ Global vars -----------------------------
.false  dw 0                ; Helper for non conditional jumps
.true   dw 1                ;   - " -

.tmp    dw 0

.source dw 0                ; Pointer to current buffer
.dest   dw 0                ; Pointer to next gen buffer
;------------------- End of global vars ------------------------

;------------------------ Init ---------------------------------
.i1     dw 0
.i2     dw 0
.ieq    dw 0
.init   add world, 0, [source]              ; Setup source pointer
        add world, world_size, [dest]       ; Setup dest pointer
        add [dest], 0, [i1]                 ;

.l      add [i1], -1, [i1]                  ; i1--
        add [i1], world_size, [i2]          ; setup target

                                            ; Self modifying code test..
        add [i1], 0 , [copyi+1]             ; Rewrite source pos parameter in instruction below
        add [i2], 0 , [copyi+3]             ; Rewrite target pos parameter in instruction below
.copyi  add [0], 0, [0]                     ; Dummy values, but modes needs to be correct

        eq world, [i1], [ieq]
        jif [ieq], l                         ; while i1 != dest (copy from top to bottom)

;----------------------- Init End ------------------------------
        jit [true], print

;-------------------------Evolve --------------------------------

.ei     dw 0
.esi    dw 0
.edi    dw 0
.nc     dw 0
.nbres  dw 0
.nbptr  dw 0

.evolve add [source], world_size, [esi]
        add [dest], world_size, [edi]
        add world_size, 0, [ei]

.eloop  add [esi], -1, [esi]
        add [edi], -1, [edi]
        add [ei], -1, [ei]

; Iterate over neighbours
        add 0, 0, [nc]          ; neighbour count = 0

;; Above
        add [esi], - x_size - 1, [nb0+1]
.nb0    eq [OW], cell, [nbres]
        add [nbres], [nc], [nc]

        add [esi], -x_size, [nb1+1]
.nb1    eq [OW], cell, [nbres]
        add [nbres], [nc], [nc]

        add [esi], -x_size + 1 , [nb2+1]
.nb2    eq [OW], cell, [nbres]
        add [nbres], [nc], [nc]

;; Left|Right
        add [esi], -1 , [nb3+1]
.nb3    eq [OW], cell, [nbres]
        add [nbres], [nc], [nc]

        add [esi], 1 , [nb4+1]
.nb4    eq [OW], cell, [nbres]
        add [nbres], [nc], [nc]

;; Below

        add [esi], x_size - 1, [nb5+1]
.nb5    eq [OW], cell, [nbres]
        add [nbres], [nc], [nc]

        add [esi], x_size, [nb6+1]
.nb6    eq [OW], cell, [nbres]
        add [nbres], [nc], [nc]

        add [esi], x_size + 1 , [nb7+1]
.nb7    eq [OW], cell, [nbres]
        add [nbres], [nc], [nc]


; Life or death calculations

        eq [nc], 3, [tmp]       ; If 3, always live
        jit [tmp], clive

        eq [nc], 2, [tmp]       ; If 2, copy
        jit [tmp], ccopy
                                ;else kill
        add [edi], 0, [cdw+3] ;
.cdw    add space, 0, [OW] ; Cell death
        jit [true], cdone

.clive   add [edi], 0, [cbw+3] ;
.cbw    add cell, 0, [OW] ; Cell born or kept alive
        jit [true], cdone

.ccopy  add [esi], 0, [nochw+1] ; Set read address
        add [edi], 0, [nochw+3] ; Set write address
.nochw  add [esi], 0, [edi]     ; Copy cell

.cdone  jit [ei], eloop
        jit [true], print


;------------------------Evolve End ----------------------------


;--------------------- Printing ----------------------------
.py     dw 0                                ; y index
.px     dw 0                                ; z index
.pi     dw 0
.pc     dw 0                                ; Conditions
.base   dw 0

.print  add [dest], 0, [pi]     ; i = dest
        add y_size, 0, [py]                 ; y = y_size
.ply    add [py], -1, [py]                  ; y = y-1
        add x_size, 0, [px]                 ; x = x_size
.plx    add [px], -1, [px]                  ; x = x-1
        ; add new check

        out [px]
        out [py]

        add [pi], 0, [cp+1]                 ; Modify source of instruction below
.cp     add [0], 0, [tmp]                   ; Dummy source, copy

        eq [tmp], 79, [tmp]
        out [tmp]

        add [pi], 1, [pi]                   ; i++

        jit [px], plx                       ; while x != 0
        jit [py], ply                       ; while y != 0

        ; output ball                       ; for sync and non breaking
        out 0
        out 0
        out 3
        ; output paddle                     :  - " -
        out 0
        out 0
        out 4

        add [source], 0, [tmp]
        add [dest], 0, [source]
        add [tmp], 0, [dest]
        jit [true], evolve                   ;return

; -------------------- End of Printing ---------------------------

.world  dw "................................................................................"
        dw "................................................................................"
        dw "................................................................................"
        dw "................................................................................"
        dw "................................................................................"
        dw "......................................OO........................OO.............."
        dw "......................................OO........................OO.............."
        dw "........................................................OO......................"
        dw ".......................................................O..O....................."
        dw "........................................................OO......................"
        dw "................................................................................"
        dw "...................................................OOO.........................."
        dw "...................................................O.O.........................."
        dw "........................OO.........................OOO.........................."
        dw "........................OO.........................OO..........................."
        dw ".......................O..O.......................OOO..........................."
        dw ".......................O..O.OO....................O.O..........................."
        dw ".......................O....OO....................OOO..........................."
        dw ".........................OO.OO.................................................."
        dw "..............................................OO................................"
        dw "....................................OO.......O..O..............................."
        dw "....................................OO........OO................................"
        dw "................................................................OO.............."
        dw "................................................................OO.............."
        dw "................................................................................"
        dw "...................OO..................O........................................"
        dw "...............OO....OOOO..........OO..OO.OOO..................................."
        dw "...............OO..OO.OOO..........OO....OOOO..................................."
        dw "...................O...................OO......................................."
        dw "................................................................................"
        dw "................................................................................"
        dw "................................................................................"
        dw "................................................................................"
        dw "................................................................................"

