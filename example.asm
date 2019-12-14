#def    count  100
#def    comp_res 101
#def    size 16*3
#def    moho  1 + size * count + comp_res

#foo

.loop   spa  1
        out %[1]                      ; comment
        add [table], 1, [count]       ; comment
        add 1, 1, [0]
        eq [tmp], size, [comp]
        mul 1, 1, %[0]
        jif [comp], loop
        halt

.table  dw  1234,1235,4,127,88
.msg    dw  "hello world", 0
.zeros  dup 10, 0
.end    dw 0


