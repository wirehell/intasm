        halt
        spa 1
        add 1, 1, [0]
        add 1+1*1, 1, [0]
.label  add ( 1 + 1 + 1 * 1 ), 1, [0]
        add 1 * 2  + 3 *4, 1, %[-3]
;        add 1+2*3+4, 1, 0
;        add [count] 1 [count]            ; comment
;        eq [tmp] size [comp]
;        jif [comp] loop
;        out [%sp-1]                       ; comment
;        jit
;        add
;        mul
;        lt
