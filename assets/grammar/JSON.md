# JSON Grammar

The formal definition that describes valid JSON syntax is defined by the following grammar,
expressed
in [ABNF](https://en.wikipedia.org/wiki/Augmented_Backus%E2%80%93Naur_form), and copied
from [IETF JSON standard (RFC)][rfc-8259]:

```text
JSON-text = object / array
begin-array     = ws %x5B ws  ; [ left square bracket
begin-object    = ws %x7B ws  ; { left curly bracket
end-array       = ws %x5D ws  ; ] right square bracket
end-object      = ws %x7D ws  ; } right curly bracket
name-separator  = ws %x3A ws  ; : colon
value-separator = ws %x2C ws  ; , comma
ws = *(
     %x20 /              ; Space
     %x09 /              ; Horizontal tab
     %x0A /              ; Line feed or New line
     %x0D                ; Carriage return
     )
value = false / null / true / object / array / number / string
false = %x66.61.6c.73.65   ; false
null  = %x6e.75.6c.6c      ; null
true  = %x74.72.75.65      ; true
object = begin-object [ member *( value-separator member ) ]
         end-object
member = string name-separator value
array = begin-array [ value *( value-separator value ) ] end-array
number = [ minus ] int [ frac ] [ exp ]
decimal-point = %x2E       ; .
digit1-9 = %x31-39         ; 1-9
e = %x65 / %x45            ; e E
exp = e [ minus / plus ] 1*DIGIT
frac = decimal-point 1*DIGIT
int = zero / ( digit1-9 *DIGIT )
minus = %x2D               ; -
plus = %x2B                ; +
zero = %x30                ; 0
string = quotation-mark *char quotation-mark
char = unescaped /
    escape (
        %x22 /          ; "    quotation mark  U+0022
        %x5C /          ; \    reverse solidus U+005C
        %x2F /          ; /    solidus         U+002F
        %x62 /          ; b    backspace       U+0008
        %x66 /          ; f    form feed       U+000C
        %x6E /          ; n    line feed       U+000A
        %x72 /          ; r    carriage return U+000D
        %x74 /          ; t    tab             U+0009
        %x75 4HEXDIG )  ; uXXXX                U+XXXX
escape = %x5C              ; \
quotation-mark = %x22      ; "
unescaped = %x20-21 / %x23-5B / %x5D-10FFFF
HEXDIG = DIGIT / %x41-46 / %x61-66   ; 0-9, A-F, or a-f
       ; HEXDIG equivalent to HEXDIG rule in [RFC5234]
DIGIT = %x30-39            ; 0-9
      ; DIGIT equivalent to DIGIT rule in [RFC5234]
```

## Insignificant Whitespace

Insignificant whitespace may be present anywhere except within a `number` or `string`.

The only valid whitespace characters are:

- tab: (`U+0009`)
- carriage return: (`U-000D`)
- line feed (`U+000A`)
- space (`U+0020`)

## Attributions

We appreciate the individuals and organizations that provided the references needed to make this
project possible. Below are the sources that were used to assemble this document and put together a
formal definition.

### Mozilla

Their documentation and reference on JSON contains the full JSON grammar.

<https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/JSON#full_json_grammar>

### Internet Engineering Task Force (IETF)

The JavaScript Object Notation (JSON) Data Interchange Format can be found via their website, which
hosts essentially all open-sourced internet standards in existence.

<https://datatracker.ietf.org/doc/html/rfc8259>

<!-- Inline Links -->

[mdn-web-docs]: https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/JSON#full_json_grammar

[rfc-8259]: https://datatracker.ietf.org/doc/html/rfc8259
