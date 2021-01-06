# TODO

Implementing the N-`for` N-`if` version of exhaustive requires:
- `for` and `if` are stored in vectors
  - `if`'s optionallity becomes the emptiness of the vector
- A combinations style iterator
  - Nested `for`s are not possible with `Zip`, since it iterates pair-wise
  - The iterator can't be imported from `itertools`, since the output mustn't depend on anything else
    - It can however, be copied from there