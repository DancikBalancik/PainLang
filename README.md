# Pain.

> A programming language designed to punish curiosity.

**Pain.** is a multi-tiered esoteric programming language suite written in Rust.
It features four escalating levels of syntactic and semantic torment.

---

## 🔥 Levels of Pain

| Level | Name                              | Description                                                       |
|-------|-----------------------------------|-------------------------------------------------------------------|
| 1     | The Knife                         | Basic tape-based interpreter (Brainfuck-like)                     |
| 2     | The Saw                           | Inverted commands and fake error messages                         |
| 3     | The Chainsaw                      | Base64-encoded bytecode language with fragile logic               |
| 4     | `[!!SYS_ERR: R3@L P@!N⧸L⧸NG!!]`    | Commands made entirely of screams — no words, just anguish        |

---

## 🛠 Usage

```sh
# Run a Pain. program (defaults to Level 1)
cargo run -- path/to/file.pain

# Run with a specific level (1–4)
cargo run -- path/to/file.pain --level=2

# The --compile flag is a planned feature and is not yet implemented.
# Running it will currently cause a panic.
cargo run -- path/to/file.pain --compile
```

---

## 🧪 Level Syntaxes

### 🔪 Level 1 — The Knife
A simple tape-based interpreter.
```
~  Move pointer right
!  Move pointer left
*  Increment current cell
^  Decrement current cell
@  Output ASCII char
#  Input one byte
{  Begin loop
}  End loop
```
All other characters are ignored.

### 🪚 Level 2 — The Saw
Reversed Brainfuck with a fragile and confusing design.
```
<  Move right
>  Move left
-  Increment cell
+  Decrement cell
.  Output
,  Input
]  Start loop
[  End loop
```
Errors are designed to mislead:
```
SAW ERROR: Teeth misaligned: ']' not closed.
SAW ERROR: Blade jammed on unmatched '['.
```

### ⚙️ Level 3 — The Chainsaw
The source code must be a valid Base64 string. The decoded bytes map to commands.
```
base64('aacceegghh...')
Where:
  a = right,    b = left
  c = inc,      d = dec
  e = output,   f = input
  g = loop start, h = loop end
```
The interpreter will crash if it encounters invalid Base64 or unknown command bytes.

### 💥 Level 4 — `[!!SYS_ERR: R3@L P@!N⧸L⧸NG!!]`
Commands are made from literal screams, parsed by a case-insensitive regex matcher.
```
A{5,}GH                Move pointer right (e.g., AAAAAAGH)
A{1,4}H                Move pointer left  (e.g., AAAH)
A{5,}H                 Increment cell     (e.g., AAAAAAAAH)
[aA]+GH                Decrement cell     (e.g., aaAAaaGH)
AAGH                   Output
U+GH                   Input              (e.g., UUUUGH)
```
Commands are matched by pattern, not exact spelling. Unintelligible screams result in `FATAL SYSTEM ANGUISH`.

---

## 💀 License

MIT. Use it however you like. Or don’t. Pain is free.