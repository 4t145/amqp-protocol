# Read

```rust
pub struct View<'frame, T> {
    pub data: &'frame [u8],
    pub _: PhantomData<T>
}

pub struct MutView<'frame, T> {
    pub data: &mut 'frame [u8],
    pub _: PhantomData<T>
}

impl Sreialisd{
    
}
```

```rust
pub struct AmqpArray<'frame, T> {
    constructor: Constructor<'frame>,
    count: usize,
    data: Data<'frame, T>,
    owned_data: &'frame [u8],
}


impl Forward
pub struct Value<'frame> {
    constructor: Constructor<'frame>
    data: Data<'frame>,
}

pub struct Data<'frame> {
    value: &'frame [u8]
}
```
```
[constructor] [[size][count][item1][item2][item3][item4]]
                              ^       ^
pub struct X<'a'> {           |       |
    pub f1: &'a str, ---------+       |
    pub f2: &'a str, -----------------+
    pub f3: 
}
```


读通路 -> Bytes -> Value -> Rust Reference Type
写通路 -> Rust Reference Type -> ValueWriter -> Bytes