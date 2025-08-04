use std::rc::Rc;
use std::cell::RefCell;

use crate::modpub;

modpub!(clickable);
modpub!(collideable);
modpub!(displayable);
modpub!(draggable);
modpub!(hoverable);
modpub!(inputtable);
modpub!(updateable);


/// An "element" - something that implements some or all of the behaviours specified 
#[derive(Default)]
pub struct Implementor {
    pub clickable: Option<Rc<RefCell<dyn Clickable>>>,
    pub collideable: Option<Rc<RefCell<dyn Collideable>>>,
    pub displayable: Option<Rc<RefCell<dyn Displayable>>>,
    pub draggable: Option<Rc<RefCell<dyn Draggable>>>,
    pub hoverable: Option<Rc<RefCell<dyn Hoverable>>>,
    pub inputtable: Option<Rc<RefCell<dyn Inputtable>>>,
    pub updateable: Option<Rc<RefCell<dyn Updateable>>>
}

/// The base trait user-made Implementors (game elements)
pub trait ImplementorBuilder {
    fn spawn() -> Implementor;
    fn despawn(&mut self);
}

#[macro_export]
macro_rules! build_implementor {
    ( 
        ($($behaviour:ident$(,)?)*)
        $($body:tt)*
    ) => {
        let rc = Rc::new(RefCell::new($($body)*));
    
        return Implementor {
            $( $behaviour: Some(rc.clone()), )*
            ..Default::default()
        };
    }
}

pub use build_implementor;
