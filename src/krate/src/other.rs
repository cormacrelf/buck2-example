pub struct Structure(std::marker::PhantomData<()>);

impl Structure {
    pub fn new() -> Self {
        Self(std::marker::PhantomData)
    }
}

#[test]
fn test_other() {
    let s = Structure::new();
}
