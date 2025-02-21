#[doc = "callback object\n\nClients can handle the 'done' event to get notified when\nthe related request is done."]
pub mod wl_callback {
    use super::wayland_server::{
        backend::{
            protocol::{same_interface, Argument, Interface, Message, WEnum},
            smallvec, InvalidId, ObjectData, ObjectId, WeakHandle,
        },
        Dispatch, DispatchError, DisplayHandle, New, Resource, ResourceData,
    };
    use std::sync::Arc;
    #[doc = r" The minimal object version supporting this event"]
    pub const EVT_DONE_SINCE: u32 = 1u32;
    #[doc = r" The wire opcode for this event"]
    pub const EVT_DONE_OPCODE: u16 = 0u16;
    #[derive(Debug)]
    #[non_exhaustive]
    pub enum Request {}
    #[derive(Debug)]
    #[non_exhaustive]
    pub enum Event {
        #[doc = "done event\n\nNotify the client when the related request is done.\n\nThis is a destructor, once sent this object cannot be used any longer."]
        Done {
            #[doc = "request-specific data for the callback"]
            callback_data: u32
        },
    }
    #[doc = "callback object\n\nClients can handle the 'done' event to get notified when\nthe related request is done.\n\nThis interface has no requests."]
    #[derive(Debug, Clone)]
    pub struct WlCallback {
        id: ObjectId,
        version: u32,
        data: Option<Arc<dyn std::any::Any + Send + Sync + 'static>>,
        handle: WeakHandle,
    }
    impl std::cmp::PartialEq for WlCallback {
        fn eq(&self, other: &WlCallback) -> bool {
            self.id == other.id
        }
    }
    impl std::cmp::Eq for WlCallback {}
    impl super::wayland_server::Resource for WlCallback {
        type Request = Request;
        type Event = Event;
        #[inline]
        fn interface() -> &'static Interface {
            &super::WL_CALLBACK_INTERFACE
        }
        #[inline]
        fn id(&self) -> ObjectId {
            self.id.clone()
        }
        #[inline]
        fn version(&self) -> u32 {
            self.version
        }
        #[inline]
        fn data<U: 'static>(&self) -> Option<&U> {
            self.data
                .as_ref()
                .and_then(|arc| (&**arc).downcast_ref::<ResourceData<Self, U>>())
                .map(|data| &data.udata)
        }
        #[inline]
        fn object_data(&self) -> Option<&Arc<dyn std::any::Any + Send + Sync>> {
            self.data.as_ref()
        }
        fn handle(&self) -> &WeakHandle {
            &self.handle
        }
        #[inline]
        fn from_id(conn: &DisplayHandle, id: ObjectId) -> Result<Self, InvalidId> {
            if !same_interface(id.interface(), Self::interface()) && !id.is_null() {
                return Err(InvalidId);
            }
            let version = conn.object_info(id.clone()).map(|info| info.version).unwrap_or(0);
            let data = conn.get_object_data(id.clone()).ok();
            Ok(WlCallback { id, data, version, handle: conn.backend_handle().downgrade() })
        }
        fn send_event(&self, evt: Self::Event) -> Result<(), InvalidId> {
            let handle = DisplayHandle::from(self.handle.upgrade().ok_or(InvalidId)?);
            handle.send_event(self, evt)
        }
        fn parse_request(
            conn: &DisplayHandle,
            msg: Message<ObjectId>,
        ) -> Result<(Self, Self::Request), DispatchError> {
            let me = Self::from_id(conn, msg.sender_id.clone()).unwrap();
            match msg.opcode {
                _ => Err(DispatchError::BadMessage { msg, interface: Self::interface().name }),
            }
        }
        fn write_event(
            &self,
            conn: &DisplayHandle,
            msg: Self::Event,
        ) -> Result<Message<ObjectId>, InvalidId> {
            match msg {
                Event::Done { callback_data } => Ok(Message {
                    sender_id: self.id.clone(),
                    opcode: 0u16,
                    args: smallvec::smallvec![Argument::Uint(callback_data)],
                }),
            }
        }
        fn __set_object_data(
            &mut self,
            odata: std::sync::Arc<dyn std::any::Any + Send + Sync + 'static>,
        ) {
            self.data = Some(odata);
        }
    }
    impl WlCallback {
        #[doc = "done event\n\nNotify the client when the related request is done."]
        #[allow(clippy::too_many_arguments)]
        pub fn done(&self, callback_data: u32) {
            let _ = self.send_event(Event::Done { callback_data });
        }
    }
}
pub mod test_global {
    use super::wayland_server::{
        backend::{
            protocol::{same_interface, Argument, Interface, Message, WEnum},
            smallvec, InvalidId, ObjectData, ObjectId, WeakHandle,
        },
        Dispatch, DispatchError, DisplayHandle, New, Resource, ResourceData,
    };
    use std::sync::Arc;
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_MANY_ARGS_SINCE: u32 = 1u32;
    #[doc = r" The wire opcode for this request"]
    pub const REQ_MANY_ARGS_OPCODE: u16 = 0u16;
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_GET_SECONDARY_SINCE: u32 = 2u32;
    #[doc = r" The wire opcode for this request"]
    pub const REQ_GET_SECONDARY_OPCODE: u16 = 1u16;
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_GET_TERTIARY_SINCE: u32 = 3u32;
    #[doc = r" The wire opcode for this request"]
    pub const REQ_GET_TERTIARY_OPCODE: u16 = 2u16;
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_LINK_SINCE: u32 = 3u32;
    #[doc = r" The wire opcode for this request"]
    pub const REQ_LINK_OPCODE: u16 = 3u16;
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_DESTROY_SINCE: u32 = 4u32;
    #[doc = r" The wire opcode for this request"]
    pub const REQ_DESTROY_OPCODE: u16 = 4u16;
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_REVERSE_LINK_SINCE: u32 = 5u32;
    #[doc = r" The wire opcode for this request"]
    pub const REQ_REVERSE_LINK_OPCODE: u16 = 5u16;
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_NEWID_AND_ALLOW_NULL_SINCE: u32 = 5u32;
    #[doc = r" The wire opcode for this request"]
    pub const REQ_NEWID_AND_ALLOW_NULL_OPCODE: u16 = 6u16;
    #[doc = r" The minimal object version supporting this event"]
    pub const EVT_MANY_ARGS_EVT_SINCE: u32 = 1u32;
    #[doc = r" The wire opcode for this event"]
    pub const EVT_MANY_ARGS_EVT_OPCODE: u16 = 0u16;
    #[doc = r" The minimal object version supporting this event"]
    pub const EVT_ACK_SECONDARY_SINCE: u32 = 1u32;
    #[doc = r" The wire opcode for this event"]
    pub const EVT_ACK_SECONDARY_OPCODE: u16 = 1u16;
    #[doc = r" The minimal object version supporting this event"]
    pub const EVT_CYCLE_QUAD_SINCE: u32 = 1u32;
    #[doc = r" The wire opcode for this event"]
    pub const EVT_CYCLE_QUAD_OPCODE: u16 = 2u16;
    #[derive(Debug)]
    #[non_exhaustive]
    pub enum Request {
        #[doc = "a request with every possible non-object arg"]
        ManyArgs {
            #[doc = "an unsigned int"]
            unsigned_int: u32,
            #[doc = "a singed int"]
            signed_int: i32,
            #[doc = "a fixed point number"]
            fixed_point: f64,
            #[doc = "an array"]
            number_array: Vec<u8>,
            #[doc = "some text"]
            some_text: String,
            #[doc = "a file descriptor"]
            file_descriptor: ::std::os::unix::io::RawFd,
        },
        #[doc = "Only available since version 2 of the interface"]
        GetSecondary {
            #[doc = "create a secondary"]
            sec: New<super::secondary::Secondary>,
        },
        #[doc = "Only available since version 3 of the interface"]
        GetTertiary {
            #[doc = "create a tertiary"]
            ter: New<super::tertiary::Tertiary>,
        },
        #[doc = "link a secondary and a tertiary\n\n\n\nOnly available since version 3 of the interface"]
        Link { sec: super::secondary::Secondary, ter: Option<super::tertiary::Tertiary>, time: u32 },
        #[doc = "This is a destructor, once received this object cannot be used any longer.\nOnly available since version 4 of the interface"]
        Destroy,
        #[doc = "reverse link a secondary and a tertiary\n\n\n\nOnly available since version 5 of the interface"]
        ReverseLink { sec: Option<super::secondary::Secondary>, ter: super::tertiary::Tertiary },
        #[doc = "a newid request that also takes allow null arg\n\n\n\nOnly available since version 5 of the interface"]
        NewidAndAllowNull {
            quad: New<super::quad::Quad>,
            sec: Option<super::secondary::Secondary>,
            ter: super::tertiary::Tertiary,
        },
    }
    #[derive(Debug)]
    #[non_exhaustive]
    pub enum Event {
        #[doc = "an event with every possible non-object arg"]
        ManyArgsEvt {
            #[doc = "an unsigned int"]
            unsigned_int: u32,
            #[doc = "a singed int"]
            signed_int: i32,
            #[doc = "a fixed point number"]
            fixed_point: f64,
            #[doc = "an array"]
            number_array: Vec<u8>,
            #[doc = "some text"]
            some_text: String,
            #[doc = "a file descriptor"]
            file_descriptor: ::std::os::unix::io::RawFd,
        },
        #[doc = "acking the creation of a secondary"]
        AckSecondary { sec: super::secondary::Secondary },
        #[doc = "create a new quad optionally replacing a previous one"]
        CycleQuad { new_quad: super::quad::Quad, old_quad: Option<super::quad::Quad> },
    }
    #[doc = "test_global\n\nSee also the [Request] enum for this interface."]
    #[derive(Debug, Clone)]
    pub struct TestGlobal {
        id: ObjectId,
        version: u32,
        data: Option<Arc<dyn std::any::Any + Send + Sync + 'static>>,
        handle: WeakHandle,
    }
    impl std::cmp::PartialEq for TestGlobal {
        fn eq(&self, other: &TestGlobal) -> bool {
            self.id == other.id
        }
    }
    impl std::cmp::Eq for TestGlobal {}
    impl super::wayland_server::Resource for TestGlobal {
        type Request = Request;
        type Event = Event;
        #[inline]
        fn interface() -> &'static Interface {
            &super::TEST_GLOBAL_INTERFACE
        }
        #[inline]
        fn id(&self) -> ObjectId {
            self.id.clone()
        }
        #[inline]
        fn version(&self) -> u32 {
            self.version
        }
        #[inline]
        fn data<U: 'static>(&self) -> Option<&U> {
            self.data
                .as_ref()
                .and_then(|arc| (&**arc).downcast_ref::<ResourceData<Self, U>>())
                .map(|data| &data.udata)
        }
        #[inline]
        fn object_data(&self) -> Option<&Arc<dyn std::any::Any + Send + Sync>> {
            self.data.as_ref()
        }
        fn handle(&self) -> &WeakHandle {
            &self.handle
        }
        #[inline]
        fn from_id(conn: &DisplayHandle, id: ObjectId) -> Result<Self, InvalidId> {
            if !same_interface(id.interface(), Self::interface()) && !id.is_null() {
                return Err(InvalidId);
            }
            let version = conn.object_info(id.clone()).map(|info| info.version).unwrap_or(0);
            let data = conn.get_object_data(id.clone()).ok();
            Ok(TestGlobal { id, data, version, handle: conn.backend_handle().downgrade() })
        }
        fn send_event(&self, evt: Self::Event) -> Result<(), InvalidId> {
            let handle = DisplayHandle::from(self.handle.upgrade().ok_or(InvalidId)?);
            handle.send_event(self, evt)
        }
        fn parse_request(
            conn: &DisplayHandle,
            msg: Message<ObjectId>,
        ) -> Result<(Self, Self::Request), DispatchError> {
            let me = Self::from_id(conn, msg.sender_id.clone()).unwrap();
            match msg.opcode {
                0u16 => {
                    if let [Argument::Uint(unsigned_int), Argument::Int(signed_int), Argument::Fixed(fixed_point), Argument::Array(number_array), Argument::Str(some_text), Argument::Fd(file_descriptor)] =
                        &msg.args[..]
                    {
                        Ok((
                            me,
                            Request::ManyArgs {
                                unsigned_int: *unsigned_int,
                                signed_int: *signed_int,
                                fixed_point: (*fixed_point as f64) / 256.,
                                number_array: *number_array.clone(),
                                some_text: String::from_utf8_lossy(
                                    some_text.as_ref().unwrap().as_bytes(),
                                )
                                .into_owned(),
                                file_descriptor: *file_descriptor,
                            },
                        ))
                    } else {
                        Err(DispatchError::BadMessage { msg, interface: Self::interface().name })
                    }
                }
                1u16 => {
                    if let [Argument::NewId(sec)] = &msg.args[..] {
                        Ok((
                            me,
                            Request::GetSecondary {
                                sec: New::wrap(
                                    match <super::secondary::Secondary as Resource>::from_id(
                                        conn,
                                        sec.clone(),
                                    ) {
                                        Ok(p) => p,
                                        Err(_) => {
                                            return Err(DispatchError::BadMessage {
                                                msg,
                                                interface: Self::interface().name,
                                            })
                                        }
                                    },
                                ),
                            },
                        ))
                    } else {
                        Err(DispatchError::BadMessage { msg, interface: Self::interface().name })
                    }
                }
                2u16 => {
                    if let [Argument::NewId(ter)] = &msg.args[..] {
                        Ok((
                            me,
                            Request::GetTertiary {
                                ter: New::wrap(
                                    match <super::tertiary::Tertiary as Resource>::from_id(
                                        conn,
                                        ter.clone(),
                                    ) {
                                        Ok(p) => p,
                                        Err(_) => {
                                            return Err(DispatchError::BadMessage {
                                                msg,
                                                interface: Self::interface().name,
                                            })
                                        }
                                    },
                                ),
                            },
                        ))
                    } else {
                        Err(DispatchError::BadMessage { msg, interface: Self::interface().name })
                    }
                }
                3u16 => {
                    if let [Argument::Object(sec), Argument::Object(ter), Argument::Uint(time)] =
                        &msg.args[..]
                    {
                        Ok((
                            me,
                            Request::Link {
                                sec: match <super::secondary::Secondary as Resource>::from_id(
                                    conn,
                                    sec.clone(),
                                ) {
                                    Ok(p) => p,
                                    Err(_) => {
                                        return Err(DispatchError::BadMessage {
                                            msg,
                                            interface: Self::interface().name,
                                        })
                                    }
                                },
                                ter: if ter.is_null() {
                                    None
                                } else {
                                    Some(
                                        match <super::tertiary::Tertiary as Resource>::from_id(
                                            conn,
                                            ter.clone(),
                                        ) {
                                            Ok(p) => p,
                                            Err(_) => {
                                                return Err(DispatchError::BadMessage {
                                                    msg,
                                                    interface: Self::interface().name,
                                                })
                                            }
                                        },
                                    )
                                },
                                time: *time,
                            },
                        ))
                    } else {
                        Err(DispatchError::BadMessage { msg, interface: Self::interface().name })
                    }
                }
                4u16 => {
                    if let [] = &msg.args[..] {
                        Ok((me, Request::Destroy {}))
                    } else {
                        Err(DispatchError::BadMessage { msg, interface: Self::interface().name })
                    }
                }
                5u16 => {
                    if let [Argument::Object(sec), Argument::Object(ter)] = &msg.args[..] {
                        Ok((
                            me,
                            Request::ReverseLink {
                                sec: if sec.is_null() {
                                    None
                                } else {
                                    Some(
                                        match <super::secondary::Secondary as Resource>::from_id(
                                            conn,
                                            sec.clone(),
                                        ) {
                                            Ok(p) => p,
                                            Err(_) => {
                                                return Err(DispatchError::BadMessage {
                                                    msg,
                                                    interface: Self::interface().name,
                                                })
                                            }
                                        },
                                    )
                                },
                                ter: match <super::tertiary::Tertiary as Resource>::from_id(
                                    conn,
                                    ter.clone(),
                                ) {
                                    Ok(p) => p,
                                    Err(_) => {
                                        return Err(DispatchError::BadMessage {
                                            msg,
                                            interface: Self::interface().name,
                                        })
                                    }
                                },
                            },
                        ))
                    } else {
                        Err(DispatchError::BadMessage { msg, interface: Self::interface().name })
                    }
                }
                6u16 => {
                    if let [Argument::NewId(quad), Argument::Object(sec), Argument::Object(ter)] =
                        &msg.args[..]
                    {
                        Ok((
                            me,
                            Request::NewidAndAllowNull {
                                quad: New::wrap(
                                    match <super::quad::Quad as Resource>::from_id(
                                        conn,
                                        quad.clone(),
                                    ) {
                                        Ok(p) => p,
                                        Err(_) => {
                                            return Err(DispatchError::BadMessage {
                                                msg,
                                                interface: Self::interface().name,
                                            })
                                        }
                                    },
                                ),
                                sec: if sec.is_null() {
                                    None
                                } else {
                                    Some(
                                        match <super::secondary::Secondary as Resource>::from_id(
                                            conn,
                                            sec.clone(),
                                        ) {
                                            Ok(p) => p,
                                            Err(_) => {
                                                return Err(DispatchError::BadMessage {
                                                    msg,
                                                    interface: Self::interface().name,
                                                })
                                            }
                                        },
                                    )
                                },
                                ter: match <super::tertiary::Tertiary as Resource>::from_id(
                                    conn,
                                    ter.clone(),
                                ) {
                                    Ok(p) => p,
                                    Err(_) => {
                                        return Err(DispatchError::BadMessage {
                                            msg,
                                            interface: Self::interface().name,
                                        })
                                    }
                                },
                            },
                        ))
                    } else {
                        Err(DispatchError::BadMessage { msg, interface: Self::interface().name })
                    }
                }
                _ => Err(DispatchError::BadMessage { msg, interface: Self::interface().name }),
            }
        }
        fn write_event(
            &self,
            conn: &DisplayHandle,
            msg: Self::Event,
        ) -> Result<Message<ObjectId>, InvalidId> {
            match msg {
                Event::ManyArgsEvt {
                    unsigned_int,
                    signed_int,
                    fixed_point,
                    number_array,
                    some_text,
                    file_descriptor,
                } => Ok(Message {
                    sender_id: self.id.clone(),
                    opcode: 0u16,
                    args: smallvec::smallvec![
                        Argument::Uint(unsigned_int),
                        Argument::Int(signed_int),
                        Argument::Fixed((fixed_point * 256.) as i32),
                        Argument::Array(Box::new(number_array)),
                        Argument::Str(Some(Box::new(std::ffi::CString::new(some_text).unwrap()))),
                        Argument::Fd(file_descriptor)
                    ],
                }),
                Event::AckSecondary { sec } => Ok(Message {
                    sender_id: self.id.clone(),
                    opcode: 1u16,
                    args: smallvec::smallvec![Argument::Object(Resource::id(&sec))],
                }),
                Event::CycleQuad { new_quad, old_quad } => Ok(Message {
                    sender_id: self.id.clone(),
                    opcode: 2u16,
                    args: smallvec::smallvec![
                        Argument::NewId(Resource::id(&new_quad)),
                        if let Some(obj) = old_quad {
                            Argument::Object(Resource::id(&obj))
                        } else {
                            Argument::Object(ObjectId::null())
                        }
                    ],
                }),
            }
        }
        fn __set_object_data(
            &mut self,
            odata: std::sync::Arc<dyn std::any::Any + Send + Sync + 'static>,
        ) {
            self.data = Some(odata);
        }
    }
    impl TestGlobal {
        #[doc = "an event with every possible non-object arg"]
        #[allow(clippy::too_many_arguments)]
        pub fn many_args_evt(
            &self,
            unsigned_int: u32,
            signed_int: i32,
            fixed_point: f64,
            number_array: Vec<u8>,
            some_text: String,
            file_descriptor: ::std::os::unix::io::RawFd,
        ) {
            let _ = self.send_event(
                Event::ManyArgsEvt {
                    unsigned_int,
                    signed_int,
                    fixed_point,
                    number_array,
                    some_text,
                    file_descriptor,
                },
            );
        }
        #[doc = "acking the creation of a secondary"]
        #[allow(clippy::too_many_arguments)]
        pub fn ack_secondary(
            &self,
            sec: &super::secondary::Secondary,
        ) {
            let _ = self.send_event(Event::AckSecondary { sec: sec.clone() });
        }
        #[doc = "create a new quad optionally replacing a previous one"]
        #[allow(clippy::too_many_arguments)]
        pub fn cycle_quad(
            &self,
            new_quad: &super::quad::Quad,
            old_quad: Option<&super::quad::Quad>,
        ) {
            let _ = self.send_event(Event::CycleQuad { new_quad: new_quad.clone(), old_quad: old_quad.cloned() });
        }
    }
}
pub mod secondary {
    use super::wayland_server::{
        backend::{
            protocol::{same_interface, Argument, Interface, Message, WEnum},
            smallvec, InvalidId, ObjectData, ObjectId, WeakHandle,
        },
        Dispatch, DispatchError, DisplayHandle, New, Resource, ResourceData,
    };
    use std::sync::Arc;
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_DESTROY_SINCE: u32 = 2u32;
    #[doc = r" The wire opcode for this request"]
    pub const REQ_DESTROY_OPCODE: u16 = 0u16;
    #[derive(Debug)]
    #[non_exhaustive]
    pub enum Request {
        #[doc = "This is a destructor, once received this object cannot be used any longer.\nOnly available since version 2 of the interface"]
        Destroy,
    }
    #[derive(Debug)]
    #[non_exhaustive]
    pub enum Event {}
    #[doc = "secondary\n\nSee also the [Request] enum for this interface."]
    #[derive(Debug, Clone)]
    pub struct Secondary {
        id: ObjectId,
        version: u32,
        data: Option<Arc<dyn std::any::Any + Send + Sync + 'static>>,
        handle: WeakHandle,
    }
    impl std::cmp::PartialEq for Secondary {
        fn eq(&self, other: &Secondary) -> bool {
            self.id == other.id
        }
    }
    impl std::cmp::Eq for Secondary {}
    impl super::wayland_server::Resource for Secondary {
        type Request = Request;
        type Event = Event;
        #[inline]
        fn interface() -> &'static Interface {
            &super::SECONDARY_INTERFACE
        }
        #[inline]
        fn id(&self) -> ObjectId {
            self.id.clone()
        }
        #[inline]
        fn version(&self) -> u32 {
            self.version
        }
        #[inline]
        fn data<U: 'static>(&self) -> Option<&U> {
            self.data
                .as_ref()
                .and_then(|arc| (&**arc).downcast_ref::<ResourceData<Self, U>>())
                .map(|data| &data.udata)
        }
        #[inline]
        fn object_data(&self) -> Option<&Arc<dyn std::any::Any + Send + Sync>> {
            self.data.as_ref()
        }
        fn handle(&self) -> &WeakHandle {
            &self.handle
        }
        #[inline]
        fn from_id(conn: &DisplayHandle, id: ObjectId) -> Result<Self, InvalidId> {
            if !same_interface(id.interface(), Self::interface()) && !id.is_null() {
                return Err(InvalidId);
            }
            let version = conn.object_info(id.clone()).map(|info| info.version).unwrap_or(0);
            let data = conn.get_object_data(id.clone()).ok();
            Ok(Secondary { id, data, version, handle: conn.backend_handle().downgrade() })
        }
        fn send_event(&self, evt: Self::Event) -> Result<(), InvalidId> {
            let handle = DisplayHandle::from(self.handle.upgrade().ok_or(InvalidId)?);
            handle.send_event(self, evt)
        }
        fn parse_request(
            conn: &DisplayHandle,
            msg: Message<ObjectId>,
        ) -> Result<(Self, Self::Request), DispatchError> {
            let me = Self::from_id(conn, msg.sender_id.clone()).unwrap();
            match msg.opcode {
                0u16 => {
                    if let [] = &msg.args[..] {
                        Ok((me, Request::Destroy {}))
                    } else {
                        Err(DispatchError::BadMessage { msg, interface: Self::interface().name })
                    }
                }
                _ => Err(DispatchError::BadMessage { msg, interface: Self::interface().name }),
            }
        }
        fn write_event(
            &self,
            conn: &DisplayHandle,
            msg: Self::Event,
        ) -> Result<Message<ObjectId>, InvalidId> {
            match msg {}
        }
        fn __set_object_data(
            &mut self,
            odata: std::sync::Arc<dyn std::any::Any + Send + Sync + 'static>,
        ) {
            self.data = Some(odata);
        }
    }
    impl Secondary {}
}
pub mod tertiary {
    use super::wayland_server::{
        backend::{
            protocol::{same_interface, Argument, Interface, Message, WEnum},
            smallvec, InvalidId, ObjectData, ObjectId, WeakHandle,
        },
        Dispatch, DispatchError, DisplayHandle, New, Resource, ResourceData,
    };
    use std::sync::Arc;
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_DESTROY_SINCE: u32 = 3u32;
    #[doc = r" The wire opcode for this request"]
    pub const REQ_DESTROY_OPCODE: u16 = 0u16;
    #[derive(Debug)]
    #[non_exhaustive]
    pub enum Request {
        #[doc = "This is a destructor, once received this object cannot be used any longer.\nOnly available since version 3 of the interface"]
        Destroy,
    }
    #[derive(Debug)]
    #[non_exhaustive]
    pub enum Event {}
    #[doc = "tertiary\n\nSee also the [Request] enum for this interface."]
    #[derive(Debug, Clone)]
    pub struct Tertiary {
        id: ObjectId,
        version: u32,
        data: Option<Arc<dyn std::any::Any + Send + Sync + 'static>>,
        handle: WeakHandle,
    }
    impl std::cmp::PartialEq for Tertiary {
        fn eq(&self, other: &Tertiary) -> bool {
            self.id == other.id
        }
    }
    impl std::cmp::Eq for Tertiary {}
    impl super::wayland_server::Resource for Tertiary {
        type Request = Request;
        type Event = Event;
        #[inline]
        fn interface() -> &'static Interface {
            &super::TERTIARY_INTERFACE
        }
        #[inline]
        fn id(&self) -> ObjectId {
            self.id.clone()
        }
        #[inline]
        fn version(&self) -> u32 {
            self.version
        }
        #[inline]
        fn data<U: 'static>(&self) -> Option<&U> {
            self.data
                .as_ref()
                .and_then(|arc| (&**arc).downcast_ref::<ResourceData<Self, U>>())
                .map(|data| &data.udata)
        }
        #[inline]
        fn object_data(&self) -> Option<&Arc<dyn std::any::Any + Send + Sync>> {
            self.data.as_ref()
        }
        fn handle(&self) -> &WeakHandle {
            &self.handle
        }
        #[inline]
        fn from_id(conn: &DisplayHandle, id: ObjectId) -> Result<Self, InvalidId> {
            if !same_interface(id.interface(), Self::interface()) && !id.is_null() {
                return Err(InvalidId);
            }
            let version = conn.object_info(id.clone()).map(|info| info.version).unwrap_or(0);
            let data = conn.get_object_data(id.clone()).ok();
            Ok(Tertiary { id, data, version, handle: conn.backend_handle().downgrade() })
        }
        fn send_event(&self, evt: Self::Event) -> Result<(), InvalidId> {
            let handle = DisplayHandle::from(self.handle.upgrade().ok_or(InvalidId)?);
            handle.send_event(self, evt)
        }
        fn parse_request(
            conn: &DisplayHandle,
            msg: Message<ObjectId>,
        ) -> Result<(Self, Self::Request), DispatchError> {
            let me = Self::from_id(conn, msg.sender_id.clone()).unwrap();
            match msg.opcode {
                0u16 => {
                    if let [] = &msg.args[..] {
                        Ok((me, Request::Destroy {}))
                    } else {
                        Err(DispatchError::BadMessage { msg, interface: Self::interface().name })
                    }
                }
                _ => Err(DispatchError::BadMessage { msg, interface: Self::interface().name }),
            }
        }
        fn write_event(
            &self,
            conn: &DisplayHandle,
            msg: Self::Event,
        ) -> Result<Message<ObjectId>, InvalidId> {
            match msg {}
        }
        fn __set_object_data(
            &mut self,
            odata: std::sync::Arc<dyn std::any::Any + Send + Sync + 'static>,
        ) {
            self.data = Some(odata);
        }
    }
    impl Tertiary {}
}
pub mod quad {
    use super::wayland_server::{
        backend::{
            protocol::{same_interface, Argument, Interface, Message, WEnum},
            smallvec, InvalidId, ObjectData, ObjectId, WeakHandle,
        },
        Dispatch, DispatchError, DisplayHandle, New, Resource, ResourceData,
    };
    use std::sync::Arc;
    #[doc = r" The minimal object version supporting this request"]
    pub const REQ_DESTROY_SINCE: u32 = 3u32;
    #[doc = r" The wire opcode for this request"]
    pub const REQ_DESTROY_OPCODE: u16 = 0u16;
    #[derive(Debug)]
    #[non_exhaustive]
    pub enum Request {
        #[doc = "This is a destructor, once received this object cannot be used any longer.\nOnly available since version 3 of the interface"]
        Destroy,
    }
    #[derive(Debug)]
    #[non_exhaustive]
    pub enum Event {}
    #[doc = "quad\n\nSee also the [Request] enum for this interface."]
    #[derive(Debug, Clone)]
    pub struct Quad {
        id: ObjectId,
        version: u32,
        data: Option<Arc<dyn std::any::Any + Send + Sync + 'static>>,
        handle: WeakHandle,
    }
    impl std::cmp::PartialEq for Quad {
        fn eq(&self, other: &Quad) -> bool {
            self.id == other.id
        }
    }
    impl std::cmp::Eq for Quad {}
    impl super::wayland_server::Resource for Quad {
        type Request = Request;
        type Event = Event;
        #[inline]
        fn interface() -> &'static Interface {
            &super::QUAD_INTERFACE
        }
        #[inline]
        fn id(&self) -> ObjectId {
            self.id.clone()
        }
        #[inline]
        fn version(&self) -> u32 {
            self.version
        }
        #[inline]
        fn data<U: 'static>(&self) -> Option<&U> {
            self.data
                .as_ref()
                .and_then(|arc| (&**arc).downcast_ref::<ResourceData<Self, U>>())
                .map(|data| &data.udata)
        }
        #[inline]
        fn object_data(&self) -> Option<&Arc<dyn std::any::Any + Send + Sync>> {
            self.data.as_ref()
        }
        fn handle(&self) -> &WeakHandle {
            &self.handle
        }
        #[inline]
        fn from_id(conn: &DisplayHandle, id: ObjectId) -> Result<Self, InvalidId> {
            if !same_interface(id.interface(), Self::interface()) && !id.is_null() {
                return Err(InvalidId);
            }
            let version = conn.object_info(id.clone()).map(|info| info.version).unwrap_or(0);
            let data = conn.get_object_data(id.clone()).ok();
            Ok(Quad { id, data, version, handle: conn.backend_handle().downgrade() })
        }
        fn send_event(&self, evt: Self::Event) -> Result<(), InvalidId> {
            let handle = DisplayHandle::from(self.handle.upgrade().ok_or(InvalidId)?);
            handle.send_event(self, evt)
        }
        fn parse_request(
            conn: &DisplayHandle,
            msg: Message<ObjectId>,
        ) -> Result<(Self, Self::Request), DispatchError> {
            let me = Self::from_id(conn, msg.sender_id.clone()).unwrap();
            match msg.opcode {
                0u16 => {
                    if let [] = &msg.args[..] {
                        Ok((me, Request::Destroy {}))
                    } else {
                        Err(DispatchError::BadMessage { msg, interface: Self::interface().name })
                    }
                }
                _ => Err(DispatchError::BadMessage { msg, interface: Self::interface().name }),
            }
        }
        fn write_event(
            &self,
            conn: &DisplayHandle,
            msg: Self::Event,
        ) -> Result<Message<ObjectId>, InvalidId> {
            match msg {}
        }
        fn __set_object_data(
            &mut self,
            odata: std::sync::Arc<dyn std::any::Any + Send + Sync + 'static>,
        ) {
            self.data = Some(odata);
        }
    }
    impl Quad {}
}
