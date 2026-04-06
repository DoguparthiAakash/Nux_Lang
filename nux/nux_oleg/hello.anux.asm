; Hello World / Solitaire Demo in Anux Assembly

; 1. Draw Green Background (Color, H, W, Y, X)
; Color: 0xFF008000 (Green)
PUSH 0
PUSH 0
PUSH 800
PUSH 600
PUSH 4278222848 ; 0xFF008000
DRAW_RECT

; 2. Draw White Card (Color, H, W, Y, X)
; Color: 0xFFFFFFFF (White)
PUSH 50
PUSH 50
PUSH 70
PUSH 100
PUSH 4294967295 ; 0xFFFFFFFF
DRAW_RECT

; 3. Draw Red Card (Red)
; Color: 0xFFFF0000 (Red)
PUSH 200
PUSH 50
PUSH 70
PUSH 100
PUSH 4294901760 ; 0xFFFF0000
DRAW_RECT

; 4. Exit
EXIT
