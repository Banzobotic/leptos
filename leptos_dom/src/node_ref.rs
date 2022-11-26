use leptos_reactive::{
    create_rw_signal, RwSignal, Scope, UntrackedGettableSignal, UntrackedSettableSignal,
};

/// Contains a shared reference to a DOM node creating while using the [view](leptos::view)
/// macro to create your UI.
///
/// ```
/// # use leptos::*;
/// #[component]
/// pub fn MyComponent(cx: Scope) -> Element {
///   let input_ref = NodeRef::new(cx);
///
///   let on_click = move |_| {
///     let node = input_ref
///       .get()
///       .expect("input_ref should be loaded by now")
///       .unchecked_into::<web_sys::HtmlInputElement>();
///     log!("value is {:?}", node.value())
///   };
///
///   view! {
///     cx,
///     <div>
///     // `node_ref` loads the input
///     <input _ref=input_ref type="text"/>
///     // the button consumes it
///     <button on:click=on_click>"Click me"</button>
///     </div>
///   }
/// }
/// ```
#[derive(Copy, Clone, PartialEq)]
pub struct NodeRef(RwSignal<Option<web_sys::Element>>);

impl NodeRef {
    /// Creates an empty reference.
    pub fn new(cx: Scope) -> Self {
        Self(create_rw_signal(cx, None))
    }

    /// Gets the element that is currently stored in the reference.
    pub fn get(&self) -> Option<web_sys::Element> {
        self.0.get_untracked()
    }

    #[doc(hidden)]
    /// Loads an element into the reference/
    pub fn load(&self, node: &web_sys::Element) {
        self.0.set_untracked(Some(node.clone()))
    }
}

cfg_if::cfg_if! {
    if #[cfg(not(feature = "stable"))] {
        impl FnOnce<()> for NodeRef {
            type Output = Option<web_sys::Element>;

            extern "rust-call" fn call_once(self, _args: ()) -> Self::Output {
                self.get()
            }
        }

        impl FnMut<()> for NodeRef {
            extern "rust-call" fn call_mut(&mut self, _args: ()) -> Self::Output {
                self.get()
            }
        }

        impl Fn<()> for NodeRef {
            extern "rust-call" fn call(&self, _args: ()) -> Self::Output {
                self.get()
            }
        }
    }
}