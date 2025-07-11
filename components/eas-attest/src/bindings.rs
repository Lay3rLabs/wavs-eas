pub type TriggerAction = wavs::worker::layer_types::TriggerAction;
pub type WasmResponse = wavs::worker::layer_types::WasmResponse;
#[doc(hidden)]
#[allow(non_snake_case)]
pub unsafe fn _export_run_cabi<T: Guest>(arg0: *mut u8) -> *mut u8 {
    #[cfg(target_arch = "wasm32")]
    _rt::run_ctors_once();
    let l0 = *arg0.add(0).cast::<*mut u8>();
    let l1 = *arg0.add(4).cast::<usize>();
    let len2 = l1;
    let bytes2 = _rt::Vec::from_raw_parts(l0.cast(), len2, len2);
    let l3 = *arg0.add(8).cast::<*mut u8>();
    let l4 = *arg0.add(12).cast::<usize>();
    let len5 = l4;
    let bytes5 = _rt::Vec::from_raw_parts(l3.cast(), len5, len5);
    let l6 = i32::from(*arg0.add(16).cast::<u8>());
    use wavs::worker::layer_types::TriggerSource as V41;
    let v41 = match l6 {
        0 => {
            let e41 = {
                let l7 = *arg0.add(24).cast::<*mut u8>();
                let l8 = *arg0.add(28).cast::<usize>();
                let len9 = l8;
                let l10 = *arg0.add(32).cast::<*mut u8>();
                let l11 = *arg0.add(36).cast::<usize>();
                let len12 = l11;
                let bytes12 = _rt::Vec::from_raw_parts(l10.cast(), len12, len12);
                let l13 = *arg0.add(40).cast::<*mut u8>();
                let l14 = *arg0.add(44).cast::<usize>();
                let len15 = l14;
                wavs::worker::layer_types::TriggerSourceEvmContractEvent {
                    address: wavs::worker::layer_types::EvmAddress {
                        raw_bytes: _rt::Vec::from_raw_parts(l7.cast(), len9, len9),
                    },
                    chain_name: _rt::string_lift(bytes12),
                    event_hash: _rt::Vec::from_raw_parts(l13.cast(), len15, len15),
                }
            };
            V41::EvmContractEvent(e41)
        }
        1 => {
            let e41 = {
                let l16 = *arg0.add(24).cast::<*mut u8>();
                let l17 = *arg0.add(28).cast::<usize>();
                let len18 = l17;
                let bytes18 = _rt::Vec::from_raw_parts(l16.cast(), len18, len18);
                let l19 = *arg0.add(32).cast::<i32>();
                let l20 = *arg0.add(36).cast::<*mut u8>();
                let l21 = *arg0.add(40).cast::<usize>();
                let len22 = l21;
                let bytes22 = _rt::Vec::from_raw_parts(l20.cast(), len22, len22);
                let l23 = *arg0.add(44).cast::<*mut u8>();
                let l24 = *arg0.add(48).cast::<usize>();
                let len25 = l24;
                let bytes25 = _rt::Vec::from_raw_parts(l23.cast(), len25, len25);
                wavs::worker::layer_types::TriggerSourceCosmosContractEvent {
                    address: wavs::worker::layer_types::CosmosAddress {
                        bech32_addr: _rt::string_lift(bytes18),
                        prefix_len: l19 as u32,
                    },
                    chain_name: _rt::string_lift(bytes22),
                    event_type: _rt::string_lift(bytes25),
                }
            };
            V41::CosmosContractEvent(e41)
        }
        2 => {
            let e41 = {
                let l26 = *arg0.add(24).cast::<*mut u8>();
                let l27 = *arg0.add(28).cast::<usize>();
                let len28 = l27;
                let bytes28 = _rt::Vec::from_raw_parts(l26.cast(), len28, len28);
                let l29 = *arg0.add(32).cast::<i32>();
                let l30 = i32::from(*arg0.add(40).cast::<u8>());
                let l32 = i32::from(*arg0.add(56).cast::<u8>());
                wavs::worker::layer_types::BlockIntervalSource {
                    chain_name: _rt::string_lift(bytes28),
                    n_blocks: l29 as u32,
                    start_block: match l30 {
                        0 => None,
                        1 => {
                            let e = {
                                let l31 = *arg0.add(48).cast::<i64>();
                                l31 as u64
                            };
                            Some(e)
                        }
                        _ => _rt::invalid_enum_discriminant(),
                    },
                    end_block: match l32 {
                        0 => None,
                        1 => {
                            let e = {
                                let l33 = *arg0.add(64).cast::<i64>();
                                l33 as u64
                            };
                            Some(e)
                        }
                        _ => _rt::invalid_enum_discriminant(),
                    },
                }
            };
            V41::BlockInterval(e41)
        }
        3 => {
            let e41 = {
                let l34 = *arg0.add(24).cast::<*mut u8>();
                let l35 = *arg0.add(28).cast::<usize>();
                let len36 = l35;
                let bytes36 = _rt::Vec::from_raw_parts(l34.cast(), len36, len36);
                let l37 = i32::from(*arg0.add(32).cast::<u8>());
                let l39 = i32::from(*arg0.add(48).cast::<u8>());
                wavs::worker::layer_types::TriggerSourceCron {
                    schedule: _rt::string_lift(bytes36),
                    start_time: match l37 {
                        0 => None,
                        1 => {
                            let e = {
                                let l38 = *arg0.add(40).cast::<i64>();
                                wavs::worker::layer_types::Timestamp { nanos: l38 as u64 }
                            };
                            Some(e)
                        }
                        _ => _rt::invalid_enum_discriminant(),
                    },
                    end_time: match l39 {
                        0 => None,
                        1 => {
                            let e = {
                                let l40 = *arg0.add(56).cast::<i64>();
                                wavs::worker::layer_types::Timestamp { nanos: l40 as u64 }
                            };
                            Some(e)
                        }
                        _ => _rt::invalid_enum_discriminant(),
                    },
                }
            };
            V41::Cron(e41)
        }
        n => {
            debug_assert_eq!(n, 4, "invalid enum discriminant");
            V41::Manual
        }
    };
    let l42 = i32::from(*arg0.add(72).cast::<u8>());
    use wavs::worker::layer_types::TriggerData as V87;
    let v87 = match l42 {
        0 => {
            let e87 = {
                let l43 = *arg0.add(80).cast::<*mut u8>();
                let l44 = *arg0.add(84).cast::<usize>();
                let len45 = l44;
                let l46 = *arg0.add(88).cast::<*mut u8>();
                let l47 = *arg0.add(92).cast::<usize>();
                let len48 = l47;
                let bytes48 = _rt::Vec::from_raw_parts(l46.cast(), len48, len48);
                let l49 = *arg0.add(96).cast::<*mut u8>();
                let l50 = *arg0.add(100).cast::<usize>();
                let base54 = l49;
                let len54 = l50;
                let mut result54 = _rt::Vec::with_capacity(len54);
                for i in 0..len54 {
                    let base = base54.add(i * 8);
                    let e54 = {
                        let l51 = *base.add(0).cast::<*mut u8>();
                        let l52 = *base.add(4).cast::<usize>();
                        let len53 = l52;
                        _rt::Vec::from_raw_parts(l51.cast(), len53, len53)
                    };
                    result54.push(e54);
                }
                _rt::cabi_dealloc(base54, len54 * 8, 4);
                let l55 = *arg0.add(104).cast::<*mut u8>();
                let l56 = *arg0.add(108).cast::<usize>();
                let len57 = l56;
                let l58 = *arg0.add(112).cast::<i64>();
                wavs::worker::layer_types::TriggerDataEvmContractEvent {
                    contract_address: wavs::worker::layer_types::EvmAddress {
                        raw_bytes: _rt::Vec::from_raw_parts(l43.cast(), len45, len45),
                    },
                    chain_name: _rt::string_lift(bytes48),
                    log: wavs::worker::layer_types::EvmEventLogData {
                        topics: result54,
                        data: _rt::Vec::from_raw_parts(l55.cast(), len57, len57),
                    },
                    block_height: l58 as u64,
                }
            };
            V87::EvmContractEvent(e87)
        }
        1 => {
            let e87 = {
                let l59 = *arg0.add(80).cast::<*mut u8>();
                let l60 = *arg0.add(84).cast::<usize>();
                let len61 = l60;
                let bytes61 = _rt::Vec::from_raw_parts(l59.cast(), len61, len61);
                let l62 = *arg0.add(88).cast::<i32>();
                let l63 = *arg0.add(92).cast::<*mut u8>();
                let l64 = *arg0.add(96).cast::<usize>();
                let len65 = l64;
                let bytes65 = _rt::Vec::from_raw_parts(l63.cast(), len65, len65);
                let l66 = *arg0.add(100).cast::<*mut u8>();
                let l67 = *arg0.add(104).cast::<usize>();
                let len68 = l67;
                let bytes68 = _rt::Vec::from_raw_parts(l66.cast(), len68, len68);
                let l69 = *arg0.add(108).cast::<*mut u8>();
                let l70 = *arg0.add(112).cast::<usize>();
                let base77 = l69;
                let len77 = l70;
                let mut result77 = _rt::Vec::with_capacity(len77);
                for i in 0..len77 {
                    let base = base77.add(i * 16);
                    let e77 = {
                        let l71 = *base.add(0).cast::<*mut u8>();
                        let l72 = *base.add(4).cast::<usize>();
                        let len73 = l72;
                        let bytes73 = _rt::Vec::from_raw_parts(l71.cast(), len73, len73);
                        let l74 = *base.add(8).cast::<*mut u8>();
                        let l75 = *base.add(12).cast::<usize>();
                        let len76 = l75;
                        let bytes76 = _rt::Vec::from_raw_parts(l74.cast(), len76, len76);
                        (_rt::string_lift(bytes73), _rt::string_lift(bytes76))
                    };
                    result77.push(e77);
                }
                _rt::cabi_dealloc(base77, len77 * 16, 4);
                let l78 = *arg0.add(120).cast::<i64>();
                wavs::worker::layer_types::TriggerDataCosmosContractEvent {
                    contract_address: wavs::worker::layer_types::CosmosAddress {
                        bech32_addr: _rt::string_lift(bytes61),
                        prefix_len: l62 as u32,
                    },
                    chain_name: _rt::string_lift(bytes65),
                    event: wavs::worker::layer_types::CosmosEvent {
                        ty: _rt::string_lift(bytes68),
                        attributes: result77,
                    },
                    block_height: l78 as u64,
                }
            };
            V87::CosmosContractEvent(e87)
        }
        2 => {
            let e87 = {
                let l79 = *arg0.add(80).cast::<*mut u8>();
                let l80 = *arg0.add(84).cast::<usize>();
                let len81 = l80;
                let bytes81 = _rt::Vec::from_raw_parts(l79.cast(), len81, len81);
                let l82 = *arg0.add(88).cast::<i64>();
                wavs::worker::layer_types::BlockIntervalData {
                    chain_name: _rt::string_lift(bytes81),
                    block_height: l82 as u64,
                }
            };
            V87::BlockInterval(e87)
        }
        3 => {
            let e87 = {
                let l83 = *arg0.add(80).cast::<i64>();
                wavs::worker::layer_types::TriggerDataCron {
                    trigger_time: wavs::worker::layer_types::Timestamp { nanos: l83 as u64 },
                }
            };
            V87::Cron(e87)
        }
        n => {
            debug_assert_eq!(n, 4, "invalid enum discriminant");
            let e87 = {
                let l84 = *arg0.add(80).cast::<*mut u8>();
                let l85 = *arg0.add(84).cast::<usize>();
                let len86 = l85;
                _rt::Vec::from_raw_parts(l84.cast(), len86, len86)
            };
            V87::Raw(e87)
        }
    };
    let result88 = T::run(wavs::worker::layer_types::TriggerAction {
        config: wavs::worker::layer_types::TriggerConfig {
            service_id: _rt::string_lift(bytes2),
            workflow_id: _rt::string_lift(bytes5),
            trigger_source: v41,
        },
        data: v87,
    });
    _rt::cabi_dealloc(arg0, 128, 8);
    let ptr89 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
    match result88 {
        Ok(e) => {
            *ptr89.add(0).cast::<u8>() = (0i32) as u8;
            match e {
                Some(e) => {
                    *ptr89.add(8).cast::<u8>() = (1i32) as u8;
                    let wavs::worker::layer_types::WasmResponse {
                        payload: payload90,
                        ordering: ordering90,
                    } = e;
                    let vec91 = (payload90).into_boxed_slice();
                    let ptr91 = vec91.as_ptr().cast::<u8>();
                    let len91 = vec91.len();
                    ::core::mem::forget(vec91);
                    *ptr89.add(20).cast::<usize>() = len91;
                    *ptr89.add(16).cast::<*mut u8>() = ptr91.cast_mut();
                    match ordering90 {
                        Some(e) => {
                            *ptr89.add(24).cast::<u8>() = (1i32) as u8;
                            *ptr89.add(32).cast::<i64>() = _rt::as_i64(e);
                        }
                        None => {
                            *ptr89.add(24).cast::<u8>() = (0i32) as u8;
                        }
                    };
                }
                None => {
                    *ptr89.add(8).cast::<u8>() = (0i32) as u8;
                }
            };
        }
        Err(e) => {
            *ptr89.add(0).cast::<u8>() = (1i32) as u8;
            let vec92 = (e.into_bytes()).into_boxed_slice();
            let ptr92 = vec92.as_ptr().cast::<u8>();
            let len92 = vec92.len();
            ::core::mem::forget(vec92);
            *ptr89.add(12).cast::<usize>() = len92;
            *ptr89.add(8).cast::<*mut u8>() = ptr92.cast_mut();
        }
    };
    ptr89
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub unsafe fn __post_return_run<T: Guest>(arg0: *mut u8) {
    let l0 = i32::from(*arg0.add(0).cast::<u8>());
    match l0 {
        0 => {
            let l1 = i32::from(*arg0.add(8).cast::<u8>());
            match l1 {
                0 => {}
                _ => {
                    let l2 = *arg0.add(16).cast::<*mut u8>();
                    let l3 = *arg0.add(20).cast::<usize>();
                    let base4 = l2;
                    let len4 = l3;
                    _rt::cabi_dealloc(base4, len4 * 1, 1);
                }
            }
        }
        _ => {
            let l5 = *arg0.add(8).cast::<*mut u8>();
            let l6 = *arg0.add(12).cast::<usize>();
            _rt::cabi_dealloc(l5, l6, 1);
        }
    }
}
pub trait Guest {
    fn run(trigger_action: TriggerAction) -> Result<Option<WasmResponse>, _rt::String>;
}
#[doc(hidden)]
macro_rules! __export_world_layer_trigger_world_cabi {
    ($ty:ident with_types_in $($path_to_types:tt)*) => {
        const _ : () = { #[export_name = "run"] unsafe extern "C" fn export_run(arg0 : *
        mut u8,) -> * mut u8 { $($path_to_types)*:: _export_run_cabi::<$ty > (arg0) }
        #[export_name = "cabi_post_run"] unsafe extern "C" fn _post_return_run(arg0 : *
        mut u8,) { $($path_to_types)*:: __post_return_run::<$ty > (arg0) } };
    };
}
#[doc(hidden)]
pub(crate) use __export_world_layer_trigger_world_cabi;
#[repr(align(8))]
struct _RetArea([::core::mem::MaybeUninit<u8>; 40]);
static mut _RET_AREA: _RetArea = _RetArea([::core::mem::MaybeUninit::uninit(); 40]);
#[allow(dead_code)]
pub mod wasi {
    #[allow(dead_code)]
    pub mod cli {
        #[allow(dead_code, clippy::all)]
        pub mod environment {
            #[used]
            #[doc(hidden)]
            static __FORCE_SECTION_REF: fn() =
                super::super::super::__link_custom_section_describing_imports;
            use super::super::super::_rt;
            #[allow(unused_unsafe, clippy::all)]
            pub fn get_environment() -> _rt::Vec<(_rt::String, _rt::String)> {
                unsafe {
                    #[repr(align(4))]
                    struct RetArea([::core::mem::MaybeUninit<u8>; 8]);
                    let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 8]);
                    let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                    #[cfg(target_arch = "wasm32")]
                    #[link(wasm_import_module = "wasi:cli/environment@0.2.0")]
                    extern "C" {
                        #[link_name = "get-environment"]
                        fn wit_import(_: *mut u8);
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import(_: *mut u8) {
                        unreachable!()
                    }
                    wit_import(ptr0);
                    let l1 = *ptr0.add(0).cast::<*mut u8>();
                    let l2 = *ptr0.add(4).cast::<usize>();
                    let base9 = l1;
                    let len9 = l2;
                    let mut result9 = _rt::Vec::with_capacity(len9);
                    for i in 0..len9 {
                        let base = base9.add(i * 16);
                        let e9 = {
                            let l3 = *base.add(0).cast::<*mut u8>();
                            let l4 = *base.add(4).cast::<usize>();
                            let len5 = l4;
                            let bytes5 = _rt::Vec::from_raw_parts(l3.cast(), len5, len5);
                            let l6 = *base.add(8).cast::<*mut u8>();
                            let l7 = *base.add(12).cast::<usize>();
                            let len8 = l7;
                            let bytes8 = _rt::Vec::from_raw_parts(l6.cast(), len8, len8);
                            (_rt::string_lift(bytes5), _rt::string_lift(bytes8))
                        };
                        result9.push(e9);
                    }
                    _rt::cabi_dealloc(base9, len9 * 16, 4);
                    result9
                }
            }
            #[allow(unused_unsafe, clippy::all)]
            pub fn get_arguments() -> _rt::Vec<_rt::String> {
                unsafe {
                    #[repr(align(4))]
                    struct RetArea([::core::mem::MaybeUninit<u8>; 8]);
                    let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 8]);
                    let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                    #[cfg(target_arch = "wasm32")]
                    #[link(wasm_import_module = "wasi:cli/environment@0.2.0")]
                    extern "C" {
                        #[link_name = "get-arguments"]
                        fn wit_import(_: *mut u8);
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import(_: *mut u8) {
                        unreachable!()
                    }
                    wit_import(ptr0);
                    let l1 = *ptr0.add(0).cast::<*mut u8>();
                    let l2 = *ptr0.add(4).cast::<usize>();
                    let base6 = l1;
                    let len6 = l2;
                    let mut result6 = _rt::Vec::with_capacity(len6);
                    for i in 0..len6 {
                        let base = base6.add(i * 8);
                        let e6 = {
                            let l3 = *base.add(0).cast::<*mut u8>();
                            let l4 = *base.add(4).cast::<usize>();
                            let len5 = l4;
                            let bytes5 = _rt::Vec::from_raw_parts(l3.cast(), len5, len5);
                            _rt::string_lift(bytes5)
                        };
                        result6.push(e6);
                    }
                    _rt::cabi_dealloc(base6, len6 * 8, 4);
                    result6
                }
            }
            #[allow(unused_unsafe, clippy::all)]
            pub fn initial_cwd() -> Option<_rt::String> {
                unsafe {
                    #[repr(align(4))]
                    struct RetArea([::core::mem::MaybeUninit<u8>; 12]);
                    let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 12]);
                    let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                    #[cfg(target_arch = "wasm32")]
                    #[link(wasm_import_module = "wasi:cli/environment@0.2.0")]
                    extern "C" {
                        #[link_name = "initial-cwd"]
                        fn wit_import(_: *mut u8);
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import(_: *mut u8) {
                        unreachable!()
                    }
                    wit_import(ptr0);
                    let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                    match l1 {
                        0 => None,
                        1 => {
                            let e = {
                                let l2 = *ptr0.add(4).cast::<*mut u8>();
                                let l3 = *ptr0.add(8).cast::<usize>();
                                let len4 = l3;
                                let bytes4 = _rt::Vec::from_raw_parts(l2.cast(), len4, len4);
                                _rt::string_lift(bytes4)
                            };
                            Some(e)
                        }
                        _ => _rt::invalid_enum_discriminant(),
                    }
                }
            }
        }
        #[allow(dead_code, clippy::all)]
        pub mod exit {
            #[used]
            #[doc(hidden)]
            static __FORCE_SECTION_REF: fn() =
                super::super::super::__link_custom_section_describing_imports;
            #[allow(unused_unsafe, clippy::all)]
            pub fn exit(status: Result<(), ()>) {
                unsafe {
                    let result0 = match status {
                        Ok(_) => 0i32,
                        Err(_) => 1i32,
                    };
                    #[cfg(target_arch = "wasm32")]
                    #[link(wasm_import_module = "wasi:cli/exit@0.2.0")]
                    extern "C" {
                        #[link_name = "exit"]
                        fn wit_import(_: i32);
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import(_: i32) {
                        unreachable!()
                    }
                    wit_import(result0);
                }
            }
        }
        #[allow(dead_code, clippy::all)]
        pub mod stdin {
            #[used]
            #[doc(hidden)]
            static __FORCE_SECTION_REF: fn() =
                super::super::super::__link_custom_section_describing_imports;
            pub type InputStream = super::super::super::wasi::io::streams::InputStream;
            #[allow(unused_unsafe, clippy::all)]
            pub fn get_stdin() -> InputStream {
                unsafe {
                    #[cfg(target_arch = "wasm32")]
                    #[link(wasm_import_module = "wasi:cli/stdin@0.2.0")]
                    extern "C" {
                        #[link_name = "get-stdin"]
                        fn wit_import() -> i32;
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import() -> i32 {
                        unreachable!()
                    }
                    let ret = wit_import();
                    super::super::super::wasi::io::streams::InputStream::from_handle(ret as u32)
                }
            }
        }
        #[allow(dead_code, clippy::all)]
        pub mod stdout {
            #[used]
            #[doc(hidden)]
            static __FORCE_SECTION_REF: fn() =
                super::super::super::__link_custom_section_describing_imports;
            pub type OutputStream = super::super::super::wasi::io::streams::OutputStream;
            #[allow(unused_unsafe, clippy::all)]
            pub fn get_stdout() -> OutputStream {
                unsafe {
                    #[cfg(target_arch = "wasm32")]
                    #[link(wasm_import_module = "wasi:cli/stdout@0.2.0")]
                    extern "C" {
                        #[link_name = "get-stdout"]
                        fn wit_import() -> i32;
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import() -> i32 {
                        unreachable!()
                    }
                    let ret = wit_import();
                    super::super::super::wasi::io::streams::OutputStream::from_handle(ret as u32)
                }
            }
        }
        #[allow(dead_code, clippy::all)]
        pub mod stderr {
            #[used]
            #[doc(hidden)]
            static __FORCE_SECTION_REF: fn() =
                super::super::super::__link_custom_section_describing_imports;
            pub type OutputStream = super::super::super::wasi::io::streams::OutputStream;
            #[allow(unused_unsafe, clippy::all)]
            pub fn get_stderr() -> OutputStream {
                unsafe {
                    #[cfg(target_arch = "wasm32")]
                    #[link(wasm_import_module = "wasi:cli/stderr@0.2.0")]
                    extern "C" {
                        #[link_name = "get-stderr"]
                        fn wit_import() -> i32;
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import() -> i32 {
                        unreachable!()
                    }
                    let ret = wit_import();
                    super::super::super::wasi::io::streams::OutputStream::from_handle(ret as u32)
                }
            }
        }
        #[allow(dead_code, clippy::all)]
        pub mod terminal_input {
            #[used]
            #[doc(hidden)]
            static __FORCE_SECTION_REF: fn() =
                super::super::super::__link_custom_section_describing_imports;
            use super::super::super::_rt;
            #[derive(Debug)]
            #[repr(transparent)]
            pub struct TerminalInput {
                handle: _rt::Resource<TerminalInput>,
            }
            impl TerminalInput {
                #[doc(hidden)]
                pub unsafe fn from_handle(handle: u32) -> Self {
                    Self { handle: _rt::Resource::from_handle(handle) }
                }
                #[doc(hidden)]
                pub fn take_handle(&self) -> u32 {
                    _rt::Resource::take_handle(&self.handle)
                }
                #[doc(hidden)]
                pub fn handle(&self) -> u32 {
                    _rt::Resource::handle(&self.handle)
                }
            }
            unsafe impl _rt::WasmResource for TerminalInput {
                #[inline]
                unsafe fn drop(_handle: u32) {
                    #[cfg(not(target_arch = "wasm32"))]
                    unreachable!();
                    #[cfg(target_arch = "wasm32")]
                    {
                        #[link(wasm_import_module = "wasi:cli/terminal-input@0.2.0")]
                        extern "C" {
                            #[link_name = "[resource-drop]terminal-input"]
                            fn drop(_: u32);
                        }
                        drop(_handle);
                    }
                }
            }
        }
        #[allow(dead_code, clippy::all)]
        pub mod terminal_output {
            #[used]
            #[doc(hidden)]
            static __FORCE_SECTION_REF: fn() =
                super::super::super::__link_custom_section_describing_imports;
            use super::super::super::_rt;
            #[derive(Debug)]
            #[repr(transparent)]
            pub struct TerminalOutput {
                handle: _rt::Resource<TerminalOutput>,
            }
            impl TerminalOutput {
                #[doc(hidden)]
                pub unsafe fn from_handle(handle: u32) -> Self {
                    Self { handle: _rt::Resource::from_handle(handle) }
                }
                #[doc(hidden)]
                pub fn take_handle(&self) -> u32 {
                    _rt::Resource::take_handle(&self.handle)
                }
                #[doc(hidden)]
                pub fn handle(&self) -> u32 {
                    _rt::Resource::handle(&self.handle)
                }
            }
            unsafe impl _rt::WasmResource for TerminalOutput {
                #[inline]
                unsafe fn drop(_handle: u32) {
                    #[cfg(not(target_arch = "wasm32"))]
                    unreachable!();
                    #[cfg(target_arch = "wasm32")]
                    {
                        #[link(wasm_import_module = "wasi:cli/terminal-output@0.2.0")]
                        extern "C" {
                            #[link_name = "[resource-drop]terminal-output"]
                            fn drop(_: u32);
                        }
                        drop(_handle);
                    }
                }
            }
        }
        #[allow(dead_code, clippy::all)]
        pub mod terminal_stdin {
            #[used]
            #[doc(hidden)]
            static __FORCE_SECTION_REF: fn() =
                super::super::super::__link_custom_section_describing_imports;
            use super::super::super::_rt;
            pub type TerminalInput = super::super::super::wasi::cli::terminal_input::TerminalInput;
            #[allow(unused_unsafe, clippy::all)]
            pub fn get_terminal_stdin() -> Option<TerminalInput> {
                unsafe {
                    #[repr(align(4))]
                    struct RetArea([::core::mem::MaybeUninit<u8>; 8]);
                    let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 8]);
                    let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                    #[cfg(target_arch = "wasm32")]
                    #[link(wasm_import_module = "wasi:cli/terminal-stdin@0.2.0")]
                    extern "C" {
                        #[link_name = "get-terminal-stdin"]
                        fn wit_import(_: *mut u8);
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import(_: *mut u8) {
                        unreachable!()
                    }
                    wit_import(ptr0);
                    let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                    match l1 {
                        0 => None,
                        1 => {
                            let e = {
                                let l2 = *ptr0.add(4).cast::<i32>();
                                super::super::super::wasi::cli::terminal_input::TerminalInput::from_handle(
                                    l2 as u32,
                                )
                            };
                            Some(e)
                        }
                        _ => _rt::invalid_enum_discriminant(),
                    }
                }
            }
        }
        #[allow(dead_code, clippy::all)]
        pub mod terminal_stdout {
            #[used]
            #[doc(hidden)]
            static __FORCE_SECTION_REF: fn() =
                super::super::super::__link_custom_section_describing_imports;
            use super::super::super::_rt;
            pub type TerminalOutput =
                super::super::super::wasi::cli::terminal_output::TerminalOutput;
            #[allow(unused_unsafe, clippy::all)]
            pub fn get_terminal_stdout() -> Option<TerminalOutput> {
                unsafe {
                    #[repr(align(4))]
                    struct RetArea([::core::mem::MaybeUninit<u8>; 8]);
                    let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 8]);
                    let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                    #[cfg(target_arch = "wasm32")]
                    #[link(wasm_import_module = "wasi:cli/terminal-stdout@0.2.0")]
                    extern "C" {
                        #[link_name = "get-terminal-stdout"]
                        fn wit_import(_: *mut u8);
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import(_: *mut u8) {
                        unreachable!()
                    }
                    wit_import(ptr0);
                    let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                    match l1 {
                        0 => None,
                        1 => {
                            let e = {
                                let l2 = *ptr0.add(4).cast::<i32>();
                                super::super::super::wasi::cli::terminal_output::TerminalOutput::from_handle(
                                    l2 as u32,
                                )
                            };
                            Some(e)
                        }
                        _ => _rt::invalid_enum_discriminant(),
                    }
                }
            }
        }
        #[allow(dead_code, clippy::all)]
        pub mod terminal_stderr {
            #[used]
            #[doc(hidden)]
            static __FORCE_SECTION_REF: fn() =
                super::super::super::__link_custom_section_describing_imports;
            use super::super::super::_rt;
            pub type TerminalOutput =
                super::super::super::wasi::cli::terminal_output::TerminalOutput;
            #[allow(unused_unsafe, clippy::all)]
            pub fn get_terminal_stderr() -> Option<TerminalOutput> {
                unsafe {
                    #[repr(align(4))]
                    struct RetArea([::core::mem::MaybeUninit<u8>; 8]);
                    let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 8]);
                    let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                    #[cfg(target_arch = "wasm32")]
                    #[link(wasm_import_module = "wasi:cli/terminal-stderr@0.2.0")]
                    extern "C" {
                        #[link_name = "get-terminal-stderr"]
                        fn wit_import(_: *mut u8);
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import(_: *mut u8) {
                        unreachable!()
                    }
                    wit_import(ptr0);
                    let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                    match l1 {
                        0 => None,
                        1 => {
                            let e = {
                                let l2 = *ptr0.add(4).cast::<i32>();
                                super::super::super::wasi::cli::terminal_output::TerminalOutput::from_handle(
                                    l2 as u32,
                                )
                            };
                            Some(e)
                        }
                        _ => _rt::invalid_enum_discriminant(),
                    }
                }
            }
        }
    }
    #[allow(dead_code)]
    pub mod clocks {
        #[allow(dead_code, clippy::all)]
        pub mod monotonic_clock {
            #[used]
            #[doc(hidden)]
            static __FORCE_SECTION_REF: fn() =
                super::super::super::__link_custom_section_describing_imports;
            use super::super::super::_rt;
            pub type Pollable = super::super::super::wasi::io::poll::Pollable;
            pub type Instant = u64;
            pub type Duration = u64;
            #[allow(unused_unsafe, clippy::all)]
            pub fn now() -> Instant {
                unsafe {
                    #[cfg(target_arch = "wasm32")]
                    #[link(wasm_import_module = "wasi:clocks/monotonic-clock@0.2.0")]
                    extern "C" {
                        #[link_name = "now"]
                        fn wit_import() -> i64;
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import() -> i64 {
                        unreachable!()
                    }
                    let ret = wit_import();
                    ret as u64
                }
            }
            #[allow(unused_unsafe, clippy::all)]
            pub fn resolution() -> Duration {
                unsafe {
                    #[cfg(target_arch = "wasm32")]
                    #[link(wasm_import_module = "wasi:clocks/monotonic-clock@0.2.0")]
                    extern "C" {
                        #[link_name = "resolution"]
                        fn wit_import() -> i64;
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import() -> i64 {
                        unreachable!()
                    }
                    let ret = wit_import();
                    ret as u64
                }
            }
            #[allow(unused_unsafe, clippy::all)]
            pub fn subscribe_instant(when: Instant) -> Pollable {
                unsafe {
                    #[cfg(target_arch = "wasm32")]
                    #[link(wasm_import_module = "wasi:clocks/monotonic-clock@0.2.0")]
                    extern "C" {
                        #[link_name = "subscribe-instant"]
                        fn wit_import(_: i64) -> i32;
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import(_: i64) -> i32 {
                        unreachable!()
                    }
                    let ret = wit_import(_rt::as_i64(when));
                    super::super::super::wasi::io::poll::Pollable::from_handle(ret as u32)
                }
            }
            #[allow(unused_unsafe, clippy::all)]
            pub fn subscribe_duration(when: Duration) -> Pollable {
                unsafe {
                    #[cfg(target_arch = "wasm32")]
                    #[link(wasm_import_module = "wasi:clocks/monotonic-clock@0.2.0")]
                    extern "C" {
                        #[link_name = "subscribe-duration"]
                        fn wit_import(_: i64) -> i32;
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import(_: i64) -> i32 {
                        unreachable!()
                    }
                    let ret = wit_import(_rt::as_i64(when));
                    super::super::super::wasi::io::poll::Pollable::from_handle(ret as u32)
                }
            }
        }
        #[allow(dead_code, clippy::all)]
        pub mod wall_clock {
            #[used]
            #[doc(hidden)]
            static __FORCE_SECTION_REF: fn() =
                super::super::super::__link_custom_section_describing_imports;
            #[repr(C)]
            #[derive(Clone, Copy)]
            pub struct Datetime {
                pub seconds: u64,
                pub nanoseconds: u32,
            }
            impl ::core::fmt::Debug for Datetime {
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    f.debug_struct("Datetime")
                        .field("seconds", &self.seconds)
                        .field("nanoseconds", &self.nanoseconds)
                        .finish()
                }
            }
            #[allow(unused_unsafe, clippy::all)]
            pub fn now() -> Datetime {
                unsafe {
                    #[repr(align(8))]
                    struct RetArea([::core::mem::MaybeUninit<u8>; 16]);
                    let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 16]);
                    let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                    #[cfg(target_arch = "wasm32")]
                    #[link(wasm_import_module = "wasi:clocks/wall-clock@0.2.0")]
                    extern "C" {
                        #[link_name = "now"]
                        fn wit_import(_: *mut u8);
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import(_: *mut u8) {
                        unreachable!()
                    }
                    wit_import(ptr0);
                    let l1 = *ptr0.add(0).cast::<i64>();
                    let l2 = *ptr0.add(8).cast::<i32>();
                    Datetime { seconds: l1 as u64, nanoseconds: l2 as u32 }
                }
            }
            #[allow(unused_unsafe, clippy::all)]
            pub fn resolution() -> Datetime {
                unsafe {
                    #[repr(align(8))]
                    struct RetArea([::core::mem::MaybeUninit<u8>; 16]);
                    let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 16]);
                    let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                    #[cfg(target_arch = "wasm32")]
                    #[link(wasm_import_module = "wasi:clocks/wall-clock@0.2.0")]
                    extern "C" {
                        #[link_name = "resolution"]
                        fn wit_import(_: *mut u8);
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import(_: *mut u8) {
                        unreachable!()
                    }
                    wit_import(ptr0);
                    let l1 = *ptr0.add(0).cast::<i64>();
                    let l2 = *ptr0.add(8).cast::<i32>();
                    Datetime { seconds: l1 as u64, nanoseconds: l2 as u32 }
                }
            }
        }
    }
    #[allow(dead_code)]
    pub mod filesystem {
        #[allow(dead_code, clippy::all)]
        pub mod types {
            #[used]
            #[doc(hidden)]
            static __FORCE_SECTION_REF: fn() =
                super::super::super::__link_custom_section_describing_imports;
            use super::super::super::_rt;
            pub type InputStream = super::super::super::wasi::io::streams::InputStream;
            pub type OutputStream = super::super::super::wasi::io::streams::OutputStream;
            pub type Error = super::super::super::wasi::io::streams::Error;
            pub type Datetime = super::super::super::wasi::clocks::wall_clock::Datetime;
            pub type Filesize = u64;
            #[repr(u8)]
            #[derive(Clone, Copy, Eq, Ord, PartialEq, PartialOrd)]
            pub enum DescriptorType {
                Unknown,
                BlockDevice,
                CharacterDevice,
                Directory,
                Fifo,
                SymbolicLink,
                RegularFile,
                Socket,
            }
            impl ::core::fmt::Debug for DescriptorType {
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    match self {
                        DescriptorType::Unknown => {
                            f.debug_tuple("DescriptorType::Unknown").finish()
                        }
                        DescriptorType::BlockDevice => {
                            f.debug_tuple("DescriptorType::BlockDevice").finish()
                        }
                        DescriptorType::CharacterDevice => {
                            f.debug_tuple("DescriptorType::CharacterDevice").finish()
                        }
                        DescriptorType::Directory => {
                            f.debug_tuple("DescriptorType::Directory").finish()
                        }
                        DescriptorType::Fifo => f.debug_tuple("DescriptorType::Fifo").finish(),
                        DescriptorType::SymbolicLink => {
                            f.debug_tuple("DescriptorType::SymbolicLink").finish()
                        }
                        DescriptorType::RegularFile => {
                            f.debug_tuple("DescriptorType::RegularFile").finish()
                        }
                        DescriptorType::Socket => f.debug_tuple("DescriptorType::Socket").finish(),
                    }
                }
            }
            impl DescriptorType {
                #[doc(hidden)]
                pub unsafe fn _lift(val: u8) -> DescriptorType {
                    if !cfg!(debug_assertions) {
                        return ::core::mem::transmute(val);
                    }
                    match val {
                        0 => DescriptorType::Unknown,
                        1 => DescriptorType::BlockDevice,
                        2 => DescriptorType::CharacterDevice,
                        3 => DescriptorType::Directory,
                        4 => DescriptorType::Fifo,
                        5 => DescriptorType::SymbolicLink,
                        6 => DescriptorType::RegularFile,
                        7 => DescriptorType::Socket,
                        _ => panic!("invalid enum discriminant"),
                    }
                }
            }
            wit_bindgen_rt::bitflags::bitflags! {
                #[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Clone, Copy)] pub
                struct DescriptorFlags : u8 { const READ = 1 << 0; const WRITE = 1 << 1;
                const FILE_INTEGRITY_SYNC = 1 << 2; const DATA_INTEGRITY_SYNC = 1 << 3;
                const REQUESTED_WRITE_SYNC = 1 << 4; const MUTATE_DIRECTORY = 1 << 5; }
            }
            wit_bindgen_rt::bitflags::bitflags! {
                #[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Clone, Copy)] pub
                struct PathFlags : u8 { const SYMLINK_FOLLOW = 1 << 0; }
            }
            wit_bindgen_rt::bitflags::bitflags! {
                #[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Clone, Copy)] pub
                struct OpenFlags : u8 { const CREATE = 1 << 0; const DIRECTORY = 1 << 1;
                const EXCLUSIVE = 1 << 2; const TRUNCATE = 1 << 3; }
            }
            pub type LinkCount = u64;
            #[repr(C)]
            #[derive(Clone, Copy)]
            pub struct DescriptorStat {
                pub type_: DescriptorType,
                pub link_count: LinkCount,
                pub size: Filesize,
                pub data_access_timestamp: Option<Datetime>,
                pub data_modification_timestamp: Option<Datetime>,
                pub status_change_timestamp: Option<Datetime>,
            }
            impl ::core::fmt::Debug for DescriptorStat {
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    f.debug_struct("DescriptorStat")
                        .field("type", &self.type_)
                        .field("link-count", &self.link_count)
                        .field("size", &self.size)
                        .field("data-access-timestamp", &self.data_access_timestamp)
                        .field("data-modification-timestamp", &self.data_modification_timestamp)
                        .field("status-change-timestamp", &self.status_change_timestamp)
                        .finish()
                }
            }
            #[derive(Clone, Copy)]
            pub enum NewTimestamp {
                NoChange,
                Now,
                Timestamp(Datetime),
            }
            impl ::core::fmt::Debug for NewTimestamp {
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    match self {
                        NewTimestamp::NoChange => f.debug_tuple("NewTimestamp::NoChange").finish(),
                        NewTimestamp::Now => f.debug_tuple("NewTimestamp::Now").finish(),
                        NewTimestamp::Timestamp(e) => {
                            f.debug_tuple("NewTimestamp::Timestamp").field(e).finish()
                        }
                    }
                }
            }
            #[derive(Clone)]
            pub struct DirectoryEntry {
                pub type_: DescriptorType,
                pub name: _rt::String,
            }
            impl ::core::fmt::Debug for DirectoryEntry {
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    f.debug_struct("DirectoryEntry")
                        .field("type", &self.type_)
                        .field("name", &self.name)
                        .finish()
                }
            }
            #[repr(u8)]
            #[derive(Clone, Copy, Eq, Ord, PartialEq, PartialOrd)]
            pub enum ErrorCode {
                Access,
                WouldBlock,
                Already,
                BadDescriptor,
                Busy,
                Deadlock,
                Quota,
                Exist,
                FileTooLarge,
                IllegalByteSequence,
                InProgress,
                Interrupted,
                Invalid,
                Io,
                IsDirectory,
                Loop,
                TooManyLinks,
                MessageSize,
                NameTooLong,
                NoDevice,
                NoEntry,
                NoLock,
                InsufficientMemory,
                InsufficientSpace,
                NotDirectory,
                NotEmpty,
                NotRecoverable,
                Unsupported,
                NoTty,
                NoSuchDevice,
                Overflow,
                NotPermitted,
                Pipe,
                ReadOnly,
                InvalidSeek,
                TextFileBusy,
                CrossDevice,
            }
            impl ErrorCode {
                pub fn name(&self) -> &'static str {
                    match self {
                        ErrorCode::Access => "access",
                        ErrorCode::WouldBlock => "would-block",
                        ErrorCode::Already => "already",
                        ErrorCode::BadDescriptor => "bad-descriptor",
                        ErrorCode::Busy => "busy",
                        ErrorCode::Deadlock => "deadlock",
                        ErrorCode::Quota => "quota",
                        ErrorCode::Exist => "exist",
                        ErrorCode::FileTooLarge => "file-too-large",
                        ErrorCode::IllegalByteSequence => "illegal-byte-sequence",
                        ErrorCode::InProgress => "in-progress",
                        ErrorCode::Interrupted => "interrupted",
                        ErrorCode::Invalid => "invalid",
                        ErrorCode::Io => "io",
                        ErrorCode::IsDirectory => "is-directory",
                        ErrorCode::Loop => "loop",
                        ErrorCode::TooManyLinks => "too-many-links",
                        ErrorCode::MessageSize => "message-size",
                        ErrorCode::NameTooLong => "name-too-long",
                        ErrorCode::NoDevice => "no-device",
                        ErrorCode::NoEntry => "no-entry",
                        ErrorCode::NoLock => "no-lock",
                        ErrorCode::InsufficientMemory => "insufficient-memory",
                        ErrorCode::InsufficientSpace => "insufficient-space",
                        ErrorCode::NotDirectory => "not-directory",
                        ErrorCode::NotEmpty => "not-empty",
                        ErrorCode::NotRecoverable => "not-recoverable",
                        ErrorCode::Unsupported => "unsupported",
                        ErrorCode::NoTty => "no-tty",
                        ErrorCode::NoSuchDevice => "no-such-device",
                        ErrorCode::Overflow => "overflow",
                        ErrorCode::NotPermitted => "not-permitted",
                        ErrorCode::Pipe => "pipe",
                        ErrorCode::ReadOnly => "read-only",
                        ErrorCode::InvalidSeek => "invalid-seek",
                        ErrorCode::TextFileBusy => "text-file-busy",
                        ErrorCode::CrossDevice => "cross-device",
                    }
                }
                pub fn message(&self) -> &'static str {
                    match self {
                        ErrorCode::Access => "",
                        ErrorCode::WouldBlock => "",
                        ErrorCode::Already => "",
                        ErrorCode::BadDescriptor => "",
                        ErrorCode::Busy => "",
                        ErrorCode::Deadlock => "",
                        ErrorCode::Quota => "",
                        ErrorCode::Exist => "",
                        ErrorCode::FileTooLarge => "",
                        ErrorCode::IllegalByteSequence => "",
                        ErrorCode::InProgress => "",
                        ErrorCode::Interrupted => "",
                        ErrorCode::Invalid => "",
                        ErrorCode::Io => "",
                        ErrorCode::IsDirectory => "",
                        ErrorCode::Loop => "",
                        ErrorCode::TooManyLinks => "",
                        ErrorCode::MessageSize => "",
                        ErrorCode::NameTooLong => "",
                        ErrorCode::NoDevice => "",
                        ErrorCode::NoEntry => "",
                        ErrorCode::NoLock => "",
                        ErrorCode::InsufficientMemory => "",
                        ErrorCode::InsufficientSpace => "",
                        ErrorCode::NotDirectory => "",
                        ErrorCode::NotEmpty => "",
                        ErrorCode::NotRecoverable => "",
                        ErrorCode::Unsupported => "",
                        ErrorCode::NoTty => "",
                        ErrorCode::NoSuchDevice => "",
                        ErrorCode::Overflow => "",
                        ErrorCode::NotPermitted => "",
                        ErrorCode::Pipe => "",
                        ErrorCode::ReadOnly => "",
                        ErrorCode::InvalidSeek => "",
                        ErrorCode::TextFileBusy => "",
                        ErrorCode::CrossDevice => "",
                    }
                }
            }
            impl ::core::fmt::Debug for ErrorCode {
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    f.debug_struct("ErrorCode")
                        .field("code", &(*self as i32))
                        .field("name", &self.name())
                        .field("message", &self.message())
                        .finish()
                }
            }
            impl ::core::fmt::Display for ErrorCode {
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    write!(f, "{} (error {})", self.name(), *self as i32)
                }
            }
            impl std::error::Error for ErrorCode {}
            impl ErrorCode {
                #[doc(hidden)]
                pub unsafe fn _lift(val: u8) -> ErrorCode {
                    if !cfg!(debug_assertions) {
                        return ::core::mem::transmute(val);
                    }
                    match val {
                        0 => ErrorCode::Access,
                        1 => ErrorCode::WouldBlock,
                        2 => ErrorCode::Already,
                        3 => ErrorCode::BadDescriptor,
                        4 => ErrorCode::Busy,
                        5 => ErrorCode::Deadlock,
                        6 => ErrorCode::Quota,
                        7 => ErrorCode::Exist,
                        8 => ErrorCode::FileTooLarge,
                        9 => ErrorCode::IllegalByteSequence,
                        10 => ErrorCode::InProgress,
                        11 => ErrorCode::Interrupted,
                        12 => ErrorCode::Invalid,
                        13 => ErrorCode::Io,
                        14 => ErrorCode::IsDirectory,
                        15 => ErrorCode::Loop,
                        16 => ErrorCode::TooManyLinks,
                        17 => ErrorCode::MessageSize,
                        18 => ErrorCode::NameTooLong,
                        19 => ErrorCode::NoDevice,
                        20 => ErrorCode::NoEntry,
                        21 => ErrorCode::NoLock,
                        22 => ErrorCode::InsufficientMemory,
                        23 => ErrorCode::InsufficientSpace,
                        24 => ErrorCode::NotDirectory,
                        25 => ErrorCode::NotEmpty,
                        26 => ErrorCode::NotRecoverable,
                        27 => ErrorCode::Unsupported,
                        28 => ErrorCode::NoTty,
                        29 => ErrorCode::NoSuchDevice,
                        30 => ErrorCode::Overflow,
                        31 => ErrorCode::NotPermitted,
                        32 => ErrorCode::Pipe,
                        33 => ErrorCode::ReadOnly,
                        34 => ErrorCode::InvalidSeek,
                        35 => ErrorCode::TextFileBusy,
                        36 => ErrorCode::CrossDevice,
                        _ => panic!("invalid enum discriminant"),
                    }
                }
            }
            #[repr(u8)]
            #[derive(Clone, Copy, Eq, Ord, PartialEq, PartialOrd)]
            pub enum Advice {
                Normal,
                Sequential,
                Random,
                WillNeed,
                DontNeed,
                NoReuse,
            }
            impl ::core::fmt::Debug for Advice {
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    match self {
                        Advice::Normal => f.debug_tuple("Advice::Normal").finish(),
                        Advice::Sequential => f.debug_tuple("Advice::Sequential").finish(),
                        Advice::Random => f.debug_tuple("Advice::Random").finish(),
                        Advice::WillNeed => f.debug_tuple("Advice::WillNeed").finish(),
                        Advice::DontNeed => f.debug_tuple("Advice::DontNeed").finish(),
                        Advice::NoReuse => f.debug_tuple("Advice::NoReuse").finish(),
                    }
                }
            }
            impl Advice {
                #[doc(hidden)]
                pub unsafe fn _lift(val: u8) -> Advice {
                    if !cfg!(debug_assertions) {
                        return ::core::mem::transmute(val);
                    }
                    match val {
                        0 => Advice::Normal,
                        1 => Advice::Sequential,
                        2 => Advice::Random,
                        3 => Advice::WillNeed,
                        4 => Advice::DontNeed,
                        5 => Advice::NoReuse,
                        _ => panic!("invalid enum discriminant"),
                    }
                }
            }
            #[repr(C)]
            #[derive(Clone, Copy)]
            pub struct MetadataHashValue {
                pub lower: u64,
                pub upper: u64,
            }
            impl ::core::fmt::Debug for MetadataHashValue {
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    f.debug_struct("MetadataHashValue")
                        .field("lower", &self.lower)
                        .field("upper", &self.upper)
                        .finish()
                }
            }
            #[derive(Debug)]
            #[repr(transparent)]
            pub struct Descriptor {
                handle: _rt::Resource<Descriptor>,
            }
            impl Descriptor {
                #[doc(hidden)]
                pub unsafe fn from_handle(handle: u32) -> Self {
                    Self { handle: _rt::Resource::from_handle(handle) }
                }
                #[doc(hidden)]
                pub fn take_handle(&self) -> u32 {
                    _rt::Resource::take_handle(&self.handle)
                }
                #[doc(hidden)]
                pub fn handle(&self) -> u32 {
                    _rt::Resource::handle(&self.handle)
                }
            }
            unsafe impl _rt::WasmResource for Descriptor {
                #[inline]
                unsafe fn drop(_handle: u32) {
                    #[cfg(not(target_arch = "wasm32"))]
                    unreachable!();
                    #[cfg(target_arch = "wasm32")]
                    {
                        #[link(wasm_import_module = "wasi:filesystem/types@0.2.0")]
                        extern "C" {
                            #[link_name = "[resource-drop]descriptor"]
                            fn drop(_: u32);
                        }
                        drop(_handle);
                    }
                }
            }
            #[derive(Debug)]
            #[repr(transparent)]
            pub struct DirectoryEntryStream {
                handle: _rt::Resource<DirectoryEntryStream>,
            }
            impl DirectoryEntryStream {
                #[doc(hidden)]
                pub unsafe fn from_handle(handle: u32) -> Self {
                    Self { handle: _rt::Resource::from_handle(handle) }
                }
                #[doc(hidden)]
                pub fn take_handle(&self) -> u32 {
                    _rt::Resource::take_handle(&self.handle)
                }
                #[doc(hidden)]
                pub fn handle(&self) -> u32 {
                    _rt::Resource::handle(&self.handle)
                }
            }
            unsafe impl _rt::WasmResource for DirectoryEntryStream {
                #[inline]
                unsafe fn drop(_handle: u32) {
                    #[cfg(not(target_arch = "wasm32"))]
                    unreachable!();
                    #[cfg(target_arch = "wasm32")]
                    {
                        #[link(wasm_import_module = "wasi:filesystem/types@0.2.0")]
                        extern "C" {
                            #[link_name = "[resource-drop]directory-entry-stream"]
                            fn drop(_: u32);
                        }
                        drop(_handle);
                    }
                }
            }
            impl Descriptor {
                #[allow(unused_unsafe, clippy::all)]
                pub fn read_via_stream(&self, offset: Filesize) -> Result<InputStream, ErrorCode> {
                    unsafe {
                        #[repr(align(4))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 8]);
                        let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 8]);
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:filesystem/types@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]descriptor.read-via-stream"]
                            fn wit_import(_: i32, _: i64, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: i64, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, _rt::as_i64(offset), ptr0);
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        match l1 {
                            0 => {
                                let e = {
                                    let l2 = *ptr0.add(4).cast::<i32>();
                                    super::super::super::wasi::io::streams::InputStream::from_handle(
                                        l2 as u32,
                                    )
                                };
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l3 = i32::from(*ptr0.add(4).cast::<u8>());
                                    ErrorCode::_lift(l3 as u8)
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl Descriptor {
                #[allow(unused_unsafe, clippy::all)]
                pub fn write_via_stream(
                    &self,
                    offset: Filesize,
                ) -> Result<OutputStream, ErrorCode> {
                    unsafe {
                        #[repr(align(4))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 8]);
                        let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 8]);
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:filesystem/types@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]descriptor.write-via-stream"]
                            fn wit_import(_: i32, _: i64, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: i64, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, _rt::as_i64(offset), ptr0);
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        match l1 {
                            0 => {
                                let e = {
                                    let l2 = *ptr0.add(4).cast::<i32>();
                                    super::super::super::wasi::io::streams::OutputStream::from_handle(
                                        l2 as u32,
                                    )
                                };
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l3 = i32::from(*ptr0.add(4).cast::<u8>());
                                    ErrorCode::_lift(l3 as u8)
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl Descriptor {
                #[allow(unused_unsafe, clippy::all)]
                pub fn append_via_stream(&self) -> Result<OutputStream, ErrorCode> {
                    unsafe {
                        #[repr(align(4))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 8]);
                        let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 8]);
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:filesystem/types@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]descriptor.append-via-stream"]
                            fn wit_import(_: i32, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, ptr0);
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        match l1 {
                            0 => {
                                let e = {
                                    let l2 = *ptr0.add(4).cast::<i32>();
                                    super::super::super::wasi::io::streams::OutputStream::from_handle(
                                        l2 as u32,
                                    )
                                };
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l3 = i32::from(*ptr0.add(4).cast::<u8>());
                                    ErrorCode::_lift(l3 as u8)
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl Descriptor {
                #[allow(unused_unsafe, clippy::all)]
                pub fn advise(
                    &self,
                    offset: Filesize,
                    length: Filesize,
                    advice: Advice,
                ) -> Result<(), ErrorCode> {
                    unsafe {
                        #[repr(align(1))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 2]);
                        let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 2]);
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:filesystem/types@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]descriptor.advise"]
                            fn wit_import(_: i32, _: i64, _: i64, _: i32, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: i64, _: i64, _: i32, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import(
                            (self).handle() as i32,
                            _rt::as_i64(offset),
                            _rt::as_i64(length),
                            advice.clone() as i32,
                            ptr0,
                        );
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        match l1 {
                            0 => {
                                let e = ();
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l2 = i32::from(*ptr0.add(1).cast::<u8>());
                                    ErrorCode::_lift(l2 as u8)
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl Descriptor {
                #[allow(unused_unsafe, clippy::all)]
                pub fn sync_data(&self) -> Result<(), ErrorCode> {
                    unsafe {
                        #[repr(align(1))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 2]);
                        let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 2]);
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:filesystem/types@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]descriptor.sync-data"]
                            fn wit_import(_: i32, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, ptr0);
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        match l1 {
                            0 => {
                                let e = ();
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l2 = i32::from(*ptr0.add(1).cast::<u8>());
                                    ErrorCode::_lift(l2 as u8)
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl Descriptor {
                #[allow(unused_unsafe, clippy::all)]
                pub fn get_flags(&self) -> Result<DescriptorFlags, ErrorCode> {
                    unsafe {
                        #[repr(align(1))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 2]);
                        let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 2]);
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:filesystem/types@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]descriptor.get-flags"]
                            fn wit_import(_: i32, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, ptr0);
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        match l1 {
                            0 => {
                                let e = {
                                    let l2 = i32::from(*ptr0.add(1).cast::<u8>());
                                    DescriptorFlags::empty()
                                        | DescriptorFlags::from_bits_retain(((l2 as u8) << 0) as _)
                                };
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l3 = i32::from(*ptr0.add(1).cast::<u8>());
                                    ErrorCode::_lift(l3 as u8)
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl Descriptor {
                #[allow(unused_unsafe, clippy::all)]
                pub fn get_type(&self) -> Result<DescriptorType, ErrorCode> {
                    unsafe {
                        #[repr(align(1))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 2]);
                        let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 2]);
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:filesystem/types@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]descriptor.get-type"]
                            fn wit_import(_: i32, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, ptr0);
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        match l1 {
                            0 => {
                                let e = {
                                    let l2 = i32::from(*ptr0.add(1).cast::<u8>());
                                    DescriptorType::_lift(l2 as u8)
                                };
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l3 = i32::from(*ptr0.add(1).cast::<u8>());
                                    ErrorCode::_lift(l3 as u8)
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl Descriptor {
                #[allow(unused_unsafe, clippy::all)]
                pub fn set_size(&self, size: Filesize) -> Result<(), ErrorCode> {
                    unsafe {
                        #[repr(align(1))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 2]);
                        let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 2]);
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:filesystem/types@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]descriptor.set-size"]
                            fn wit_import(_: i32, _: i64, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: i64, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, _rt::as_i64(size), ptr0);
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        match l1 {
                            0 => {
                                let e = ();
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l2 = i32::from(*ptr0.add(1).cast::<u8>());
                                    ErrorCode::_lift(l2 as u8)
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl Descriptor {
                #[allow(unused_unsafe, clippy::all)]
                pub fn set_times(
                    &self,
                    data_access_timestamp: NewTimestamp,
                    data_modification_timestamp: NewTimestamp,
                ) -> Result<(), ErrorCode> {
                    unsafe {
                        #[repr(align(1))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 2]);
                        let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 2]);
                        let (result1_0, result1_1, result1_2) = match data_access_timestamp {
                            NewTimestamp::NoChange => (0i32, 0i64, 0i32),
                            NewTimestamp::Now => (1i32, 0i64, 0i32),
                            NewTimestamp::Timestamp(e) => {
                                let super::super::super::wasi::clocks::wall_clock::Datetime {
                                    seconds: seconds0,
                                    nanoseconds: nanoseconds0,
                                } = e;
                                (2i32, _rt::as_i64(seconds0), _rt::as_i32(nanoseconds0))
                            }
                        };
                        let (result3_0, result3_1, result3_2) = match data_modification_timestamp {
                            NewTimestamp::NoChange => (0i32, 0i64, 0i32),
                            NewTimestamp::Now => (1i32, 0i64, 0i32),
                            NewTimestamp::Timestamp(e) => {
                                let super::super::super::wasi::clocks::wall_clock::Datetime {
                                    seconds: seconds2,
                                    nanoseconds: nanoseconds2,
                                } = e;
                                (2i32, _rt::as_i64(seconds2), _rt::as_i32(nanoseconds2))
                            }
                        };
                        let ptr4 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:filesystem/types@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]descriptor.set-times"]
                            fn wit_import(
                                _: i32,
                                _: i32,
                                _: i64,
                                _: i32,
                                _: i32,
                                _: i64,
                                _: i32,
                                _: *mut u8,
                            );
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(
                            _: i32,
                            _: i32,
                            _: i64,
                            _: i32,
                            _: i32,
                            _: i64,
                            _: i32,
                            _: *mut u8,
                        ) {
                            unreachable!()
                        }
                        wit_import(
                            (self).handle() as i32,
                            result1_0,
                            result1_1,
                            result1_2,
                            result3_0,
                            result3_1,
                            result3_2,
                            ptr4,
                        );
                        let l5 = i32::from(*ptr4.add(0).cast::<u8>());
                        match l5 {
                            0 => {
                                let e = ();
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l6 = i32::from(*ptr4.add(1).cast::<u8>());
                                    ErrorCode::_lift(l6 as u8)
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl Descriptor {
                #[allow(unused_unsafe, clippy::all)]
                pub fn read(
                    &self,
                    length: Filesize,
                    offset: Filesize,
                ) -> Result<(_rt::Vec<u8>, bool), ErrorCode> {
                    unsafe {
                        #[repr(align(4))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 16]);
                        let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 16]);
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:filesystem/types@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]descriptor.read"]
                            fn wit_import(_: i32, _: i64, _: i64, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: i64, _: i64, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import(
                            (self).handle() as i32,
                            _rt::as_i64(length),
                            _rt::as_i64(offset),
                            ptr0,
                        );
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        match l1 {
                            0 => {
                                let e = {
                                    let l2 = *ptr0.add(4).cast::<*mut u8>();
                                    let l3 = *ptr0.add(8).cast::<usize>();
                                    let len4 = l3;
                                    let l5 = i32::from(*ptr0.add(12).cast::<u8>());
                                    (
                                        _rt::Vec::from_raw_parts(l2.cast(), len4, len4),
                                        _rt::bool_lift(l5 as u8),
                                    )
                                };
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l6 = i32::from(*ptr0.add(4).cast::<u8>());
                                    ErrorCode::_lift(l6 as u8)
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl Descriptor {
                #[allow(unused_unsafe, clippy::all)]
                pub fn write(
                    &self,
                    buffer: &[u8],
                    offset: Filesize,
                ) -> Result<Filesize, ErrorCode> {
                    unsafe {
                        #[repr(align(8))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 16]);
                        let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 16]);
                        let vec0 = buffer;
                        let ptr0 = vec0.as_ptr().cast::<u8>();
                        let len0 = vec0.len();
                        let ptr1 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:filesystem/types@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]descriptor.write"]
                            fn wit_import(_: i32, _: *mut u8, _: usize, _: i64, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: *mut u8, _: usize, _: i64, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import(
                            (self).handle() as i32,
                            ptr0.cast_mut(),
                            len0,
                            _rt::as_i64(offset),
                            ptr1,
                        );
                        let l2 = i32::from(*ptr1.add(0).cast::<u8>());
                        match l2 {
                            0 => {
                                let e = {
                                    let l3 = *ptr1.add(8).cast::<i64>();
                                    l3 as u64
                                };
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l4 = i32::from(*ptr1.add(8).cast::<u8>());
                                    ErrorCode::_lift(l4 as u8)
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl Descriptor {
                #[allow(unused_unsafe, clippy::all)]
                pub fn read_directory(&self) -> Result<DirectoryEntryStream, ErrorCode> {
                    unsafe {
                        #[repr(align(4))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 8]);
                        let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 8]);
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:filesystem/types@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]descriptor.read-directory"]
                            fn wit_import(_: i32, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, ptr0);
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        match l1 {
                            0 => {
                                let e = {
                                    let l2 = *ptr0.add(4).cast::<i32>();
                                    DirectoryEntryStream::from_handle(l2 as u32)
                                };
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l3 = i32::from(*ptr0.add(4).cast::<u8>());
                                    ErrorCode::_lift(l3 as u8)
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl Descriptor {
                #[allow(unused_unsafe, clippy::all)]
                pub fn sync(&self) -> Result<(), ErrorCode> {
                    unsafe {
                        #[repr(align(1))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 2]);
                        let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 2]);
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:filesystem/types@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]descriptor.sync"]
                            fn wit_import(_: i32, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, ptr0);
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        match l1 {
                            0 => {
                                let e = ();
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l2 = i32::from(*ptr0.add(1).cast::<u8>());
                                    ErrorCode::_lift(l2 as u8)
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl Descriptor {
                #[allow(unused_unsafe, clippy::all)]
                pub fn create_directory_at(&self, path: &str) -> Result<(), ErrorCode> {
                    unsafe {
                        #[repr(align(1))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 2]);
                        let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 2]);
                        let vec0 = path;
                        let ptr0 = vec0.as_ptr().cast::<u8>();
                        let len0 = vec0.len();
                        let ptr1 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:filesystem/types@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]descriptor.create-directory-at"]
                            fn wit_import(_: i32, _: *mut u8, _: usize, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: *mut u8, _: usize, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, ptr0.cast_mut(), len0, ptr1);
                        let l2 = i32::from(*ptr1.add(0).cast::<u8>());
                        match l2 {
                            0 => {
                                let e = ();
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l3 = i32::from(*ptr1.add(1).cast::<u8>());
                                    ErrorCode::_lift(l3 as u8)
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl Descriptor {
                #[allow(unused_unsafe, clippy::all)]
                pub fn stat(&self) -> Result<DescriptorStat, ErrorCode> {
                    unsafe {
                        #[repr(align(8))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 104]);
                        let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 104]);
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:filesystem/types@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]descriptor.stat"]
                            fn wit_import(_: i32, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, ptr0);
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        match l1 {
                            0 => {
                                let e = {
                                    let l2 = i32::from(*ptr0.add(8).cast::<u8>());
                                    let l3 = *ptr0.add(16).cast::<i64>();
                                    let l4 = *ptr0.add(24).cast::<i64>();
                                    let l5 = i32::from(*ptr0.add(32).cast::<u8>());
                                    let l8 = i32::from(*ptr0.add(56).cast::<u8>());
                                    let l11 = i32::from(*ptr0.add(80).cast::<u8>());
                                    DescriptorStat {
                                        type_: DescriptorType::_lift(l2 as u8),
                                        link_count: l3 as u64,
                                        size: l4 as u64,
                                        data_access_timestamp: match l5 {
                                            0 => None,
                                            1 => {
                                                let e = {
                                                    let l6 = *ptr0.add(40).cast::<i64>();
                                                    let l7 = *ptr0.add(48).cast::<i32>();
                                                    super::super::super::wasi::clocks::wall_clock::Datetime {
                                                        seconds: l6 as u64,
                                                        nanoseconds: l7 as u32,
                                                    }
                                                };
                                                Some(e)
                                            }
                                            _ => _rt::invalid_enum_discriminant(),
                                        },
                                        data_modification_timestamp: match l8 {
                                            0 => None,
                                            1 => {
                                                let e = {
                                                    let l9 = *ptr0.add(64).cast::<i64>();
                                                    let l10 = *ptr0.add(72).cast::<i32>();
                                                    super::super::super::wasi::clocks::wall_clock::Datetime {
                                                        seconds: l9 as u64,
                                                        nanoseconds: l10 as u32,
                                                    }
                                                };
                                                Some(e)
                                            }
                                            _ => _rt::invalid_enum_discriminant(),
                                        },
                                        status_change_timestamp: match l11 {
                                            0 => None,
                                            1 => {
                                                let e = {
                                                    let l12 = *ptr0.add(88).cast::<i64>();
                                                    let l13 = *ptr0.add(96).cast::<i32>();
                                                    super::super::super::wasi::clocks::wall_clock::Datetime {
                                                        seconds: l12 as u64,
                                                        nanoseconds: l13 as u32,
                                                    }
                                                };
                                                Some(e)
                                            }
                                            _ => _rt::invalid_enum_discriminant(),
                                        },
                                    }
                                };
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l14 = i32::from(*ptr0.add(8).cast::<u8>());
                                    ErrorCode::_lift(l14 as u8)
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl Descriptor {
                #[allow(unused_unsafe, clippy::all)]
                pub fn stat_at(
                    &self,
                    path_flags: PathFlags,
                    path: &str,
                ) -> Result<DescriptorStat, ErrorCode> {
                    unsafe {
                        #[repr(align(8))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 104]);
                        let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 104]);
                        let flags0 = path_flags;
                        let vec1 = path;
                        let ptr1 = vec1.as_ptr().cast::<u8>();
                        let len1 = vec1.len();
                        let ptr2 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:filesystem/types@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]descriptor.stat-at"]
                            fn wit_import(_: i32, _: i32, _: *mut u8, _: usize, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: i32, _: *mut u8, _: usize, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import(
                            (self).handle() as i32,
                            (flags0.bits() >> 0) as i32,
                            ptr1.cast_mut(),
                            len1,
                            ptr2,
                        );
                        let l3 = i32::from(*ptr2.add(0).cast::<u8>());
                        match l3 {
                            0 => {
                                let e = {
                                    let l4 = i32::from(*ptr2.add(8).cast::<u8>());
                                    let l5 = *ptr2.add(16).cast::<i64>();
                                    let l6 = *ptr2.add(24).cast::<i64>();
                                    let l7 = i32::from(*ptr2.add(32).cast::<u8>());
                                    let l10 = i32::from(*ptr2.add(56).cast::<u8>());
                                    let l13 = i32::from(*ptr2.add(80).cast::<u8>());
                                    DescriptorStat {
                                        type_: DescriptorType::_lift(l4 as u8),
                                        link_count: l5 as u64,
                                        size: l6 as u64,
                                        data_access_timestamp: match l7 {
                                            0 => None,
                                            1 => {
                                                let e = {
                                                    let l8 = *ptr2.add(40).cast::<i64>();
                                                    let l9 = *ptr2.add(48).cast::<i32>();
                                                    super::super::super::wasi::clocks::wall_clock::Datetime {
                                                        seconds: l8 as u64,
                                                        nanoseconds: l9 as u32,
                                                    }
                                                };
                                                Some(e)
                                            }
                                            _ => _rt::invalid_enum_discriminant(),
                                        },
                                        data_modification_timestamp: match l10 {
                                            0 => None,
                                            1 => {
                                                let e = {
                                                    let l11 = *ptr2.add(64).cast::<i64>();
                                                    let l12 = *ptr2.add(72).cast::<i32>();
                                                    super::super::super::wasi::clocks::wall_clock::Datetime {
                                                        seconds: l11 as u64,
                                                        nanoseconds: l12 as u32,
                                                    }
                                                };
                                                Some(e)
                                            }
                                            _ => _rt::invalid_enum_discriminant(),
                                        },
                                        status_change_timestamp: match l13 {
                                            0 => None,
                                            1 => {
                                                let e = {
                                                    let l14 = *ptr2.add(88).cast::<i64>();
                                                    let l15 = *ptr2.add(96).cast::<i32>();
                                                    super::super::super::wasi::clocks::wall_clock::Datetime {
                                                        seconds: l14 as u64,
                                                        nanoseconds: l15 as u32,
                                                    }
                                                };
                                                Some(e)
                                            }
                                            _ => _rt::invalid_enum_discriminant(),
                                        },
                                    }
                                };
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l16 = i32::from(*ptr2.add(8).cast::<u8>());
                                    ErrorCode::_lift(l16 as u8)
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl Descriptor {
                #[allow(unused_unsafe, clippy::all)]
                pub fn set_times_at(
                    &self,
                    path_flags: PathFlags,
                    path: &str,
                    data_access_timestamp: NewTimestamp,
                    data_modification_timestamp: NewTimestamp,
                ) -> Result<(), ErrorCode> {
                    unsafe {
                        #[repr(align(1))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 2]);
                        let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 2]);
                        let flags0 = path_flags;
                        let vec1 = path;
                        let ptr1 = vec1.as_ptr().cast::<u8>();
                        let len1 = vec1.len();
                        let (result3_0, result3_1, result3_2) = match data_access_timestamp {
                            NewTimestamp::NoChange => (0i32, 0i64, 0i32),
                            NewTimestamp::Now => (1i32, 0i64, 0i32),
                            NewTimestamp::Timestamp(e) => {
                                let super::super::super::wasi::clocks::wall_clock::Datetime {
                                    seconds: seconds2,
                                    nanoseconds: nanoseconds2,
                                } = e;
                                (2i32, _rt::as_i64(seconds2), _rt::as_i32(nanoseconds2))
                            }
                        };
                        let (result5_0, result5_1, result5_2) = match data_modification_timestamp {
                            NewTimestamp::NoChange => (0i32, 0i64, 0i32),
                            NewTimestamp::Now => (1i32, 0i64, 0i32),
                            NewTimestamp::Timestamp(e) => {
                                let super::super::super::wasi::clocks::wall_clock::Datetime {
                                    seconds: seconds4,
                                    nanoseconds: nanoseconds4,
                                } = e;
                                (2i32, _rt::as_i64(seconds4), _rt::as_i32(nanoseconds4))
                            }
                        };
                        let ptr6 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:filesystem/types@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]descriptor.set-times-at"]
                            fn wit_import(
                                _: i32,
                                _: i32,
                                _: *mut u8,
                                _: usize,
                                _: i32,
                                _: i64,
                                _: i32,
                                _: i32,
                                _: i64,
                                _: i32,
                                _: *mut u8,
                            );
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(
                            _: i32,
                            _: i32,
                            _: *mut u8,
                            _: usize,
                            _: i32,
                            _: i64,
                            _: i32,
                            _: i32,
                            _: i64,
                            _: i32,
                            _: *mut u8,
                        ) {
                            unreachable!()
                        }
                        wit_import(
                            (self).handle() as i32,
                            (flags0.bits() >> 0) as i32,
                            ptr1.cast_mut(),
                            len1,
                            result3_0,
                            result3_1,
                            result3_2,
                            result5_0,
                            result5_1,
                            result5_2,
                            ptr6,
                        );
                        let l7 = i32::from(*ptr6.add(0).cast::<u8>());
                        match l7 {
                            0 => {
                                let e = ();
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l8 = i32::from(*ptr6.add(1).cast::<u8>());
                                    ErrorCode::_lift(l8 as u8)
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl Descriptor {
                #[allow(unused_unsafe, clippy::all)]
                pub fn link_at(
                    &self,
                    old_path_flags: PathFlags,
                    old_path: &str,
                    new_descriptor: &Descriptor,
                    new_path: &str,
                ) -> Result<(), ErrorCode> {
                    unsafe {
                        #[repr(align(1))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 2]);
                        let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 2]);
                        let flags0 = old_path_flags;
                        let vec1 = old_path;
                        let ptr1 = vec1.as_ptr().cast::<u8>();
                        let len1 = vec1.len();
                        let vec2 = new_path;
                        let ptr2 = vec2.as_ptr().cast::<u8>();
                        let len2 = vec2.len();
                        let ptr3 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:filesystem/types@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]descriptor.link-at"]
                            fn wit_import(
                                _: i32,
                                _: i32,
                                _: *mut u8,
                                _: usize,
                                _: i32,
                                _: *mut u8,
                                _: usize,
                                _: *mut u8,
                            );
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(
                            _: i32,
                            _: i32,
                            _: *mut u8,
                            _: usize,
                            _: i32,
                            _: *mut u8,
                            _: usize,
                            _: *mut u8,
                        ) {
                            unreachable!()
                        }
                        wit_import(
                            (self).handle() as i32,
                            (flags0.bits() >> 0) as i32,
                            ptr1.cast_mut(),
                            len1,
                            (new_descriptor).handle() as i32,
                            ptr2.cast_mut(),
                            len2,
                            ptr3,
                        );
                        let l4 = i32::from(*ptr3.add(0).cast::<u8>());
                        match l4 {
                            0 => {
                                let e = ();
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l5 = i32::from(*ptr3.add(1).cast::<u8>());
                                    ErrorCode::_lift(l5 as u8)
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl Descriptor {
                #[allow(unused_unsafe, clippy::all)]
                pub fn open_at(
                    &self,
                    path_flags: PathFlags,
                    path: &str,
                    open_flags: OpenFlags,
                    flags: DescriptorFlags,
                ) -> Result<Descriptor, ErrorCode> {
                    unsafe {
                        #[repr(align(4))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 8]);
                        let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 8]);
                        let flags0 = path_flags;
                        let vec1 = path;
                        let ptr1 = vec1.as_ptr().cast::<u8>();
                        let len1 = vec1.len();
                        let flags2 = open_flags;
                        let flags3 = flags;
                        let ptr4 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:filesystem/types@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]descriptor.open-at"]
                            fn wit_import(
                                _: i32,
                                _: i32,
                                _: *mut u8,
                                _: usize,
                                _: i32,
                                _: i32,
                                _: *mut u8,
                            );
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(
                            _: i32,
                            _: i32,
                            _: *mut u8,
                            _: usize,
                            _: i32,
                            _: i32,
                            _: *mut u8,
                        ) {
                            unreachable!()
                        }
                        wit_import(
                            (self).handle() as i32,
                            (flags0.bits() >> 0) as i32,
                            ptr1.cast_mut(),
                            len1,
                            (flags2.bits() >> 0) as i32,
                            (flags3.bits() >> 0) as i32,
                            ptr4,
                        );
                        let l5 = i32::from(*ptr4.add(0).cast::<u8>());
                        match l5 {
                            0 => {
                                let e = {
                                    let l6 = *ptr4.add(4).cast::<i32>();
                                    Descriptor::from_handle(l6 as u32)
                                };
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l7 = i32::from(*ptr4.add(4).cast::<u8>());
                                    ErrorCode::_lift(l7 as u8)
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl Descriptor {
                #[allow(unused_unsafe, clippy::all)]
                pub fn readlink_at(&self, path: &str) -> Result<_rt::String, ErrorCode> {
                    unsafe {
                        #[repr(align(4))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 12]);
                        let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 12]);
                        let vec0 = path;
                        let ptr0 = vec0.as_ptr().cast::<u8>();
                        let len0 = vec0.len();
                        let ptr1 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:filesystem/types@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]descriptor.readlink-at"]
                            fn wit_import(_: i32, _: *mut u8, _: usize, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: *mut u8, _: usize, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, ptr0.cast_mut(), len0, ptr1);
                        let l2 = i32::from(*ptr1.add(0).cast::<u8>());
                        match l2 {
                            0 => {
                                let e = {
                                    let l3 = *ptr1.add(4).cast::<*mut u8>();
                                    let l4 = *ptr1.add(8).cast::<usize>();
                                    let len5 = l4;
                                    let bytes5 = _rt::Vec::from_raw_parts(l3.cast(), len5, len5);
                                    _rt::string_lift(bytes5)
                                };
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l6 = i32::from(*ptr1.add(4).cast::<u8>());
                                    ErrorCode::_lift(l6 as u8)
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl Descriptor {
                #[allow(unused_unsafe, clippy::all)]
                pub fn remove_directory_at(&self, path: &str) -> Result<(), ErrorCode> {
                    unsafe {
                        #[repr(align(1))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 2]);
                        let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 2]);
                        let vec0 = path;
                        let ptr0 = vec0.as_ptr().cast::<u8>();
                        let len0 = vec0.len();
                        let ptr1 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:filesystem/types@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]descriptor.remove-directory-at"]
                            fn wit_import(_: i32, _: *mut u8, _: usize, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: *mut u8, _: usize, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, ptr0.cast_mut(), len0, ptr1);
                        let l2 = i32::from(*ptr1.add(0).cast::<u8>());
                        match l2 {
                            0 => {
                                let e = ();
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l3 = i32::from(*ptr1.add(1).cast::<u8>());
                                    ErrorCode::_lift(l3 as u8)
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl Descriptor {
                #[allow(unused_unsafe, clippy::all)]
                pub fn rename_at(
                    &self,
                    old_path: &str,
                    new_descriptor: &Descriptor,
                    new_path: &str,
                ) -> Result<(), ErrorCode> {
                    unsafe {
                        #[repr(align(1))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 2]);
                        let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 2]);
                        let vec0 = old_path;
                        let ptr0 = vec0.as_ptr().cast::<u8>();
                        let len0 = vec0.len();
                        let vec1 = new_path;
                        let ptr1 = vec1.as_ptr().cast::<u8>();
                        let len1 = vec1.len();
                        let ptr2 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:filesystem/types@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]descriptor.rename-at"]
                            fn wit_import(
                                _: i32,
                                _: *mut u8,
                                _: usize,
                                _: i32,
                                _: *mut u8,
                                _: usize,
                                _: *mut u8,
                            );
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(
                            _: i32,
                            _: *mut u8,
                            _: usize,
                            _: i32,
                            _: *mut u8,
                            _: usize,
                            _: *mut u8,
                        ) {
                            unreachable!()
                        }
                        wit_import(
                            (self).handle() as i32,
                            ptr0.cast_mut(),
                            len0,
                            (new_descriptor).handle() as i32,
                            ptr1.cast_mut(),
                            len1,
                            ptr2,
                        );
                        let l3 = i32::from(*ptr2.add(0).cast::<u8>());
                        match l3 {
                            0 => {
                                let e = ();
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l4 = i32::from(*ptr2.add(1).cast::<u8>());
                                    ErrorCode::_lift(l4 as u8)
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl Descriptor {
                #[allow(unused_unsafe, clippy::all)]
                pub fn symlink_at(&self, old_path: &str, new_path: &str) -> Result<(), ErrorCode> {
                    unsafe {
                        #[repr(align(1))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 2]);
                        let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 2]);
                        let vec0 = old_path;
                        let ptr0 = vec0.as_ptr().cast::<u8>();
                        let len0 = vec0.len();
                        let vec1 = new_path;
                        let ptr1 = vec1.as_ptr().cast::<u8>();
                        let len1 = vec1.len();
                        let ptr2 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:filesystem/types@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]descriptor.symlink-at"]
                            fn wit_import(
                                _: i32,
                                _: *mut u8,
                                _: usize,
                                _: *mut u8,
                                _: usize,
                                _: *mut u8,
                            );
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(
                            _: i32,
                            _: *mut u8,
                            _: usize,
                            _: *mut u8,
                            _: usize,
                            _: *mut u8,
                        ) {
                            unreachable!()
                        }
                        wit_import(
                            (self).handle() as i32,
                            ptr0.cast_mut(),
                            len0,
                            ptr1.cast_mut(),
                            len1,
                            ptr2,
                        );
                        let l3 = i32::from(*ptr2.add(0).cast::<u8>());
                        match l3 {
                            0 => {
                                let e = ();
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l4 = i32::from(*ptr2.add(1).cast::<u8>());
                                    ErrorCode::_lift(l4 as u8)
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl Descriptor {
                #[allow(unused_unsafe, clippy::all)]
                pub fn unlink_file_at(&self, path: &str) -> Result<(), ErrorCode> {
                    unsafe {
                        #[repr(align(1))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 2]);
                        let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 2]);
                        let vec0 = path;
                        let ptr0 = vec0.as_ptr().cast::<u8>();
                        let len0 = vec0.len();
                        let ptr1 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:filesystem/types@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]descriptor.unlink-file-at"]
                            fn wit_import(_: i32, _: *mut u8, _: usize, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: *mut u8, _: usize, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, ptr0.cast_mut(), len0, ptr1);
                        let l2 = i32::from(*ptr1.add(0).cast::<u8>());
                        match l2 {
                            0 => {
                                let e = ();
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l3 = i32::from(*ptr1.add(1).cast::<u8>());
                                    ErrorCode::_lift(l3 as u8)
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl Descriptor {
                #[allow(unused_unsafe, clippy::all)]
                pub fn is_same_object(&self, other: &Descriptor) -> bool {
                    unsafe {
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:filesystem/types@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]descriptor.is-same-object"]
                            fn wit_import(_: i32, _: i32) -> i32;
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: i32) -> i32 {
                            unreachable!()
                        }
                        let ret = wit_import((self).handle() as i32, (other).handle() as i32);
                        _rt::bool_lift(ret as u8)
                    }
                }
            }
            impl Descriptor {
                #[allow(unused_unsafe, clippy::all)]
                pub fn metadata_hash(&self) -> Result<MetadataHashValue, ErrorCode> {
                    unsafe {
                        #[repr(align(8))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 24]);
                        let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 24]);
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:filesystem/types@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]descriptor.metadata-hash"]
                            fn wit_import(_: i32, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, ptr0);
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        match l1 {
                            0 => {
                                let e = {
                                    let l2 = *ptr0.add(8).cast::<i64>();
                                    let l3 = *ptr0.add(16).cast::<i64>();
                                    MetadataHashValue { lower: l2 as u64, upper: l3 as u64 }
                                };
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l4 = i32::from(*ptr0.add(8).cast::<u8>());
                                    ErrorCode::_lift(l4 as u8)
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl Descriptor {
                #[allow(unused_unsafe, clippy::all)]
                pub fn metadata_hash_at(
                    &self,
                    path_flags: PathFlags,
                    path: &str,
                ) -> Result<MetadataHashValue, ErrorCode> {
                    unsafe {
                        #[repr(align(8))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 24]);
                        let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 24]);
                        let flags0 = path_flags;
                        let vec1 = path;
                        let ptr1 = vec1.as_ptr().cast::<u8>();
                        let len1 = vec1.len();
                        let ptr2 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:filesystem/types@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]descriptor.metadata-hash-at"]
                            fn wit_import(_: i32, _: i32, _: *mut u8, _: usize, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: i32, _: *mut u8, _: usize, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import(
                            (self).handle() as i32,
                            (flags0.bits() >> 0) as i32,
                            ptr1.cast_mut(),
                            len1,
                            ptr2,
                        );
                        let l3 = i32::from(*ptr2.add(0).cast::<u8>());
                        match l3 {
                            0 => {
                                let e = {
                                    let l4 = *ptr2.add(8).cast::<i64>();
                                    let l5 = *ptr2.add(16).cast::<i64>();
                                    MetadataHashValue { lower: l4 as u64, upper: l5 as u64 }
                                };
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l6 = i32::from(*ptr2.add(8).cast::<u8>());
                                    ErrorCode::_lift(l6 as u8)
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl DirectoryEntryStream {
                #[allow(unused_unsafe, clippy::all)]
                pub fn read_directory_entry(&self) -> Result<Option<DirectoryEntry>, ErrorCode> {
                    unsafe {
                        #[repr(align(4))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 20]);
                        let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 20]);
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:filesystem/types@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]directory-entry-stream.read-directory-entry"]
                            fn wit_import(_: i32, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, ptr0);
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        match l1 {
                            0 => {
                                let e = {
                                    let l2 = i32::from(*ptr0.add(4).cast::<u8>());
                                    match l2 {
                                        0 => None,
                                        1 => {
                                            let e = {
                                                let l3 = i32::from(*ptr0.add(8).cast::<u8>());
                                                let l4 = *ptr0.add(12).cast::<*mut u8>();
                                                let l5 = *ptr0.add(16).cast::<usize>();
                                                let len6 = l5;
                                                let bytes6 =
                                                    _rt::Vec::from_raw_parts(l4.cast(), len6, len6);
                                                DirectoryEntry {
                                                    type_: DescriptorType::_lift(l3 as u8),
                                                    name: _rt::string_lift(bytes6),
                                                }
                                            };
                                            Some(e)
                                        }
                                        _ => _rt::invalid_enum_discriminant(),
                                    }
                                };
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l7 = i32::from(*ptr0.add(4).cast::<u8>());
                                    ErrorCode::_lift(l7 as u8)
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            #[allow(unused_unsafe, clippy::all)]
            pub fn filesystem_error_code(err: &Error) -> Option<ErrorCode> {
                unsafe {
                    #[repr(align(1))]
                    struct RetArea([::core::mem::MaybeUninit<u8>; 2]);
                    let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 2]);
                    let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                    #[cfg(target_arch = "wasm32")]
                    #[link(wasm_import_module = "wasi:filesystem/types@0.2.0")]
                    extern "C" {
                        #[link_name = "filesystem-error-code"]
                        fn wit_import(_: i32, _: *mut u8);
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import(_: i32, _: *mut u8) {
                        unreachable!()
                    }
                    wit_import((err).handle() as i32, ptr0);
                    let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                    match l1 {
                        0 => None,
                        1 => {
                            let e = {
                                let l2 = i32::from(*ptr0.add(1).cast::<u8>());
                                ErrorCode::_lift(l2 as u8)
                            };
                            Some(e)
                        }
                        _ => _rt::invalid_enum_discriminant(),
                    }
                }
            }
        }
        #[allow(dead_code, clippy::all)]
        pub mod preopens {
            #[used]
            #[doc(hidden)]
            static __FORCE_SECTION_REF: fn() =
                super::super::super::__link_custom_section_describing_imports;
            use super::super::super::_rt;
            pub type Descriptor = super::super::super::wasi::filesystem::types::Descriptor;
            #[allow(unused_unsafe, clippy::all)]
            pub fn get_directories() -> _rt::Vec<(Descriptor, _rt::String)> {
                unsafe {
                    #[repr(align(4))]
                    struct RetArea([::core::mem::MaybeUninit<u8>; 8]);
                    let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 8]);
                    let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                    #[cfg(target_arch = "wasm32")]
                    #[link(wasm_import_module = "wasi:filesystem/preopens@0.2.0")]
                    extern "C" {
                        #[link_name = "get-directories"]
                        fn wit_import(_: *mut u8);
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import(_: *mut u8) {
                        unreachable!()
                    }
                    wit_import(ptr0);
                    let l1 = *ptr0.add(0).cast::<*mut u8>();
                    let l2 = *ptr0.add(4).cast::<usize>();
                    let base7 = l1;
                    let len7 = l2;
                    let mut result7 = _rt::Vec::with_capacity(len7);
                    for i in 0..len7 {
                        let base = base7.add(i * 12);
                        let e7 = {
                            let l3 = *base.add(0).cast::<i32>();
                            let l4 = *base.add(4).cast::<*mut u8>();
                            let l5 = *base.add(8).cast::<usize>();
                            let len6 = l5;
                            let bytes6 = _rt::Vec::from_raw_parts(l4.cast(), len6, len6);
                            (
                                super::super::super::wasi::filesystem::types::Descriptor::from_handle(
                                    l3 as u32,
                                ),
                                _rt::string_lift(bytes6),
                            )
                        };
                        result7.push(e7);
                    }
                    _rt::cabi_dealloc(base7, len7 * 12, 4);
                    result7
                }
            }
        }
    }
    #[allow(dead_code)]
    pub mod http {
        #[allow(dead_code, clippy::all)]
        pub mod types {
            #[used]
            #[doc(hidden)]
            static __FORCE_SECTION_REF: fn() =
                super::super::super::__link_custom_section_describing_imports;
            use super::super::super::_rt;
            pub type Duration = super::super::super::wasi::clocks::monotonic_clock::Duration;
            pub type InputStream = super::super::super::wasi::io::streams::InputStream;
            pub type OutputStream = super::super::super::wasi::io::streams::OutputStream;
            pub type IoError = super::super::super::wasi::io::error::Error;
            pub type Pollable = super::super::super::wasi::io::poll::Pollable;
            #[derive(Clone)]
            pub enum Method {
                Get,
                Head,
                Post,
                Put,
                Delete,
                Connect,
                Options,
                Trace,
                Patch,
                Other(_rt::String),
            }
            impl ::core::fmt::Debug for Method {
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    match self {
                        Method::Get => f.debug_tuple("Method::Get").finish(),
                        Method::Head => f.debug_tuple("Method::Head").finish(),
                        Method::Post => f.debug_tuple("Method::Post").finish(),
                        Method::Put => f.debug_tuple("Method::Put").finish(),
                        Method::Delete => f.debug_tuple("Method::Delete").finish(),
                        Method::Connect => f.debug_tuple("Method::Connect").finish(),
                        Method::Options => f.debug_tuple("Method::Options").finish(),
                        Method::Trace => f.debug_tuple("Method::Trace").finish(),
                        Method::Patch => f.debug_tuple("Method::Patch").finish(),
                        Method::Other(e) => f.debug_tuple("Method::Other").field(e).finish(),
                    }
                }
            }
            #[derive(Clone)]
            pub enum Scheme {
                Http,
                Https,
                Other(_rt::String),
            }
            impl ::core::fmt::Debug for Scheme {
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    match self {
                        Scheme::Http => f.debug_tuple("Scheme::Http").finish(),
                        Scheme::Https => f.debug_tuple("Scheme::Https").finish(),
                        Scheme::Other(e) => f.debug_tuple("Scheme::Other").field(e).finish(),
                    }
                }
            }
            #[derive(Clone)]
            pub struct DnsErrorPayload {
                pub rcode: Option<_rt::String>,
                pub info_code: Option<u16>,
            }
            impl ::core::fmt::Debug for DnsErrorPayload {
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    f.debug_struct("DnsErrorPayload")
                        .field("rcode", &self.rcode)
                        .field("info-code", &self.info_code)
                        .finish()
                }
            }
            #[derive(Clone)]
            pub struct TlsAlertReceivedPayload {
                pub alert_id: Option<u8>,
                pub alert_message: Option<_rt::String>,
            }
            impl ::core::fmt::Debug for TlsAlertReceivedPayload {
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    f.debug_struct("TlsAlertReceivedPayload")
                        .field("alert-id", &self.alert_id)
                        .field("alert-message", &self.alert_message)
                        .finish()
                }
            }
            #[derive(Clone)]
            pub struct FieldSizePayload {
                pub field_name: Option<_rt::String>,
                pub field_size: Option<u32>,
            }
            impl ::core::fmt::Debug for FieldSizePayload {
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    f.debug_struct("FieldSizePayload")
                        .field("field-name", &self.field_name)
                        .field("field-size", &self.field_size)
                        .finish()
                }
            }
            #[derive(Clone)]
            pub enum ErrorCode {
                DnsTimeout,
                DnsError(DnsErrorPayload),
                DestinationNotFound,
                DestinationUnavailable,
                DestinationIpProhibited,
                DestinationIpUnroutable,
                ConnectionRefused,
                ConnectionTerminated,
                ConnectionTimeout,
                ConnectionReadTimeout,
                ConnectionWriteTimeout,
                ConnectionLimitReached,
                TlsProtocolError,
                TlsCertificateError,
                TlsAlertReceived(TlsAlertReceivedPayload),
                HttpRequestDenied,
                HttpRequestLengthRequired,
                HttpRequestBodySize(Option<u64>),
                HttpRequestMethodInvalid,
                HttpRequestUriInvalid,
                HttpRequestUriTooLong,
                HttpRequestHeaderSectionSize(Option<u32>),
                HttpRequestHeaderSize(Option<FieldSizePayload>),
                HttpRequestTrailerSectionSize(Option<u32>),
                HttpRequestTrailerSize(FieldSizePayload),
                HttpResponseIncomplete,
                HttpResponseHeaderSectionSize(Option<u32>),
                HttpResponseHeaderSize(FieldSizePayload),
                HttpResponseBodySize(Option<u64>),
                HttpResponseTrailerSectionSize(Option<u32>),
                HttpResponseTrailerSize(FieldSizePayload),
                HttpResponseTransferCoding(Option<_rt::String>),
                HttpResponseContentCoding(Option<_rt::String>),
                HttpResponseTimeout,
                HttpUpgradeFailed,
                HttpProtocolError,
                LoopDetected,
                ConfigurationError,
                InternalError(Option<_rt::String>),
            }
            impl ::core::fmt::Debug for ErrorCode {
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    match self {
                        ErrorCode::DnsTimeout => f.debug_tuple("ErrorCode::DnsTimeout").finish(),
                        ErrorCode::DnsError(e) => {
                            f.debug_tuple("ErrorCode::DnsError").field(e).finish()
                        }
                        ErrorCode::DestinationNotFound => {
                            f.debug_tuple("ErrorCode::DestinationNotFound").finish()
                        }
                        ErrorCode::DestinationUnavailable => {
                            f.debug_tuple("ErrorCode::DestinationUnavailable").finish()
                        }
                        ErrorCode::DestinationIpProhibited => {
                            f.debug_tuple("ErrorCode::DestinationIpProhibited").finish()
                        }
                        ErrorCode::DestinationIpUnroutable => {
                            f.debug_tuple("ErrorCode::DestinationIpUnroutable").finish()
                        }
                        ErrorCode::ConnectionRefused => {
                            f.debug_tuple("ErrorCode::ConnectionRefused").finish()
                        }
                        ErrorCode::ConnectionTerminated => {
                            f.debug_tuple("ErrorCode::ConnectionTerminated").finish()
                        }
                        ErrorCode::ConnectionTimeout => {
                            f.debug_tuple("ErrorCode::ConnectionTimeout").finish()
                        }
                        ErrorCode::ConnectionReadTimeout => {
                            f.debug_tuple("ErrorCode::ConnectionReadTimeout").finish()
                        }
                        ErrorCode::ConnectionWriteTimeout => {
                            f.debug_tuple("ErrorCode::ConnectionWriteTimeout").finish()
                        }
                        ErrorCode::ConnectionLimitReached => {
                            f.debug_tuple("ErrorCode::ConnectionLimitReached").finish()
                        }
                        ErrorCode::TlsProtocolError => {
                            f.debug_tuple("ErrorCode::TlsProtocolError").finish()
                        }
                        ErrorCode::TlsCertificateError => {
                            f.debug_tuple("ErrorCode::TlsCertificateError").finish()
                        }
                        ErrorCode::TlsAlertReceived(e) => {
                            f.debug_tuple("ErrorCode::TlsAlertReceived").field(e).finish()
                        }
                        ErrorCode::HttpRequestDenied => {
                            f.debug_tuple("ErrorCode::HttpRequestDenied").finish()
                        }
                        ErrorCode::HttpRequestLengthRequired => {
                            f.debug_tuple("ErrorCode::HttpRequestLengthRequired").finish()
                        }
                        ErrorCode::HttpRequestBodySize(e) => {
                            f.debug_tuple("ErrorCode::HttpRequestBodySize").field(e).finish()
                        }
                        ErrorCode::HttpRequestMethodInvalid => {
                            f.debug_tuple("ErrorCode::HttpRequestMethodInvalid").finish()
                        }
                        ErrorCode::HttpRequestUriInvalid => {
                            f.debug_tuple("ErrorCode::HttpRequestUriInvalid").finish()
                        }
                        ErrorCode::HttpRequestUriTooLong => {
                            f.debug_tuple("ErrorCode::HttpRequestUriTooLong").finish()
                        }
                        ErrorCode::HttpRequestHeaderSectionSize(e) => f
                            .debug_tuple("ErrorCode::HttpRequestHeaderSectionSize")
                            .field(e)
                            .finish(),
                        ErrorCode::HttpRequestHeaderSize(e) => {
                            f.debug_tuple("ErrorCode::HttpRequestHeaderSize").field(e).finish()
                        }
                        ErrorCode::HttpRequestTrailerSectionSize(e) => f
                            .debug_tuple("ErrorCode::HttpRequestTrailerSectionSize")
                            .field(e)
                            .finish(),
                        ErrorCode::HttpRequestTrailerSize(e) => {
                            f.debug_tuple("ErrorCode::HttpRequestTrailerSize").field(e).finish()
                        }
                        ErrorCode::HttpResponseIncomplete => {
                            f.debug_tuple("ErrorCode::HttpResponseIncomplete").finish()
                        }
                        ErrorCode::HttpResponseHeaderSectionSize(e) => f
                            .debug_tuple("ErrorCode::HttpResponseHeaderSectionSize")
                            .field(e)
                            .finish(),
                        ErrorCode::HttpResponseHeaderSize(e) => {
                            f.debug_tuple("ErrorCode::HttpResponseHeaderSize").field(e).finish()
                        }
                        ErrorCode::HttpResponseBodySize(e) => {
                            f.debug_tuple("ErrorCode::HttpResponseBodySize").field(e).finish()
                        }
                        ErrorCode::HttpResponseTrailerSectionSize(e) => f
                            .debug_tuple("ErrorCode::HttpResponseTrailerSectionSize")
                            .field(e)
                            .finish(),
                        ErrorCode::HttpResponseTrailerSize(e) => {
                            f.debug_tuple("ErrorCode::HttpResponseTrailerSize").field(e).finish()
                        }
                        ErrorCode::HttpResponseTransferCoding(e) => {
                            f.debug_tuple("ErrorCode::HttpResponseTransferCoding").field(e).finish()
                        }
                        ErrorCode::HttpResponseContentCoding(e) => {
                            f.debug_tuple("ErrorCode::HttpResponseContentCoding").field(e).finish()
                        }
                        ErrorCode::HttpResponseTimeout => {
                            f.debug_tuple("ErrorCode::HttpResponseTimeout").finish()
                        }
                        ErrorCode::HttpUpgradeFailed => {
                            f.debug_tuple("ErrorCode::HttpUpgradeFailed").finish()
                        }
                        ErrorCode::HttpProtocolError => {
                            f.debug_tuple("ErrorCode::HttpProtocolError").finish()
                        }
                        ErrorCode::LoopDetected => {
                            f.debug_tuple("ErrorCode::LoopDetected").finish()
                        }
                        ErrorCode::ConfigurationError => {
                            f.debug_tuple("ErrorCode::ConfigurationError").finish()
                        }
                        ErrorCode::InternalError(e) => {
                            f.debug_tuple("ErrorCode::InternalError").field(e).finish()
                        }
                    }
                }
            }
            impl ::core::fmt::Display for ErrorCode {
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    write!(f, "{:?}", self)
                }
            }
            impl std::error::Error for ErrorCode {}
            #[derive(Clone, Copy)]
            pub enum HeaderError {
                InvalidSyntax,
                Forbidden,
                Immutable,
            }
            impl ::core::fmt::Debug for HeaderError {
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    match self {
                        HeaderError::InvalidSyntax => {
                            f.debug_tuple("HeaderError::InvalidSyntax").finish()
                        }
                        HeaderError::Forbidden => f.debug_tuple("HeaderError::Forbidden").finish(),
                        HeaderError::Immutable => f.debug_tuple("HeaderError::Immutable").finish(),
                    }
                }
            }
            impl ::core::fmt::Display for HeaderError {
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    write!(f, "{:?}", self)
                }
            }
            impl std::error::Error for HeaderError {}
            pub type FieldKey = _rt::String;
            pub type FieldValue = _rt::Vec<u8>;
            #[derive(Debug)]
            #[repr(transparent)]
            pub struct Fields {
                handle: _rt::Resource<Fields>,
            }
            impl Fields {
                #[doc(hidden)]
                pub unsafe fn from_handle(handle: u32) -> Self {
                    Self { handle: _rt::Resource::from_handle(handle) }
                }
                #[doc(hidden)]
                pub fn take_handle(&self) -> u32 {
                    _rt::Resource::take_handle(&self.handle)
                }
                #[doc(hidden)]
                pub fn handle(&self) -> u32 {
                    _rt::Resource::handle(&self.handle)
                }
            }
            unsafe impl _rt::WasmResource for Fields {
                #[inline]
                unsafe fn drop(_handle: u32) {
                    #[cfg(not(target_arch = "wasm32"))]
                    unreachable!();
                    #[cfg(target_arch = "wasm32")]
                    {
                        #[link(wasm_import_module = "wasi:http/types@0.2.0")]
                        extern "C" {
                            #[link_name = "[resource-drop]fields"]
                            fn drop(_: u32);
                        }
                        drop(_handle);
                    }
                }
            }
            pub type Headers = Fields;
            pub type Trailers = Fields;
            #[derive(Debug)]
            #[repr(transparent)]
            pub struct IncomingRequest {
                handle: _rt::Resource<IncomingRequest>,
            }
            impl IncomingRequest {
                #[doc(hidden)]
                pub unsafe fn from_handle(handle: u32) -> Self {
                    Self { handle: _rt::Resource::from_handle(handle) }
                }
                #[doc(hidden)]
                pub fn take_handle(&self) -> u32 {
                    _rt::Resource::take_handle(&self.handle)
                }
                #[doc(hidden)]
                pub fn handle(&self) -> u32 {
                    _rt::Resource::handle(&self.handle)
                }
            }
            unsafe impl _rt::WasmResource for IncomingRequest {
                #[inline]
                unsafe fn drop(_handle: u32) {
                    #[cfg(not(target_arch = "wasm32"))]
                    unreachable!();
                    #[cfg(target_arch = "wasm32")]
                    {
                        #[link(wasm_import_module = "wasi:http/types@0.2.0")]
                        extern "C" {
                            #[link_name = "[resource-drop]incoming-request"]
                            fn drop(_: u32);
                        }
                        drop(_handle);
                    }
                }
            }
            #[derive(Debug)]
            #[repr(transparent)]
            pub struct OutgoingRequest {
                handle: _rt::Resource<OutgoingRequest>,
            }
            impl OutgoingRequest {
                #[doc(hidden)]
                pub unsafe fn from_handle(handle: u32) -> Self {
                    Self { handle: _rt::Resource::from_handle(handle) }
                }
                #[doc(hidden)]
                pub fn take_handle(&self) -> u32 {
                    _rt::Resource::take_handle(&self.handle)
                }
                #[doc(hidden)]
                pub fn handle(&self) -> u32 {
                    _rt::Resource::handle(&self.handle)
                }
            }
            unsafe impl _rt::WasmResource for OutgoingRequest {
                #[inline]
                unsafe fn drop(_handle: u32) {
                    #[cfg(not(target_arch = "wasm32"))]
                    unreachable!();
                    #[cfg(target_arch = "wasm32")]
                    {
                        #[link(wasm_import_module = "wasi:http/types@0.2.0")]
                        extern "C" {
                            #[link_name = "[resource-drop]outgoing-request"]
                            fn drop(_: u32);
                        }
                        drop(_handle);
                    }
                }
            }
            #[derive(Debug)]
            #[repr(transparent)]
            pub struct RequestOptions {
                handle: _rt::Resource<RequestOptions>,
            }
            impl RequestOptions {
                #[doc(hidden)]
                pub unsafe fn from_handle(handle: u32) -> Self {
                    Self { handle: _rt::Resource::from_handle(handle) }
                }
                #[doc(hidden)]
                pub fn take_handle(&self) -> u32 {
                    _rt::Resource::take_handle(&self.handle)
                }
                #[doc(hidden)]
                pub fn handle(&self) -> u32 {
                    _rt::Resource::handle(&self.handle)
                }
            }
            unsafe impl _rt::WasmResource for RequestOptions {
                #[inline]
                unsafe fn drop(_handle: u32) {
                    #[cfg(not(target_arch = "wasm32"))]
                    unreachable!();
                    #[cfg(target_arch = "wasm32")]
                    {
                        #[link(wasm_import_module = "wasi:http/types@0.2.0")]
                        extern "C" {
                            #[link_name = "[resource-drop]request-options"]
                            fn drop(_: u32);
                        }
                        drop(_handle);
                    }
                }
            }
            #[derive(Debug)]
            #[repr(transparent)]
            pub struct ResponseOutparam {
                handle: _rt::Resource<ResponseOutparam>,
            }
            impl ResponseOutparam {
                #[doc(hidden)]
                pub unsafe fn from_handle(handle: u32) -> Self {
                    Self { handle: _rt::Resource::from_handle(handle) }
                }
                #[doc(hidden)]
                pub fn take_handle(&self) -> u32 {
                    _rt::Resource::take_handle(&self.handle)
                }
                #[doc(hidden)]
                pub fn handle(&self) -> u32 {
                    _rt::Resource::handle(&self.handle)
                }
            }
            unsafe impl _rt::WasmResource for ResponseOutparam {
                #[inline]
                unsafe fn drop(_handle: u32) {
                    #[cfg(not(target_arch = "wasm32"))]
                    unreachable!();
                    #[cfg(target_arch = "wasm32")]
                    {
                        #[link(wasm_import_module = "wasi:http/types@0.2.0")]
                        extern "C" {
                            #[link_name = "[resource-drop]response-outparam"]
                            fn drop(_: u32);
                        }
                        drop(_handle);
                    }
                }
            }
            pub type StatusCode = u16;
            #[derive(Debug)]
            #[repr(transparent)]
            pub struct IncomingResponse {
                handle: _rt::Resource<IncomingResponse>,
            }
            impl IncomingResponse {
                #[doc(hidden)]
                pub unsafe fn from_handle(handle: u32) -> Self {
                    Self { handle: _rt::Resource::from_handle(handle) }
                }
                #[doc(hidden)]
                pub fn take_handle(&self) -> u32 {
                    _rt::Resource::take_handle(&self.handle)
                }
                #[doc(hidden)]
                pub fn handle(&self) -> u32 {
                    _rt::Resource::handle(&self.handle)
                }
            }
            unsafe impl _rt::WasmResource for IncomingResponse {
                #[inline]
                unsafe fn drop(_handle: u32) {
                    #[cfg(not(target_arch = "wasm32"))]
                    unreachable!();
                    #[cfg(target_arch = "wasm32")]
                    {
                        #[link(wasm_import_module = "wasi:http/types@0.2.0")]
                        extern "C" {
                            #[link_name = "[resource-drop]incoming-response"]
                            fn drop(_: u32);
                        }
                        drop(_handle);
                    }
                }
            }
            #[derive(Debug)]
            #[repr(transparent)]
            pub struct IncomingBody {
                handle: _rt::Resource<IncomingBody>,
            }
            impl IncomingBody {
                #[doc(hidden)]
                pub unsafe fn from_handle(handle: u32) -> Self {
                    Self { handle: _rt::Resource::from_handle(handle) }
                }
                #[doc(hidden)]
                pub fn take_handle(&self) -> u32 {
                    _rt::Resource::take_handle(&self.handle)
                }
                #[doc(hidden)]
                pub fn handle(&self) -> u32 {
                    _rt::Resource::handle(&self.handle)
                }
            }
            unsafe impl _rt::WasmResource for IncomingBody {
                #[inline]
                unsafe fn drop(_handle: u32) {
                    #[cfg(not(target_arch = "wasm32"))]
                    unreachable!();
                    #[cfg(target_arch = "wasm32")]
                    {
                        #[link(wasm_import_module = "wasi:http/types@0.2.0")]
                        extern "C" {
                            #[link_name = "[resource-drop]incoming-body"]
                            fn drop(_: u32);
                        }
                        drop(_handle);
                    }
                }
            }
            #[derive(Debug)]
            #[repr(transparent)]
            pub struct FutureTrailers {
                handle: _rt::Resource<FutureTrailers>,
            }
            impl FutureTrailers {
                #[doc(hidden)]
                pub unsafe fn from_handle(handle: u32) -> Self {
                    Self { handle: _rt::Resource::from_handle(handle) }
                }
                #[doc(hidden)]
                pub fn take_handle(&self) -> u32 {
                    _rt::Resource::take_handle(&self.handle)
                }
                #[doc(hidden)]
                pub fn handle(&self) -> u32 {
                    _rt::Resource::handle(&self.handle)
                }
            }
            unsafe impl _rt::WasmResource for FutureTrailers {
                #[inline]
                unsafe fn drop(_handle: u32) {
                    #[cfg(not(target_arch = "wasm32"))]
                    unreachable!();
                    #[cfg(target_arch = "wasm32")]
                    {
                        #[link(wasm_import_module = "wasi:http/types@0.2.0")]
                        extern "C" {
                            #[link_name = "[resource-drop]future-trailers"]
                            fn drop(_: u32);
                        }
                        drop(_handle);
                    }
                }
            }
            #[derive(Debug)]
            #[repr(transparent)]
            pub struct OutgoingResponse {
                handle: _rt::Resource<OutgoingResponse>,
            }
            impl OutgoingResponse {
                #[doc(hidden)]
                pub unsafe fn from_handle(handle: u32) -> Self {
                    Self { handle: _rt::Resource::from_handle(handle) }
                }
                #[doc(hidden)]
                pub fn take_handle(&self) -> u32 {
                    _rt::Resource::take_handle(&self.handle)
                }
                #[doc(hidden)]
                pub fn handle(&self) -> u32 {
                    _rt::Resource::handle(&self.handle)
                }
            }
            unsafe impl _rt::WasmResource for OutgoingResponse {
                #[inline]
                unsafe fn drop(_handle: u32) {
                    #[cfg(not(target_arch = "wasm32"))]
                    unreachable!();
                    #[cfg(target_arch = "wasm32")]
                    {
                        #[link(wasm_import_module = "wasi:http/types@0.2.0")]
                        extern "C" {
                            #[link_name = "[resource-drop]outgoing-response"]
                            fn drop(_: u32);
                        }
                        drop(_handle);
                    }
                }
            }
            #[derive(Debug)]
            #[repr(transparent)]
            pub struct OutgoingBody {
                handle: _rt::Resource<OutgoingBody>,
            }
            impl OutgoingBody {
                #[doc(hidden)]
                pub unsafe fn from_handle(handle: u32) -> Self {
                    Self { handle: _rt::Resource::from_handle(handle) }
                }
                #[doc(hidden)]
                pub fn take_handle(&self) -> u32 {
                    _rt::Resource::take_handle(&self.handle)
                }
                #[doc(hidden)]
                pub fn handle(&self) -> u32 {
                    _rt::Resource::handle(&self.handle)
                }
            }
            unsafe impl _rt::WasmResource for OutgoingBody {
                #[inline]
                unsafe fn drop(_handle: u32) {
                    #[cfg(not(target_arch = "wasm32"))]
                    unreachable!();
                    #[cfg(target_arch = "wasm32")]
                    {
                        #[link(wasm_import_module = "wasi:http/types@0.2.0")]
                        extern "C" {
                            #[link_name = "[resource-drop]outgoing-body"]
                            fn drop(_: u32);
                        }
                        drop(_handle);
                    }
                }
            }
            #[derive(Debug)]
            #[repr(transparent)]
            pub struct FutureIncomingResponse {
                handle: _rt::Resource<FutureIncomingResponse>,
            }
            impl FutureIncomingResponse {
                #[doc(hidden)]
                pub unsafe fn from_handle(handle: u32) -> Self {
                    Self { handle: _rt::Resource::from_handle(handle) }
                }
                #[doc(hidden)]
                pub fn take_handle(&self) -> u32 {
                    _rt::Resource::take_handle(&self.handle)
                }
                #[doc(hidden)]
                pub fn handle(&self) -> u32 {
                    _rt::Resource::handle(&self.handle)
                }
            }
            unsafe impl _rt::WasmResource for FutureIncomingResponse {
                #[inline]
                unsafe fn drop(_handle: u32) {
                    #[cfg(not(target_arch = "wasm32"))]
                    unreachable!();
                    #[cfg(target_arch = "wasm32")]
                    {
                        #[link(wasm_import_module = "wasi:http/types@0.2.0")]
                        extern "C" {
                            #[link_name = "[resource-drop]future-incoming-response"]
                            fn drop(_: u32);
                        }
                        drop(_handle);
                    }
                }
            }
            impl Fields {
                #[allow(unused_unsafe, clippy::all)]
                pub fn new() -> Self {
                    unsafe {
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:http/types@0.2.0")]
                        extern "C" {
                            #[link_name = "[constructor]fields"]
                            fn wit_import() -> i32;
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import() -> i32 {
                            unreachable!()
                        }
                        let ret = wit_import();
                        Fields::from_handle(ret as u32)
                    }
                }
            }
            impl Fields {
                #[allow(unused_unsafe, clippy::all)]
                pub fn from_list(
                    entries: &[(FieldKey, FieldValue)],
                ) -> Result<Fields, HeaderError> {
                    unsafe {
                        #[repr(align(4))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 8]);
                        let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 8]);
                        let vec3 = entries;
                        let len3 = vec3.len();
                        let layout3 =
                            _rt::alloc::Layout::from_size_align_unchecked(vec3.len() * 16, 4);
                        let result3 = if layout3.size() != 0 {
                            let ptr = _rt::alloc::alloc(layout3).cast::<u8>();
                            if ptr.is_null() {
                                _rt::alloc::handle_alloc_error(layout3);
                            }
                            ptr
                        } else {
                            ::core::ptr::null_mut()
                        };
                        for (i, e) in vec3.into_iter().enumerate() {
                            let base = result3.add(i * 16);
                            {
                                let (t0_0, t0_1) = e;
                                let vec1 = t0_0;
                                let ptr1 = vec1.as_ptr().cast::<u8>();
                                let len1 = vec1.len();
                                *base.add(4).cast::<usize>() = len1;
                                *base.add(0).cast::<*mut u8>() = ptr1.cast_mut();
                                let vec2 = t0_1;
                                let ptr2 = vec2.as_ptr().cast::<u8>();
                                let len2 = vec2.len();
                                *base.add(12).cast::<usize>() = len2;
                                *base.add(8).cast::<*mut u8>() = ptr2.cast_mut();
                            }
                        }
                        let ptr4 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:http/types@0.2.0")]
                        extern "C" {
                            #[link_name = "[static]fields.from-list"]
                            fn wit_import(_: *mut u8, _: usize, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: *mut u8, _: usize, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import(result3, len3, ptr4);
                        let l5 = i32::from(*ptr4.add(0).cast::<u8>());
                        if layout3.size() != 0 {
                            _rt::alloc::dealloc(result3.cast(), layout3);
                        }
                        match l5 {
                            0 => {
                                let e = {
                                    let l6 = *ptr4.add(4).cast::<i32>();
                                    Fields::from_handle(l6 as u32)
                                };
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l7 = i32::from(*ptr4.add(4).cast::<u8>());
                                    let v8 = match l7 {
                                        0 => HeaderError::InvalidSyntax,
                                        1 => HeaderError::Forbidden,
                                        n => {
                                            debug_assert_eq!(n, 2, "invalid enum discriminant");
                                            HeaderError::Immutable
                                        }
                                    };
                                    v8
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl Fields {
                #[allow(unused_unsafe, clippy::all)]
                pub fn get(&self, name: &FieldKey) -> _rt::Vec<FieldValue> {
                    unsafe {
                        #[repr(align(4))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 8]);
                        let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 8]);
                        let vec0 = name;
                        let ptr0 = vec0.as_ptr().cast::<u8>();
                        let len0 = vec0.len();
                        let ptr1 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:http/types@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]fields.get"]
                            fn wit_import(_: i32, _: *mut u8, _: usize, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: *mut u8, _: usize, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, ptr0.cast_mut(), len0, ptr1);
                        let l2 = *ptr1.add(0).cast::<*mut u8>();
                        let l3 = *ptr1.add(4).cast::<usize>();
                        let base7 = l2;
                        let len7 = l3;
                        let mut result7 = _rt::Vec::with_capacity(len7);
                        for i in 0..len7 {
                            let base = base7.add(i * 8);
                            let e7 = {
                                let l4 = *base.add(0).cast::<*mut u8>();
                                let l5 = *base.add(4).cast::<usize>();
                                let len6 = l5;
                                _rt::Vec::from_raw_parts(l4.cast(), len6, len6)
                            };
                            result7.push(e7);
                        }
                        _rt::cabi_dealloc(base7, len7 * 8, 4);
                        result7
                    }
                }
            }
            impl Fields {
                #[allow(unused_unsafe, clippy::all)]
                pub fn has(&self, name: &FieldKey) -> bool {
                    unsafe {
                        let vec0 = name;
                        let ptr0 = vec0.as_ptr().cast::<u8>();
                        let len0 = vec0.len();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:http/types@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]fields.has"]
                            fn wit_import(_: i32, _: *mut u8, _: usize) -> i32;
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: *mut u8, _: usize) -> i32 {
                            unreachable!()
                        }
                        let ret = wit_import((self).handle() as i32, ptr0.cast_mut(), len0);
                        _rt::bool_lift(ret as u8)
                    }
                }
            }
            impl Fields {
                #[allow(unused_unsafe, clippy::all)]
                pub fn set(
                    &self,
                    name: &FieldKey,
                    value: &[FieldValue],
                ) -> Result<(), HeaderError> {
                    unsafe {
                        #[repr(align(1))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 2]);
                        let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 2]);
                        let vec0 = name;
                        let ptr0 = vec0.as_ptr().cast::<u8>();
                        let len0 = vec0.len();
                        let vec2 = value;
                        let len2 = vec2.len();
                        let layout2 =
                            _rt::alloc::Layout::from_size_align_unchecked(vec2.len() * 8, 4);
                        let result2 = if layout2.size() != 0 {
                            let ptr = _rt::alloc::alloc(layout2).cast::<u8>();
                            if ptr.is_null() {
                                _rt::alloc::handle_alloc_error(layout2);
                            }
                            ptr
                        } else {
                            ::core::ptr::null_mut()
                        };
                        for (i, e) in vec2.into_iter().enumerate() {
                            let base = result2.add(i * 8);
                            {
                                let vec1 = e;
                                let ptr1 = vec1.as_ptr().cast::<u8>();
                                let len1 = vec1.len();
                                *base.add(4).cast::<usize>() = len1;
                                *base.add(0).cast::<*mut u8>() = ptr1.cast_mut();
                            }
                        }
                        let ptr3 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:http/types@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]fields.set"]
                            fn wit_import(
                                _: i32,
                                _: *mut u8,
                                _: usize,
                                _: *mut u8,
                                _: usize,
                                _: *mut u8,
                            );
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(
                            _: i32,
                            _: *mut u8,
                            _: usize,
                            _: *mut u8,
                            _: usize,
                            _: *mut u8,
                        ) {
                            unreachable!()
                        }
                        wit_import(
                            (self).handle() as i32,
                            ptr0.cast_mut(),
                            len0,
                            result2,
                            len2,
                            ptr3,
                        );
                        let l4 = i32::from(*ptr3.add(0).cast::<u8>());
                        if layout2.size() != 0 {
                            _rt::alloc::dealloc(result2.cast(), layout2);
                        }
                        match l4 {
                            0 => {
                                let e = ();
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l5 = i32::from(*ptr3.add(1).cast::<u8>());
                                    let v6 = match l5 {
                                        0 => HeaderError::InvalidSyntax,
                                        1 => HeaderError::Forbidden,
                                        n => {
                                            debug_assert_eq!(n, 2, "invalid enum discriminant");
                                            HeaderError::Immutable
                                        }
                                    };
                                    v6
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl Fields {
                #[allow(unused_unsafe, clippy::all)]
                pub fn delete(&self, name: &FieldKey) -> Result<(), HeaderError> {
                    unsafe {
                        #[repr(align(1))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 2]);
                        let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 2]);
                        let vec0 = name;
                        let ptr0 = vec0.as_ptr().cast::<u8>();
                        let len0 = vec0.len();
                        let ptr1 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:http/types@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]fields.delete"]
                            fn wit_import(_: i32, _: *mut u8, _: usize, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: *mut u8, _: usize, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, ptr0.cast_mut(), len0, ptr1);
                        let l2 = i32::from(*ptr1.add(0).cast::<u8>());
                        match l2 {
                            0 => {
                                let e = ();
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l3 = i32::from(*ptr1.add(1).cast::<u8>());
                                    let v4 = match l3 {
                                        0 => HeaderError::InvalidSyntax,
                                        1 => HeaderError::Forbidden,
                                        n => {
                                            debug_assert_eq!(n, 2, "invalid enum discriminant");
                                            HeaderError::Immutable
                                        }
                                    };
                                    v4
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl Fields {
                #[allow(unused_unsafe, clippy::all)]
                pub fn append(
                    &self,
                    name: &FieldKey,
                    value: &FieldValue,
                ) -> Result<(), HeaderError> {
                    unsafe {
                        #[repr(align(1))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 2]);
                        let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 2]);
                        let vec0 = name;
                        let ptr0 = vec0.as_ptr().cast::<u8>();
                        let len0 = vec0.len();
                        let vec1 = value;
                        let ptr1 = vec1.as_ptr().cast::<u8>();
                        let len1 = vec1.len();
                        let ptr2 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:http/types@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]fields.append"]
                            fn wit_import(
                                _: i32,
                                _: *mut u8,
                                _: usize,
                                _: *mut u8,
                                _: usize,
                                _: *mut u8,
                            );
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(
                            _: i32,
                            _: *mut u8,
                            _: usize,
                            _: *mut u8,
                            _: usize,
                            _: *mut u8,
                        ) {
                            unreachable!()
                        }
                        wit_import(
                            (self).handle() as i32,
                            ptr0.cast_mut(),
                            len0,
                            ptr1.cast_mut(),
                            len1,
                            ptr2,
                        );
                        let l3 = i32::from(*ptr2.add(0).cast::<u8>());
                        match l3 {
                            0 => {
                                let e = ();
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l4 = i32::from(*ptr2.add(1).cast::<u8>());
                                    let v5 = match l4 {
                                        0 => HeaderError::InvalidSyntax,
                                        1 => HeaderError::Forbidden,
                                        n => {
                                            debug_assert_eq!(n, 2, "invalid enum discriminant");
                                            HeaderError::Immutable
                                        }
                                    };
                                    v5
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl Fields {
                #[allow(unused_unsafe, clippy::all)]
                pub fn entries(&self) -> _rt::Vec<(FieldKey, FieldValue)> {
                    unsafe {
                        #[repr(align(4))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 8]);
                        let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 8]);
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:http/types@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]fields.entries"]
                            fn wit_import(_: i32, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, ptr0);
                        let l1 = *ptr0.add(0).cast::<*mut u8>();
                        let l2 = *ptr0.add(4).cast::<usize>();
                        let base9 = l1;
                        let len9 = l2;
                        let mut result9 = _rt::Vec::with_capacity(len9);
                        for i in 0..len9 {
                            let base = base9.add(i * 16);
                            let e9 = {
                                let l3 = *base.add(0).cast::<*mut u8>();
                                let l4 = *base.add(4).cast::<usize>();
                                let len5 = l4;
                                let bytes5 = _rt::Vec::from_raw_parts(l3.cast(), len5, len5);
                                let l6 = *base.add(8).cast::<*mut u8>();
                                let l7 = *base.add(12).cast::<usize>();
                                let len8 = l7;
                                (
                                    _rt::string_lift(bytes5),
                                    _rt::Vec::from_raw_parts(l6.cast(), len8, len8),
                                )
                            };
                            result9.push(e9);
                        }
                        _rt::cabi_dealloc(base9, len9 * 16, 4);
                        result9
                    }
                }
            }
            impl Fields {
                #[allow(unused_unsafe, clippy::all)]
                pub fn clone(&self) -> Fields {
                    unsafe {
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:http/types@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]fields.clone"]
                            fn wit_import(_: i32) -> i32;
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32) -> i32 {
                            unreachable!()
                        }
                        let ret = wit_import((self).handle() as i32);
                        Fields::from_handle(ret as u32)
                    }
                }
            }
            impl IncomingRequest {
                #[allow(unused_unsafe, clippy::all)]
                pub fn method(&self) -> Method {
                    unsafe {
                        #[repr(align(4))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 12]);
                        let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 12]);
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:http/types@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]incoming-request.method"]
                            fn wit_import(_: i32, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, ptr0);
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        let v5 = match l1 {
                            0 => Method::Get,
                            1 => Method::Head,
                            2 => Method::Post,
                            3 => Method::Put,
                            4 => Method::Delete,
                            5 => Method::Connect,
                            6 => Method::Options,
                            7 => Method::Trace,
                            8 => Method::Patch,
                            n => {
                                debug_assert_eq!(n, 9, "invalid enum discriminant");
                                let e5 = {
                                    let l2 = *ptr0.add(4).cast::<*mut u8>();
                                    let l3 = *ptr0.add(8).cast::<usize>();
                                    let len4 = l3;
                                    let bytes4 = _rt::Vec::from_raw_parts(l2.cast(), len4, len4);
                                    _rt::string_lift(bytes4)
                                };
                                Method::Other(e5)
                            }
                        };
                        v5
                    }
                }
            }
            impl IncomingRequest {
                #[allow(unused_unsafe, clippy::all)]
                pub fn path_with_query(&self) -> Option<_rt::String> {
                    unsafe {
                        #[repr(align(4))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 12]);
                        let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 12]);
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:http/types@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]incoming-request.path-with-query"]
                            fn wit_import(_: i32, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, ptr0);
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        match l1 {
                            0 => None,
                            1 => {
                                let e = {
                                    let l2 = *ptr0.add(4).cast::<*mut u8>();
                                    let l3 = *ptr0.add(8).cast::<usize>();
                                    let len4 = l3;
                                    let bytes4 = _rt::Vec::from_raw_parts(l2.cast(), len4, len4);
                                    _rt::string_lift(bytes4)
                                };
                                Some(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl IncomingRequest {
                #[allow(unused_unsafe, clippy::all)]
                pub fn scheme(&self) -> Option<Scheme> {
                    unsafe {
                        #[repr(align(4))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 16]);
                        let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 16]);
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:http/types@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]incoming-request.scheme"]
                            fn wit_import(_: i32, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, ptr0);
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        match l1 {
                            0 => None,
                            1 => {
                                let e = {
                                    let l2 = i32::from(*ptr0.add(4).cast::<u8>());
                                    let v6 = match l2 {
                                        0 => Scheme::Http,
                                        1 => Scheme::Https,
                                        n => {
                                            debug_assert_eq!(n, 2, "invalid enum discriminant");
                                            let e6 = {
                                                let l3 = *ptr0.add(8).cast::<*mut u8>();
                                                let l4 = *ptr0.add(12).cast::<usize>();
                                                let len5 = l4;
                                                let bytes5 =
                                                    _rt::Vec::from_raw_parts(l3.cast(), len5, len5);
                                                _rt::string_lift(bytes5)
                                            };
                                            Scheme::Other(e6)
                                        }
                                    };
                                    v6
                                };
                                Some(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl IncomingRequest {
                #[allow(unused_unsafe, clippy::all)]
                pub fn authority(&self) -> Option<_rt::String> {
                    unsafe {
                        #[repr(align(4))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 12]);
                        let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 12]);
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:http/types@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]incoming-request.authority"]
                            fn wit_import(_: i32, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, ptr0);
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        match l1 {
                            0 => None,
                            1 => {
                                let e = {
                                    let l2 = *ptr0.add(4).cast::<*mut u8>();
                                    let l3 = *ptr0.add(8).cast::<usize>();
                                    let len4 = l3;
                                    let bytes4 = _rt::Vec::from_raw_parts(l2.cast(), len4, len4);
                                    _rt::string_lift(bytes4)
                                };
                                Some(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl IncomingRequest {
                #[allow(unused_unsafe, clippy::all)]
                pub fn headers(&self) -> Headers {
                    unsafe {
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:http/types@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]incoming-request.headers"]
                            fn wit_import(_: i32) -> i32;
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32) -> i32 {
                            unreachable!()
                        }
                        let ret = wit_import((self).handle() as i32);
                        Fields::from_handle(ret as u32)
                    }
                }
            }
            impl IncomingRequest {
                #[allow(unused_unsafe, clippy::all)]
                pub fn consume(&self) -> Result<IncomingBody, ()> {
                    unsafe {
                        #[repr(align(4))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 8]);
                        let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 8]);
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:http/types@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]incoming-request.consume"]
                            fn wit_import(_: i32, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, ptr0);
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        match l1 {
                            0 => {
                                let e = {
                                    let l2 = *ptr0.add(4).cast::<i32>();
                                    IncomingBody::from_handle(l2 as u32)
                                };
                                Ok(e)
                            }
                            1 => {
                                let e = ();
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl OutgoingRequest {
                #[allow(unused_unsafe, clippy::all)]
                pub fn new(headers: Headers) -> Self {
                    unsafe {
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:http/types@0.2.0")]
                        extern "C" {
                            #[link_name = "[constructor]outgoing-request"]
                            fn wit_import(_: i32) -> i32;
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32) -> i32 {
                            unreachable!()
                        }
                        let ret = wit_import((&headers).take_handle() as i32);
                        OutgoingRequest::from_handle(ret as u32)
                    }
                }
            }
            impl OutgoingRequest {
                #[allow(unused_unsafe, clippy::all)]
                pub fn body(&self) -> Result<OutgoingBody, ()> {
                    unsafe {
                        #[repr(align(4))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 8]);
                        let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 8]);
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:http/types@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]outgoing-request.body"]
                            fn wit_import(_: i32, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, ptr0);
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        match l1 {
                            0 => {
                                let e = {
                                    let l2 = *ptr0.add(4).cast::<i32>();
                                    OutgoingBody::from_handle(l2 as u32)
                                };
                                Ok(e)
                            }
                            1 => {
                                let e = ();
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl OutgoingRequest {
                #[allow(unused_unsafe, clippy::all)]
                pub fn method(&self) -> Method {
                    unsafe {
                        #[repr(align(4))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 12]);
                        let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 12]);
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:http/types@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]outgoing-request.method"]
                            fn wit_import(_: i32, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, ptr0);
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        let v5 = match l1 {
                            0 => Method::Get,
                            1 => Method::Head,
                            2 => Method::Post,
                            3 => Method::Put,
                            4 => Method::Delete,
                            5 => Method::Connect,
                            6 => Method::Options,
                            7 => Method::Trace,
                            8 => Method::Patch,
                            n => {
                                debug_assert_eq!(n, 9, "invalid enum discriminant");
                                let e5 = {
                                    let l2 = *ptr0.add(4).cast::<*mut u8>();
                                    let l3 = *ptr0.add(8).cast::<usize>();
                                    let len4 = l3;
                                    let bytes4 = _rt::Vec::from_raw_parts(l2.cast(), len4, len4);
                                    _rt::string_lift(bytes4)
                                };
                                Method::Other(e5)
                            }
                        };
                        v5
                    }
                }
            }
            impl OutgoingRequest {
                #[allow(unused_unsafe, clippy::all)]
                pub fn set_method(&self, method: &Method) -> Result<(), ()> {
                    unsafe {
                        let (result1_0, result1_1, result1_2) = match method {
                            Method::Get => (0i32, ::core::ptr::null_mut(), 0usize),
                            Method::Head => (1i32, ::core::ptr::null_mut(), 0usize),
                            Method::Post => (2i32, ::core::ptr::null_mut(), 0usize),
                            Method::Put => (3i32, ::core::ptr::null_mut(), 0usize),
                            Method::Delete => (4i32, ::core::ptr::null_mut(), 0usize),
                            Method::Connect => (5i32, ::core::ptr::null_mut(), 0usize),
                            Method::Options => (6i32, ::core::ptr::null_mut(), 0usize),
                            Method::Trace => (7i32, ::core::ptr::null_mut(), 0usize),
                            Method::Patch => (8i32, ::core::ptr::null_mut(), 0usize),
                            Method::Other(e) => {
                                let vec0 = e;
                                let ptr0 = vec0.as_ptr().cast::<u8>();
                                let len0 = vec0.len();
                                (9i32, ptr0.cast_mut(), len0)
                            }
                        };
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:http/types@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]outgoing-request.set-method"]
                            fn wit_import(_: i32, _: i32, _: *mut u8, _: usize) -> i32;
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: i32, _: *mut u8, _: usize) -> i32 {
                            unreachable!()
                        }
                        let ret =
                            wit_import((self).handle() as i32, result1_0, result1_1, result1_2);
                        match ret {
                            0 => {
                                let e = ();
                                Ok(e)
                            }
                            1 => {
                                let e = ();
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl OutgoingRequest {
                #[allow(unused_unsafe, clippy::all)]
                pub fn path_with_query(&self) -> Option<_rt::String> {
                    unsafe {
                        #[repr(align(4))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 12]);
                        let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 12]);
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:http/types@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]outgoing-request.path-with-query"]
                            fn wit_import(_: i32, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, ptr0);
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        match l1 {
                            0 => None,
                            1 => {
                                let e = {
                                    let l2 = *ptr0.add(4).cast::<*mut u8>();
                                    let l3 = *ptr0.add(8).cast::<usize>();
                                    let len4 = l3;
                                    let bytes4 = _rt::Vec::from_raw_parts(l2.cast(), len4, len4);
                                    _rt::string_lift(bytes4)
                                };
                                Some(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl OutgoingRequest {
                #[allow(unused_unsafe, clippy::all)]
                pub fn set_path_with_query(&self, path_with_query: Option<&str>) -> Result<(), ()> {
                    unsafe {
                        let (result1_0, result1_1, result1_2) = match path_with_query {
                            Some(e) => {
                                let vec0 = e;
                                let ptr0 = vec0.as_ptr().cast::<u8>();
                                let len0 = vec0.len();
                                (1i32, ptr0.cast_mut(), len0)
                            }
                            None => (0i32, ::core::ptr::null_mut(), 0usize),
                        };
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:http/types@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]outgoing-request.set-path-with-query"]
                            fn wit_import(_: i32, _: i32, _: *mut u8, _: usize) -> i32;
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: i32, _: *mut u8, _: usize) -> i32 {
                            unreachable!()
                        }
                        let ret =
                            wit_import((self).handle() as i32, result1_0, result1_1, result1_2);
                        match ret {
                            0 => {
                                let e = ();
                                Ok(e)
                            }
                            1 => {
                                let e = ();
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl OutgoingRequest {
                #[allow(unused_unsafe, clippy::all)]
                pub fn scheme(&self) -> Option<Scheme> {
                    unsafe {
                        #[repr(align(4))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 16]);
                        let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 16]);
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:http/types@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]outgoing-request.scheme"]
                            fn wit_import(_: i32, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, ptr0);
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        match l1 {
                            0 => None,
                            1 => {
                                let e = {
                                    let l2 = i32::from(*ptr0.add(4).cast::<u8>());
                                    let v6 = match l2 {
                                        0 => Scheme::Http,
                                        1 => Scheme::Https,
                                        n => {
                                            debug_assert_eq!(n, 2, "invalid enum discriminant");
                                            let e6 = {
                                                let l3 = *ptr0.add(8).cast::<*mut u8>();
                                                let l4 = *ptr0.add(12).cast::<usize>();
                                                let len5 = l4;
                                                let bytes5 =
                                                    _rt::Vec::from_raw_parts(l3.cast(), len5, len5);
                                                _rt::string_lift(bytes5)
                                            };
                                            Scheme::Other(e6)
                                        }
                                    };
                                    v6
                                };
                                Some(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl OutgoingRequest {
                #[allow(unused_unsafe, clippy::all)]
                pub fn set_scheme(&self, scheme: Option<&Scheme>) -> Result<(), ()> {
                    unsafe {
                        let (result2_0, result2_1, result2_2, result2_3) = match scheme {
                            Some(e) => {
                                let (result1_0, result1_1, result1_2) = match e {
                                    Scheme::Http => (0i32, ::core::ptr::null_mut(), 0usize),
                                    Scheme::Https => (1i32, ::core::ptr::null_mut(), 0usize),
                                    Scheme::Other(e) => {
                                        let vec0 = e;
                                        let ptr0 = vec0.as_ptr().cast::<u8>();
                                        let len0 = vec0.len();
                                        (2i32, ptr0.cast_mut(), len0)
                                    }
                                };
                                (1i32, result1_0, result1_1, result1_2)
                            }
                            None => (0i32, 0i32, ::core::ptr::null_mut(), 0usize),
                        };
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:http/types@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]outgoing-request.set-scheme"]
                            fn wit_import(_: i32, _: i32, _: i32, _: *mut u8, _: usize) -> i32;
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: i32, _: i32, _: *mut u8, _: usize) -> i32 {
                            unreachable!()
                        }
                        let ret = wit_import(
                            (self).handle() as i32,
                            result2_0,
                            result2_1,
                            result2_2,
                            result2_3,
                        );
                        match ret {
                            0 => {
                                let e = ();
                                Ok(e)
                            }
                            1 => {
                                let e = ();
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl OutgoingRequest {
                #[allow(unused_unsafe, clippy::all)]
                pub fn authority(&self) -> Option<_rt::String> {
                    unsafe {
                        #[repr(align(4))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 12]);
                        let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 12]);
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:http/types@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]outgoing-request.authority"]
                            fn wit_import(_: i32, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, ptr0);
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        match l1 {
                            0 => None,
                            1 => {
                                let e = {
                                    let l2 = *ptr0.add(4).cast::<*mut u8>();
                                    let l3 = *ptr0.add(8).cast::<usize>();
                                    let len4 = l3;
                                    let bytes4 = _rt::Vec::from_raw_parts(l2.cast(), len4, len4);
                                    _rt::string_lift(bytes4)
                                };
                                Some(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl OutgoingRequest {
                #[allow(unused_unsafe, clippy::all)]
                pub fn set_authority(&self, authority: Option<&str>) -> Result<(), ()> {
                    unsafe {
                        let (result1_0, result1_1, result1_2) = match authority {
                            Some(e) => {
                                let vec0 = e;
                                let ptr0 = vec0.as_ptr().cast::<u8>();
                                let len0 = vec0.len();
                                (1i32, ptr0.cast_mut(), len0)
                            }
                            None => (0i32, ::core::ptr::null_mut(), 0usize),
                        };
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:http/types@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]outgoing-request.set-authority"]
                            fn wit_import(_: i32, _: i32, _: *mut u8, _: usize) -> i32;
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: i32, _: *mut u8, _: usize) -> i32 {
                            unreachable!()
                        }
                        let ret =
                            wit_import((self).handle() as i32, result1_0, result1_1, result1_2);
                        match ret {
                            0 => {
                                let e = ();
                                Ok(e)
                            }
                            1 => {
                                let e = ();
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl OutgoingRequest {
                #[allow(unused_unsafe, clippy::all)]
                pub fn headers(&self) -> Headers {
                    unsafe {
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:http/types@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]outgoing-request.headers"]
                            fn wit_import(_: i32) -> i32;
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32) -> i32 {
                            unreachable!()
                        }
                        let ret = wit_import((self).handle() as i32);
                        Fields::from_handle(ret as u32)
                    }
                }
            }
            impl RequestOptions {
                #[allow(unused_unsafe, clippy::all)]
                pub fn new() -> Self {
                    unsafe {
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:http/types@0.2.0")]
                        extern "C" {
                            #[link_name = "[constructor]request-options"]
                            fn wit_import() -> i32;
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import() -> i32 {
                            unreachable!()
                        }
                        let ret = wit_import();
                        RequestOptions::from_handle(ret as u32)
                    }
                }
            }
            impl RequestOptions {
                #[allow(unused_unsafe, clippy::all)]
                pub fn connect_timeout(&self) -> Option<Duration> {
                    unsafe {
                        #[repr(align(8))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 16]);
                        let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 16]);
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:http/types@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]request-options.connect-timeout"]
                            fn wit_import(_: i32, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, ptr0);
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        match l1 {
                            0 => None,
                            1 => {
                                let e = {
                                    let l2 = *ptr0.add(8).cast::<i64>();
                                    l2 as u64
                                };
                                Some(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl RequestOptions {
                #[allow(unused_unsafe, clippy::all)]
                pub fn set_connect_timeout(&self, duration: Option<Duration>) -> Result<(), ()> {
                    unsafe {
                        let (result0_0, result0_1) = match duration {
                            Some(e) => (1i32, _rt::as_i64(e)),
                            None => (0i32, 0i64),
                        };
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:http/types@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]request-options.set-connect-timeout"]
                            fn wit_import(_: i32, _: i32, _: i64) -> i32;
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: i32, _: i64) -> i32 {
                            unreachable!()
                        }
                        let ret = wit_import((self).handle() as i32, result0_0, result0_1);
                        match ret {
                            0 => {
                                let e = ();
                                Ok(e)
                            }
                            1 => {
                                let e = ();
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl RequestOptions {
                #[allow(unused_unsafe, clippy::all)]
                pub fn first_byte_timeout(&self) -> Option<Duration> {
                    unsafe {
                        #[repr(align(8))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 16]);
                        let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 16]);
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:http/types@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]request-options.first-byte-timeout"]
                            fn wit_import(_: i32, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, ptr0);
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        match l1 {
                            0 => None,
                            1 => {
                                let e = {
                                    let l2 = *ptr0.add(8).cast::<i64>();
                                    l2 as u64
                                };
                                Some(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl RequestOptions {
                #[allow(unused_unsafe, clippy::all)]
                pub fn set_first_byte_timeout(&self, duration: Option<Duration>) -> Result<(), ()> {
                    unsafe {
                        let (result0_0, result0_1) = match duration {
                            Some(e) => (1i32, _rt::as_i64(e)),
                            None => (0i32, 0i64),
                        };
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:http/types@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]request-options.set-first-byte-timeout"]
                            fn wit_import(_: i32, _: i32, _: i64) -> i32;
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: i32, _: i64) -> i32 {
                            unreachable!()
                        }
                        let ret = wit_import((self).handle() as i32, result0_0, result0_1);
                        match ret {
                            0 => {
                                let e = ();
                                Ok(e)
                            }
                            1 => {
                                let e = ();
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl RequestOptions {
                #[allow(unused_unsafe, clippy::all)]
                pub fn between_bytes_timeout(&self) -> Option<Duration> {
                    unsafe {
                        #[repr(align(8))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 16]);
                        let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 16]);
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:http/types@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]request-options.between-bytes-timeout"]
                            fn wit_import(_: i32, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, ptr0);
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        match l1 {
                            0 => None,
                            1 => {
                                let e = {
                                    let l2 = *ptr0.add(8).cast::<i64>();
                                    l2 as u64
                                };
                                Some(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl RequestOptions {
                #[allow(unused_unsafe, clippy::all)]
                pub fn set_between_bytes_timeout(
                    &self,
                    duration: Option<Duration>,
                ) -> Result<(), ()> {
                    unsafe {
                        let (result0_0, result0_1) = match duration {
                            Some(e) => (1i32, _rt::as_i64(e)),
                            None => (0i32, 0i64),
                        };
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:http/types@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]request-options.set-between-bytes-timeout"]
                            fn wit_import(_: i32, _: i32, _: i64) -> i32;
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: i32, _: i64) -> i32 {
                            unreachable!()
                        }
                        let ret = wit_import((self).handle() as i32, result0_0, result0_1);
                        match ret {
                            0 => {
                                let e = ();
                                Ok(e)
                            }
                            1 => {
                                let e = ();
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl ResponseOutparam {
                #[allow(unused_unsafe, clippy::all)]
                pub fn set(param: ResponseOutparam, response: Result<OutgoingResponse, ErrorCode>) {
                    unsafe {
                        let (
                            result38_0,
                            result38_1,
                            result38_2,
                            result38_3,
                            result38_4,
                            result38_5,
                            result38_6,
                            result38_7,
                        ) = match &response {
                            Ok(e) => (
                                0i32,
                                (e).take_handle() as i32,
                                0i32,
                                ::core::mem::MaybeUninit::<u64>::zeroed(),
                                ::core::ptr::null_mut(),
                                ::core::ptr::null_mut(),
                                0usize,
                                0i32,
                            ),
                            Err(e) => {
                                let (
                                    result37_0,
                                    result37_1,
                                    result37_2,
                                    result37_3,
                                    result37_4,
                                    result37_5,
                                    result37_6,
                                ) = match e {
                                    ErrorCode::DnsTimeout => (
                                        0i32,
                                        0i32,
                                        ::core::mem::MaybeUninit::<u64>::zeroed(),
                                        ::core::ptr::null_mut(),
                                        ::core::ptr::null_mut(),
                                        0usize,
                                        0i32,
                                    ),
                                    ErrorCode::DnsError(e) => {
                                        let DnsErrorPayload {
                                            rcode: rcode0,
                                            info_code: info_code0,
                                        } = e;
                                        let (result2_0, result2_1, result2_2) = match rcode0 {
                                            Some(e) => {
                                                let vec1 = e;
                                                let ptr1 = vec1.as_ptr().cast::<u8>();
                                                let len1 = vec1.len();
                                                (1i32, ptr1.cast_mut(), len1)
                                            }
                                            None => (0i32, ::core::ptr::null_mut(), 0usize),
                                        };
                                        let (result3_0, result3_1) = match info_code0 {
                                            Some(e) => (1i32, _rt::as_i32(e)),
                                            None => (0i32, 0i32),
                                        };
                                        (
                                            1i32,
                                            result2_0,
                                            {
                                                let mut t =
                                                    ::core::mem::MaybeUninit::<u64>::uninit();
                                                t.as_mut_ptr().cast::<*mut u8>().write(result2_1);
                                                t
                                            },
                                            result2_2 as *mut u8,
                                            result3_0 as *mut u8,
                                            result3_1 as usize,
                                            0i32,
                                        )
                                    }
                                    ErrorCode::DestinationNotFound => (
                                        2i32,
                                        0i32,
                                        ::core::mem::MaybeUninit::<u64>::zeroed(),
                                        ::core::ptr::null_mut(),
                                        ::core::ptr::null_mut(),
                                        0usize,
                                        0i32,
                                    ),
                                    ErrorCode::DestinationUnavailable => (
                                        3i32,
                                        0i32,
                                        ::core::mem::MaybeUninit::<u64>::zeroed(),
                                        ::core::ptr::null_mut(),
                                        ::core::ptr::null_mut(),
                                        0usize,
                                        0i32,
                                    ),
                                    ErrorCode::DestinationIpProhibited => (
                                        4i32,
                                        0i32,
                                        ::core::mem::MaybeUninit::<u64>::zeroed(),
                                        ::core::ptr::null_mut(),
                                        ::core::ptr::null_mut(),
                                        0usize,
                                        0i32,
                                    ),
                                    ErrorCode::DestinationIpUnroutable => (
                                        5i32,
                                        0i32,
                                        ::core::mem::MaybeUninit::<u64>::zeroed(),
                                        ::core::ptr::null_mut(),
                                        ::core::ptr::null_mut(),
                                        0usize,
                                        0i32,
                                    ),
                                    ErrorCode::ConnectionRefused => (
                                        6i32,
                                        0i32,
                                        ::core::mem::MaybeUninit::<u64>::zeroed(),
                                        ::core::ptr::null_mut(),
                                        ::core::ptr::null_mut(),
                                        0usize,
                                        0i32,
                                    ),
                                    ErrorCode::ConnectionTerminated => (
                                        7i32,
                                        0i32,
                                        ::core::mem::MaybeUninit::<u64>::zeroed(),
                                        ::core::ptr::null_mut(),
                                        ::core::ptr::null_mut(),
                                        0usize,
                                        0i32,
                                    ),
                                    ErrorCode::ConnectionTimeout => (
                                        8i32,
                                        0i32,
                                        ::core::mem::MaybeUninit::<u64>::zeroed(),
                                        ::core::ptr::null_mut(),
                                        ::core::ptr::null_mut(),
                                        0usize,
                                        0i32,
                                    ),
                                    ErrorCode::ConnectionReadTimeout => (
                                        9i32,
                                        0i32,
                                        ::core::mem::MaybeUninit::<u64>::zeroed(),
                                        ::core::ptr::null_mut(),
                                        ::core::ptr::null_mut(),
                                        0usize,
                                        0i32,
                                    ),
                                    ErrorCode::ConnectionWriteTimeout => (
                                        10i32,
                                        0i32,
                                        ::core::mem::MaybeUninit::<u64>::zeroed(),
                                        ::core::ptr::null_mut(),
                                        ::core::ptr::null_mut(),
                                        0usize,
                                        0i32,
                                    ),
                                    ErrorCode::ConnectionLimitReached => (
                                        11i32,
                                        0i32,
                                        ::core::mem::MaybeUninit::<u64>::zeroed(),
                                        ::core::ptr::null_mut(),
                                        ::core::ptr::null_mut(),
                                        0usize,
                                        0i32,
                                    ),
                                    ErrorCode::TlsProtocolError => (
                                        12i32,
                                        0i32,
                                        ::core::mem::MaybeUninit::<u64>::zeroed(),
                                        ::core::ptr::null_mut(),
                                        ::core::ptr::null_mut(),
                                        0usize,
                                        0i32,
                                    ),
                                    ErrorCode::TlsCertificateError => (
                                        13i32,
                                        0i32,
                                        ::core::mem::MaybeUninit::<u64>::zeroed(),
                                        ::core::ptr::null_mut(),
                                        ::core::ptr::null_mut(),
                                        0usize,
                                        0i32,
                                    ),
                                    ErrorCode::TlsAlertReceived(e) => {
                                        let TlsAlertReceivedPayload {
                                            alert_id: alert_id4,
                                            alert_message: alert_message4,
                                        } = e;
                                        let (result5_0, result5_1) = match alert_id4 {
                                            Some(e) => (1i32, _rt::as_i32(e)),
                                            None => (0i32, 0i32),
                                        };
                                        let (result7_0, result7_1, result7_2) = match alert_message4
                                        {
                                            Some(e) => {
                                                let vec6 = e;
                                                let ptr6 = vec6.as_ptr().cast::<u8>();
                                                let len6 = vec6.len();
                                                (1i32, ptr6.cast_mut(), len6)
                                            }
                                            None => (0i32, ::core::ptr::null_mut(), 0usize),
                                        };
                                        (
                                            14i32,
                                            result5_0,
                                            ::core::mem::MaybeUninit::new(
                                                i64::from(result5_1) as u64
                                            ),
                                            result7_0 as *mut u8,
                                            result7_1,
                                            result7_2,
                                            0i32,
                                        )
                                    }
                                    ErrorCode::HttpRequestDenied => (
                                        15i32,
                                        0i32,
                                        ::core::mem::MaybeUninit::<u64>::zeroed(),
                                        ::core::ptr::null_mut(),
                                        ::core::ptr::null_mut(),
                                        0usize,
                                        0i32,
                                    ),
                                    ErrorCode::HttpRequestLengthRequired => (
                                        16i32,
                                        0i32,
                                        ::core::mem::MaybeUninit::<u64>::zeroed(),
                                        ::core::ptr::null_mut(),
                                        ::core::ptr::null_mut(),
                                        0usize,
                                        0i32,
                                    ),
                                    ErrorCode::HttpRequestBodySize(e) => {
                                        let (result8_0, result8_1) = match e {
                                            Some(e) => (1i32, _rt::as_i64(e)),
                                            None => (0i32, 0i64),
                                        };
                                        (
                                            17i32,
                                            result8_0,
                                            ::core::mem::MaybeUninit::new(result8_1 as u64),
                                            ::core::ptr::null_mut(),
                                            ::core::ptr::null_mut(),
                                            0usize,
                                            0i32,
                                        )
                                    }
                                    ErrorCode::HttpRequestMethodInvalid => (
                                        18i32,
                                        0i32,
                                        ::core::mem::MaybeUninit::<u64>::zeroed(),
                                        ::core::ptr::null_mut(),
                                        ::core::ptr::null_mut(),
                                        0usize,
                                        0i32,
                                    ),
                                    ErrorCode::HttpRequestUriInvalid => (
                                        19i32,
                                        0i32,
                                        ::core::mem::MaybeUninit::<u64>::zeroed(),
                                        ::core::ptr::null_mut(),
                                        ::core::ptr::null_mut(),
                                        0usize,
                                        0i32,
                                    ),
                                    ErrorCode::HttpRequestUriTooLong => (
                                        20i32,
                                        0i32,
                                        ::core::mem::MaybeUninit::<u64>::zeroed(),
                                        ::core::ptr::null_mut(),
                                        ::core::ptr::null_mut(),
                                        0usize,
                                        0i32,
                                    ),
                                    ErrorCode::HttpRequestHeaderSectionSize(e) => {
                                        let (result9_0, result9_1) = match e {
                                            Some(e) => (1i32, _rt::as_i32(e)),
                                            None => (0i32, 0i32),
                                        };
                                        (
                                            21i32,
                                            result9_0,
                                            ::core::mem::MaybeUninit::new(
                                                i64::from(result9_1) as u64
                                            ),
                                            ::core::ptr::null_mut(),
                                            ::core::ptr::null_mut(),
                                            0usize,
                                            0i32,
                                        )
                                    }
                                    ErrorCode::HttpRequestHeaderSize(e) => {
                                        let (
                                            result14_0,
                                            result14_1,
                                            result14_2,
                                            result14_3,
                                            result14_4,
                                            result14_5,
                                        ) = match e {
                                            Some(e) => {
                                                let FieldSizePayload {
                                                    field_name: field_name10,
                                                    field_size: field_size10,
                                                } = e;
                                                let (result12_0, result12_1, result12_2) =
                                                    match field_name10 {
                                                        Some(e) => {
                                                            let vec11 = e;
                                                            let ptr11 = vec11.as_ptr().cast::<u8>();
                                                            let len11 = vec11.len();
                                                            (1i32, ptr11.cast_mut(), len11)
                                                        }
                                                        None => {
                                                            (0i32, ::core::ptr::null_mut(), 0usize)
                                                        }
                                                    };
                                                let (result13_0, result13_1) = match field_size10 {
                                                    Some(e) => (1i32, _rt::as_i32(e)),
                                                    None => (0i32, 0i32),
                                                };
                                                (
                                                    1i32, result12_0, result12_1, result12_2,
                                                    result13_0, result13_1,
                                                )
                                            }
                                            None => (
                                                0i32,
                                                0i32,
                                                ::core::ptr::null_mut(),
                                                0usize,
                                                0i32,
                                                0i32,
                                            ),
                                        };
                                        (
                                            22i32,
                                            result14_0,
                                            ::core::mem::MaybeUninit::new(
                                                i64::from(result14_1) as u64
                                            ),
                                            result14_2,
                                            result14_3 as *mut u8,
                                            result14_4 as usize,
                                            result14_5,
                                        )
                                    }
                                    ErrorCode::HttpRequestTrailerSectionSize(e) => {
                                        let (result15_0, result15_1) = match e {
                                            Some(e) => (1i32, _rt::as_i32(e)),
                                            None => (0i32, 0i32),
                                        };
                                        (
                                            23i32,
                                            result15_0,
                                            ::core::mem::MaybeUninit::new(
                                                i64::from(result15_1) as u64
                                            ),
                                            ::core::ptr::null_mut(),
                                            ::core::ptr::null_mut(),
                                            0usize,
                                            0i32,
                                        )
                                    }
                                    ErrorCode::HttpRequestTrailerSize(e) => {
                                        let FieldSizePayload {
                                            field_name: field_name16,
                                            field_size: field_size16,
                                        } = e;
                                        let (result18_0, result18_1, result18_2) =
                                            match field_name16 {
                                                Some(e) => {
                                                    let vec17 = e;
                                                    let ptr17 = vec17.as_ptr().cast::<u8>();
                                                    let len17 = vec17.len();
                                                    (1i32, ptr17.cast_mut(), len17)
                                                }
                                                None => (0i32, ::core::ptr::null_mut(), 0usize),
                                            };
                                        let (result19_0, result19_1) = match field_size16 {
                                            Some(e) => (1i32, _rt::as_i32(e)),
                                            None => (0i32, 0i32),
                                        };
                                        (
                                            24i32,
                                            result18_0,
                                            {
                                                let mut t =
                                                    ::core::mem::MaybeUninit::<u64>::uninit();
                                                t.as_mut_ptr().cast::<*mut u8>().write(result18_1);
                                                t
                                            },
                                            result18_2 as *mut u8,
                                            result19_0 as *mut u8,
                                            result19_1 as usize,
                                            0i32,
                                        )
                                    }
                                    ErrorCode::HttpResponseIncomplete => (
                                        25i32,
                                        0i32,
                                        ::core::mem::MaybeUninit::<u64>::zeroed(),
                                        ::core::ptr::null_mut(),
                                        ::core::ptr::null_mut(),
                                        0usize,
                                        0i32,
                                    ),
                                    ErrorCode::HttpResponseHeaderSectionSize(e) => {
                                        let (result20_0, result20_1) = match e {
                                            Some(e) => (1i32, _rt::as_i32(e)),
                                            None => (0i32, 0i32),
                                        };
                                        (
                                            26i32,
                                            result20_0,
                                            ::core::mem::MaybeUninit::new(
                                                i64::from(result20_1) as u64
                                            ),
                                            ::core::ptr::null_mut(),
                                            ::core::ptr::null_mut(),
                                            0usize,
                                            0i32,
                                        )
                                    }
                                    ErrorCode::HttpResponseHeaderSize(e) => {
                                        let FieldSizePayload {
                                            field_name: field_name21,
                                            field_size: field_size21,
                                        } = e;
                                        let (result23_0, result23_1, result23_2) =
                                            match field_name21 {
                                                Some(e) => {
                                                    let vec22 = e;
                                                    let ptr22 = vec22.as_ptr().cast::<u8>();
                                                    let len22 = vec22.len();
                                                    (1i32, ptr22.cast_mut(), len22)
                                                }
                                                None => (0i32, ::core::ptr::null_mut(), 0usize),
                                            };
                                        let (result24_0, result24_1) = match field_size21 {
                                            Some(e) => (1i32, _rt::as_i32(e)),
                                            None => (0i32, 0i32),
                                        };
                                        (
                                            27i32,
                                            result23_0,
                                            {
                                                let mut t =
                                                    ::core::mem::MaybeUninit::<u64>::uninit();
                                                t.as_mut_ptr().cast::<*mut u8>().write(result23_1);
                                                t
                                            },
                                            result23_2 as *mut u8,
                                            result24_0 as *mut u8,
                                            result24_1 as usize,
                                            0i32,
                                        )
                                    }
                                    ErrorCode::HttpResponseBodySize(e) => {
                                        let (result25_0, result25_1) = match e {
                                            Some(e) => (1i32, _rt::as_i64(e)),
                                            None => (0i32, 0i64),
                                        };
                                        (
                                            28i32,
                                            result25_0,
                                            ::core::mem::MaybeUninit::new(result25_1 as u64),
                                            ::core::ptr::null_mut(),
                                            ::core::ptr::null_mut(),
                                            0usize,
                                            0i32,
                                        )
                                    }
                                    ErrorCode::HttpResponseTrailerSectionSize(e) => {
                                        let (result26_0, result26_1) = match e {
                                            Some(e) => (1i32, _rt::as_i32(e)),
                                            None => (0i32, 0i32),
                                        };
                                        (
                                            29i32,
                                            result26_0,
                                            ::core::mem::MaybeUninit::new(
                                                i64::from(result26_1) as u64
                                            ),
                                            ::core::ptr::null_mut(),
                                            ::core::ptr::null_mut(),
                                            0usize,
                                            0i32,
                                        )
                                    }
                                    ErrorCode::HttpResponseTrailerSize(e) => {
                                        let FieldSizePayload {
                                            field_name: field_name27,
                                            field_size: field_size27,
                                        } = e;
                                        let (result29_0, result29_1, result29_2) =
                                            match field_name27 {
                                                Some(e) => {
                                                    let vec28 = e;
                                                    let ptr28 = vec28.as_ptr().cast::<u8>();
                                                    let len28 = vec28.len();
                                                    (1i32, ptr28.cast_mut(), len28)
                                                }
                                                None => (0i32, ::core::ptr::null_mut(), 0usize),
                                            };
                                        let (result30_0, result30_1) = match field_size27 {
                                            Some(e) => (1i32, _rt::as_i32(e)),
                                            None => (0i32, 0i32),
                                        };
                                        (
                                            30i32,
                                            result29_0,
                                            {
                                                let mut t =
                                                    ::core::mem::MaybeUninit::<u64>::uninit();
                                                t.as_mut_ptr().cast::<*mut u8>().write(result29_1);
                                                t
                                            },
                                            result29_2 as *mut u8,
                                            result30_0 as *mut u8,
                                            result30_1 as usize,
                                            0i32,
                                        )
                                    }
                                    ErrorCode::HttpResponseTransferCoding(e) => {
                                        let (result32_0, result32_1, result32_2) = match e {
                                            Some(e) => {
                                                let vec31 = e;
                                                let ptr31 = vec31.as_ptr().cast::<u8>();
                                                let len31 = vec31.len();
                                                (1i32, ptr31.cast_mut(), len31)
                                            }
                                            None => (0i32, ::core::ptr::null_mut(), 0usize),
                                        };
                                        (
                                            31i32,
                                            result32_0,
                                            {
                                                let mut t =
                                                    ::core::mem::MaybeUninit::<u64>::uninit();
                                                t.as_mut_ptr().cast::<*mut u8>().write(result32_1);
                                                t
                                            },
                                            result32_2 as *mut u8,
                                            ::core::ptr::null_mut(),
                                            0usize,
                                            0i32,
                                        )
                                    }
                                    ErrorCode::HttpResponseContentCoding(e) => {
                                        let (result34_0, result34_1, result34_2) = match e {
                                            Some(e) => {
                                                let vec33 = e;
                                                let ptr33 = vec33.as_ptr().cast::<u8>();
                                                let len33 = vec33.len();
                                                (1i32, ptr33.cast_mut(), len33)
                                            }
                                            None => (0i32, ::core::ptr::null_mut(), 0usize),
                                        };
                                        (
                                            32i32,
                                            result34_0,
                                            {
                                                let mut t =
                                                    ::core::mem::MaybeUninit::<u64>::uninit();
                                                t.as_mut_ptr().cast::<*mut u8>().write(result34_1);
                                                t
                                            },
                                            result34_2 as *mut u8,
                                            ::core::ptr::null_mut(),
                                            0usize,
                                            0i32,
                                        )
                                    }
                                    ErrorCode::HttpResponseTimeout => (
                                        33i32,
                                        0i32,
                                        ::core::mem::MaybeUninit::<u64>::zeroed(),
                                        ::core::ptr::null_mut(),
                                        ::core::ptr::null_mut(),
                                        0usize,
                                        0i32,
                                    ),
                                    ErrorCode::HttpUpgradeFailed => (
                                        34i32,
                                        0i32,
                                        ::core::mem::MaybeUninit::<u64>::zeroed(),
                                        ::core::ptr::null_mut(),
                                        ::core::ptr::null_mut(),
                                        0usize,
                                        0i32,
                                    ),
                                    ErrorCode::HttpProtocolError => (
                                        35i32,
                                        0i32,
                                        ::core::mem::MaybeUninit::<u64>::zeroed(),
                                        ::core::ptr::null_mut(),
                                        ::core::ptr::null_mut(),
                                        0usize,
                                        0i32,
                                    ),
                                    ErrorCode::LoopDetected => (
                                        36i32,
                                        0i32,
                                        ::core::mem::MaybeUninit::<u64>::zeroed(),
                                        ::core::ptr::null_mut(),
                                        ::core::ptr::null_mut(),
                                        0usize,
                                        0i32,
                                    ),
                                    ErrorCode::ConfigurationError => (
                                        37i32,
                                        0i32,
                                        ::core::mem::MaybeUninit::<u64>::zeroed(),
                                        ::core::ptr::null_mut(),
                                        ::core::ptr::null_mut(),
                                        0usize,
                                        0i32,
                                    ),
                                    ErrorCode::InternalError(e) => {
                                        let (result36_0, result36_1, result36_2) = match e {
                                            Some(e) => {
                                                let vec35 = e;
                                                let ptr35 = vec35.as_ptr().cast::<u8>();
                                                let len35 = vec35.len();
                                                (1i32, ptr35.cast_mut(), len35)
                                            }
                                            None => (0i32, ::core::ptr::null_mut(), 0usize),
                                        };
                                        (
                                            38i32,
                                            result36_0,
                                            {
                                                let mut t =
                                                    ::core::mem::MaybeUninit::<u64>::uninit();
                                                t.as_mut_ptr().cast::<*mut u8>().write(result36_1);
                                                t
                                            },
                                            result36_2 as *mut u8,
                                            ::core::ptr::null_mut(),
                                            0usize,
                                            0i32,
                                        )
                                    }
                                };
                                (
                                    1i32, result37_0, result37_1, result37_2, result37_3,
                                    result37_4, result37_5, result37_6,
                                )
                            }
                        };
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:http/types@0.2.0")]
                        extern "C" {
                            #[link_name = "[static]response-outparam.set"]
                            fn wit_import(
                                _: i32,
                                _: i32,
                                _: i32,
                                _: i32,
                                _: ::core::mem::MaybeUninit<u64>,
                                _: *mut u8,
                                _: *mut u8,
                                _: usize,
                                _: i32,
                            );
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(
                            _: i32,
                            _: i32,
                            _: i32,
                            _: i32,
                            _: ::core::mem::MaybeUninit<u64>,
                            _: *mut u8,
                            _: *mut u8,
                            _: usize,
                            _: i32,
                        ) {
                            unreachable!()
                        }
                        wit_import(
                            (&param).take_handle() as i32,
                            result38_0,
                            result38_1,
                            result38_2,
                            result38_3,
                            result38_4,
                            result38_5,
                            result38_6,
                            result38_7,
                        );
                    }
                }
            }
            impl IncomingResponse {
                #[allow(unused_unsafe, clippy::all)]
                pub fn status(&self) -> StatusCode {
                    unsafe {
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:http/types@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]incoming-response.status"]
                            fn wit_import(_: i32) -> i32;
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32) -> i32 {
                            unreachable!()
                        }
                        let ret = wit_import((self).handle() as i32);
                        ret as u16
                    }
                }
            }
            impl IncomingResponse {
                #[allow(unused_unsafe, clippy::all)]
                pub fn headers(&self) -> Headers {
                    unsafe {
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:http/types@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]incoming-response.headers"]
                            fn wit_import(_: i32) -> i32;
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32) -> i32 {
                            unreachable!()
                        }
                        let ret = wit_import((self).handle() as i32);
                        Fields::from_handle(ret as u32)
                    }
                }
            }
            impl IncomingResponse {
                #[allow(unused_unsafe, clippy::all)]
                pub fn consume(&self) -> Result<IncomingBody, ()> {
                    unsafe {
                        #[repr(align(4))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 8]);
                        let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 8]);
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:http/types@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]incoming-response.consume"]
                            fn wit_import(_: i32, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, ptr0);
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        match l1 {
                            0 => {
                                let e = {
                                    let l2 = *ptr0.add(4).cast::<i32>();
                                    IncomingBody::from_handle(l2 as u32)
                                };
                                Ok(e)
                            }
                            1 => {
                                let e = ();
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl IncomingBody {
                #[allow(unused_unsafe, clippy::all)]
                pub fn stream(&self) -> Result<InputStream, ()> {
                    unsafe {
                        #[repr(align(4))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 8]);
                        let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 8]);
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:http/types@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]incoming-body.stream"]
                            fn wit_import(_: i32, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, ptr0);
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        match l1 {
                            0 => {
                                let e = {
                                    let l2 = *ptr0.add(4).cast::<i32>();
                                    super::super::super::wasi::io::streams::InputStream::from_handle(
                                        l2 as u32,
                                    )
                                };
                                Ok(e)
                            }
                            1 => {
                                let e = ();
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl IncomingBody {
                #[allow(unused_unsafe, clippy::all)]
                pub fn finish(this: IncomingBody) -> FutureTrailers {
                    unsafe {
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:http/types@0.2.0")]
                        extern "C" {
                            #[link_name = "[static]incoming-body.finish"]
                            fn wit_import(_: i32) -> i32;
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32) -> i32 {
                            unreachable!()
                        }
                        let ret = wit_import((&this).take_handle() as i32);
                        FutureTrailers::from_handle(ret as u32)
                    }
                }
            }
            impl FutureTrailers {
                #[allow(unused_unsafe, clippy::all)]
                pub fn subscribe(&self) -> Pollable {
                    unsafe {
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:http/types@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]future-trailers.subscribe"]
                            fn wit_import(_: i32) -> i32;
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32) -> i32 {
                            unreachable!()
                        }
                        let ret = wit_import((self).handle() as i32);
                        super::super::super::wasi::io::poll::Pollable::from_handle(ret as u32)
                    }
                }
            }
            impl FutureTrailers {
                #[allow(unused_unsafe, clippy::all)]
                pub fn get(&self) -> Option<Result<Result<Option<Trailers>, ErrorCode>, ()>> {
                    unsafe {
                        #[repr(align(8))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 56]);
                        let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 56]);
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:http/types@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]future-trailers.get"]
                            fn wit_import(_: i32, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, ptr0);
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        match l1 {
                            0 => None,
                            1 => {
                                let e = {
                                    let l2 = i32::from(*ptr0.add(8).cast::<u8>());
                                    match l2 {
                                        0 => {
                                            let e = {
                                                let l3 = i32::from(*ptr0.add(16).cast::<u8>());
                                                match l3 {
                                                    0 => {
                                                        let e = {
                                                            let l4 = i32::from(
                                                                *ptr0.add(24).cast::<u8>(),
                                                            );
                                                            match l4 {
                                                                0 => None,
                                                                1 => {
                                                                    let e = {
                                                                        let l5 = *ptr0
                                                                            .add(28)
                                                                            .cast::<i32>();
                                                                        Fields::from_handle(
                                                                            l5 as u32,
                                                                        )
                                                                    };
                                                                    Some(e)
                                                                }
                                                                _ => {
                                                                    _rt::invalid_enum_discriminant()
                                                                }
                                                            }
                                                        };
                                                        Ok(e)
                                                    }
                                                    1 => {
                                                        let e = {
                                                            let l6 = i32::from(
                                                                *ptr0.add(24).cast::<u8>(),
                                                            );
                                                            let v68 = match l6 {
                                                                0 => ErrorCode::DnsTimeout,
                                                                1 => {
                                                                    let e68 = {
                                                                        let l7 = i32::from(*ptr0.add(32).cast::<u8>());
                                                                        let l11 = i32::from(*ptr0.add(44).cast::<u8>());
                                                                        DnsErrorPayload {
                                                                            rcode: match l7 {
                                                                                0 => None,
                                                                                1 => {
                                                                                    let e = {
                                                                                        let l8 = *ptr0.add(36).cast::<*mut u8>();
                                                                                        let l9 = *ptr0.add(40).cast::<usize>();
                                                                                        let len10 = l9;
                                                                                        let bytes10 = _rt::Vec::from_raw_parts(
                                                                                            l8.cast(),
                                                                                            len10,
                                                                                            len10,
                                                                                        );
                                                                                        _rt::string_lift(bytes10)
                                                                                    };
                                                                                    Some(e)
                                                                                }
                                                                                _ => _rt::invalid_enum_discriminant(),
                                                                            },
                                                                            info_code: match l11 {
                                                                                0 => None,
                                                                                1 => {
                                                                                    let e = {
                                                                                        let l12 = i32::from(*ptr0.add(46).cast::<u16>());
                                                                                        l12 as u16
                                                                                    };
                                                                                    Some(e)
                                                                                }
                                                                                _ => _rt::invalid_enum_discriminant(),
                                                                            },
                                                                        }
                                                                    };
                                                                    ErrorCode::DnsError(e68)
                                                                }
                                                                2 => ErrorCode::DestinationNotFound,
                                                                3 => ErrorCode::DestinationUnavailable,
                                                                4 => ErrorCode::DestinationIpProhibited,
                                                                5 => ErrorCode::DestinationIpUnroutable,
                                                                6 => ErrorCode::ConnectionRefused,
                                                                7 => ErrorCode::ConnectionTerminated,
                                                                8 => ErrorCode::ConnectionTimeout,
                                                                9 => ErrorCode::ConnectionReadTimeout,
                                                                10 => ErrorCode::ConnectionWriteTimeout,
                                                                11 => ErrorCode::ConnectionLimitReached,
                                                                12 => ErrorCode::TlsProtocolError,
                                                                13 => ErrorCode::TlsCertificateError,
                                                                14 => {
                                                                    let e68 = {
                                                                        let l13 = i32::from(*ptr0.add(32).cast::<u8>());
                                                                        let l15 = i32::from(*ptr0.add(36).cast::<u8>());
                                                                        TlsAlertReceivedPayload {
                                                                            alert_id: match l13 {
                                                                                0 => None,
                                                                                1 => {
                                                                                    let e = {
                                                                                        let l14 = i32::from(*ptr0.add(33).cast::<u8>());
                                                                                        l14 as u8
                                                                                    };
                                                                                    Some(e)
                                                                                }
                                                                                _ => _rt::invalid_enum_discriminant(),
                                                                            },
                                                                            alert_message: match l15 {
                                                                                0 => None,
                                                                                1 => {
                                                                                    let e = {
                                                                                        let l16 = *ptr0.add(40).cast::<*mut u8>();
                                                                                        let l17 = *ptr0.add(44).cast::<usize>();
                                                                                        let len18 = l17;
                                                                                        let bytes18 = _rt::Vec::from_raw_parts(
                                                                                            l16.cast(),
                                                                                            len18,
                                                                                            len18,
                                                                                        );
                                                                                        _rt::string_lift(bytes18)
                                                                                    };
                                                                                    Some(e)
                                                                                }
                                                                                _ => _rt::invalid_enum_discriminant(),
                                                                            },
                                                                        }
                                                                    };
                                                                    ErrorCode::TlsAlertReceived(e68)
                                                                }
                                                                15 => ErrorCode::HttpRequestDenied,
                                                                16 => ErrorCode::HttpRequestLengthRequired,
                                                                17 => {
                                                                    let e68 = {
                                                                        let l19 = i32::from(*ptr0.add(32).cast::<u8>());
                                                                        match l19 {
                                                                            0 => None,
                                                                            1 => {
                                                                                let e = {
                                                                                    let l20 = *ptr0.add(40).cast::<i64>();
                                                                                    l20 as u64
                                                                                };
                                                                                Some(e)
                                                                            }
                                                                            _ => _rt::invalid_enum_discriminant(),
                                                                        }
                                                                    };
                                                                    ErrorCode::HttpRequestBodySize(e68)
                                                                }
                                                                18 => ErrorCode::HttpRequestMethodInvalid,
                                                                19 => ErrorCode::HttpRequestUriInvalid,
                                                                20 => ErrorCode::HttpRequestUriTooLong,
                                                                21 => {
                                                                    let e68 = {
                                                                        let l21 = i32::from(*ptr0.add(32).cast::<u8>());
                                                                        match l21 {
                                                                            0 => None,
                                                                            1 => {
                                                                                let e = {
                                                                                    let l22 = *ptr0.add(36).cast::<i32>();
                                                                                    l22 as u32
                                                                                };
                                                                                Some(e)
                                                                            }
                                                                            _ => _rt::invalid_enum_discriminant(),
                                                                        }
                                                                    };
                                                                    ErrorCode::HttpRequestHeaderSectionSize(e68)
                                                                }
                                                                22 => {
                                                                    let e68 = {
                                                                        let l23 = i32::from(*ptr0.add(32).cast::<u8>());
                                                                        match l23 {
                                                                            0 => None,
                                                                            1 => {
                                                                                let e = {
                                                                                    let l24 = i32::from(*ptr0.add(36).cast::<u8>());
                                                                                    let l28 = i32::from(*ptr0.add(48).cast::<u8>());
                                                                                    FieldSizePayload {
                                                                                        field_name: match l24 {
                                                                                            0 => None,
                                                                                            1 => {
                                                                                                let e = {
                                                                                                    let l25 = *ptr0.add(40).cast::<*mut u8>();
                                                                                                    let l26 = *ptr0.add(44).cast::<usize>();
                                                                                                    let len27 = l26;
                                                                                                    let bytes27 = _rt::Vec::from_raw_parts(
                                                                                                        l25.cast(),
                                                                                                        len27,
                                                                                                        len27,
                                                                                                    );
                                                                                                    _rt::string_lift(bytes27)
                                                                                                };
                                                                                                Some(e)
                                                                                            }
                                                                                            _ => _rt::invalid_enum_discriminant(),
                                                                                        },
                                                                                        field_size: match l28 {
                                                                                            0 => None,
                                                                                            1 => {
                                                                                                let e = {
                                                                                                    let l29 = *ptr0.add(52).cast::<i32>();
                                                                                                    l29 as u32
                                                                                                };
                                                                                                Some(e)
                                                                                            }
                                                                                            _ => _rt::invalid_enum_discriminant(),
                                                                                        },
                                                                                    }
                                                                                };
                                                                                Some(e)
                                                                            }
                                                                            _ => _rt::invalid_enum_discriminant(),
                                                                        }
                                                                    };
                                                                    ErrorCode::HttpRequestHeaderSize(e68)
                                                                }
                                                                23 => {
                                                                    let e68 = {
                                                                        let l30 = i32::from(*ptr0.add(32).cast::<u8>());
                                                                        match l30 {
                                                                            0 => None,
                                                                            1 => {
                                                                                let e = {
                                                                                    let l31 = *ptr0.add(36).cast::<i32>();
                                                                                    l31 as u32
                                                                                };
                                                                                Some(e)
                                                                            }
                                                                            _ => _rt::invalid_enum_discriminant(),
                                                                        }
                                                                    };
                                                                    ErrorCode::HttpRequestTrailerSectionSize(e68)
                                                                }
                                                                24 => {
                                                                    let e68 = {
                                                                        let l32 = i32::from(*ptr0.add(32).cast::<u8>());
                                                                        let l36 = i32::from(*ptr0.add(44).cast::<u8>());
                                                                        FieldSizePayload {
                                                                            field_name: match l32 {
                                                                                0 => None,
                                                                                1 => {
                                                                                    let e = {
                                                                                        let l33 = *ptr0.add(36).cast::<*mut u8>();
                                                                                        let l34 = *ptr0.add(40).cast::<usize>();
                                                                                        let len35 = l34;
                                                                                        let bytes35 = _rt::Vec::from_raw_parts(
                                                                                            l33.cast(),
                                                                                            len35,
                                                                                            len35,
                                                                                        );
                                                                                        _rt::string_lift(bytes35)
                                                                                    };
                                                                                    Some(e)
                                                                                }
                                                                                _ => _rt::invalid_enum_discriminant(),
                                                                            },
                                                                            field_size: match l36 {
                                                                                0 => None,
                                                                                1 => {
                                                                                    let e = {
                                                                                        let l37 = *ptr0.add(48).cast::<i32>();
                                                                                        l37 as u32
                                                                                    };
                                                                                    Some(e)
                                                                                }
                                                                                _ => _rt::invalid_enum_discriminant(),
                                                                            },
                                                                        }
                                                                    };
                                                                    ErrorCode::HttpRequestTrailerSize(e68)
                                                                }
                                                                25 => ErrorCode::HttpResponseIncomplete,
                                                                26 => {
                                                                    let e68 = {
                                                                        let l38 = i32::from(*ptr0.add(32).cast::<u8>());
                                                                        match l38 {
                                                                            0 => None,
                                                                            1 => {
                                                                                let e = {
                                                                                    let l39 = *ptr0.add(36).cast::<i32>();
                                                                                    l39 as u32
                                                                                };
                                                                                Some(e)
                                                                            }
                                                                            _ => _rt::invalid_enum_discriminant(),
                                                                        }
                                                                    };
                                                                    ErrorCode::HttpResponseHeaderSectionSize(e68)
                                                                }
                                                                27 => {
                                                                    let e68 = {
                                                                        let l40 = i32::from(*ptr0.add(32).cast::<u8>());
                                                                        let l44 = i32::from(*ptr0.add(44).cast::<u8>());
                                                                        FieldSizePayload {
                                                                            field_name: match l40 {
                                                                                0 => None,
                                                                                1 => {
                                                                                    let e = {
                                                                                        let l41 = *ptr0.add(36).cast::<*mut u8>();
                                                                                        let l42 = *ptr0.add(40).cast::<usize>();
                                                                                        let len43 = l42;
                                                                                        let bytes43 = _rt::Vec::from_raw_parts(
                                                                                            l41.cast(),
                                                                                            len43,
                                                                                            len43,
                                                                                        );
                                                                                        _rt::string_lift(bytes43)
                                                                                    };
                                                                                    Some(e)
                                                                                }
                                                                                _ => _rt::invalid_enum_discriminant(),
                                                                            },
                                                                            field_size: match l44 {
                                                                                0 => None,
                                                                                1 => {
                                                                                    let e = {
                                                                                        let l45 = *ptr0.add(48).cast::<i32>();
                                                                                        l45 as u32
                                                                                    };
                                                                                    Some(e)
                                                                                }
                                                                                _ => _rt::invalid_enum_discriminant(),
                                                                            },
                                                                        }
                                                                    };
                                                                    ErrorCode::HttpResponseHeaderSize(e68)
                                                                }
                                                                28 => {
                                                                    let e68 = {
                                                                        let l46 = i32::from(*ptr0.add(32).cast::<u8>());
                                                                        match l46 {
                                                                            0 => None,
                                                                            1 => {
                                                                                let e = {
                                                                                    let l47 = *ptr0.add(40).cast::<i64>();
                                                                                    l47 as u64
                                                                                };
                                                                                Some(e)
                                                                            }
                                                                            _ => _rt::invalid_enum_discriminant(),
                                                                        }
                                                                    };
                                                                    ErrorCode::HttpResponseBodySize(e68)
                                                                }
                                                                29 => {
                                                                    let e68 = {
                                                                        let l48 = i32::from(*ptr0.add(32).cast::<u8>());
                                                                        match l48 {
                                                                            0 => None,
                                                                            1 => {
                                                                                let e = {
                                                                                    let l49 = *ptr0.add(36).cast::<i32>();
                                                                                    l49 as u32
                                                                                };
                                                                                Some(e)
                                                                            }
                                                                            _ => _rt::invalid_enum_discriminant(),
                                                                        }
                                                                    };
                                                                    ErrorCode::HttpResponseTrailerSectionSize(e68)
                                                                }
                                                                30 => {
                                                                    let e68 = {
                                                                        let l50 = i32::from(*ptr0.add(32).cast::<u8>());
                                                                        let l54 = i32::from(*ptr0.add(44).cast::<u8>());
                                                                        FieldSizePayload {
                                                                            field_name: match l50 {
                                                                                0 => None,
                                                                                1 => {
                                                                                    let e = {
                                                                                        let l51 = *ptr0.add(36).cast::<*mut u8>();
                                                                                        let l52 = *ptr0.add(40).cast::<usize>();
                                                                                        let len53 = l52;
                                                                                        let bytes53 = _rt::Vec::from_raw_parts(
                                                                                            l51.cast(),
                                                                                            len53,
                                                                                            len53,
                                                                                        );
                                                                                        _rt::string_lift(bytes53)
                                                                                    };
                                                                                    Some(e)
                                                                                }
                                                                                _ => _rt::invalid_enum_discriminant(),
                                                                            },
                                                                            field_size: match l54 {
                                                                                0 => None,
                                                                                1 => {
                                                                                    let e = {
                                                                                        let l55 = *ptr0.add(48).cast::<i32>();
                                                                                        l55 as u32
                                                                                    };
                                                                                    Some(e)
                                                                                }
                                                                                _ => _rt::invalid_enum_discriminant(),
                                                                            },
                                                                        }
                                                                    };
                                                                    ErrorCode::HttpResponseTrailerSize(e68)
                                                                }
                                                                31 => {
                                                                    let e68 = {
                                                                        let l56 = i32::from(*ptr0.add(32).cast::<u8>());
                                                                        match l56 {
                                                                            0 => None,
                                                                            1 => {
                                                                                let e = {
                                                                                    let l57 = *ptr0.add(36).cast::<*mut u8>();
                                                                                    let l58 = *ptr0.add(40).cast::<usize>();
                                                                                    let len59 = l58;
                                                                                    let bytes59 = _rt::Vec::from_raw_parts(
                                                                                        l57.cast(),
                                                                                        len59,
                                                                                        len59,
                                                                                    );
                                                                                    _rt::string_lift(bytes59)
                                                                                };
                                                                                Some(e)
                                                                            }
                                                                            _ => _rt::invalid_enum_discriminant(),
                                                                        }
                                                                    };
                                                                    ErrorCode::HttpResponseTransferCoding(e68)
                                                                }
                                                                32 => {
                                                                    let e68 = {
                                                                        let l60 = i32::from(*ptr0.add(32).cast::<u8>());
                                                                        match l60 {
                                                                            0 => None,
                                                                            1 => {
                                                                                let e = {
                                                                                    let l61 = *ptr0.add(36).cast::<*mut u8>();
                                                                                    let l62 = *ptr0.add(40).cast::<usize>();
                                                                                    let len63 = l62;
                                                                                    let bytes63 = _rt::Vec::from_raw_parts(
                                                                                        l61.cast(),
                                                                                        len63,
                                                                                        len63,
                                                                                    );
                                                                                    _rt::string_lift(bytes63)
                                                                                };
                                                                                Some(e)
                                                                            }
                                                                            _ => _rt::invalid_enum_discriminant(),
                                                                        }
                                                                    };
                                                                    ErrorCode::HttpResponseContentCoding(e68)
                                                                }
                                                                33 => ErrorCode::HttpResponseTimeout,
                                                                34 => ErrorCode::HttpUpgradeFailed,
                                                                35 => ErrorCode::HttpProtocolError,
                                                                36 => ErrorCode::LoopDetected,
                                                                37 => ErrorCode::ConfigurationError,
                                                                n => {
                                                                    debug_assert_eq!(n, 38, "invalid enum discriminant");
                                                                    let e68 = {
                                                                        let l64 = i32::from(*ptr0.add(32).cast::<u8>());
                                                                        match l64 {
                                                                            0 => None,
                                                                            1 => {
                                                                                let e = {
                                                                                    let l65 = *ptr0.add(36).cast::<*mut u8>();
                                                                                    let l66 = *ptr0.add(40).cast::<usize>();
                                                                                    let len67 = l66;
                                                                                    let bytes67 = _rt::Vec::from_raw_parts(
                                                                                        l65.cast(),
                                                                                        len67,
                                                                                        len67,
                                                                                    );
                                                                                    _rt::string_lift(bytes67)
                                                                                };
                                                                                Some(e)
                                                                            }
                                                                            _ => _rt::invalid_enum_discriminant(),
                                                                        }
                                                                    };
                                                                    ErrorCode::InternalError(e68)
                                                                }
                                                            };
                                                            v68
                                                        };
                                                        Err(e)
                                                    }
                                                    _ => _rt::invalid_enum_discriminant(),
                                                }
                                            };
                                            Ok(e)
                                        }
                                        1 => {
                                            let e = ();
                                            Err(e)
                                        }
                                        _ => _rt::invalid_enum_discriminant(),
                                    }
                                };
                                Some(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl OutgoingResponse {
                #[allow(unused_unsafe, clippy::all)]
                pub fn new(headers: Headers) -> Self {
                    unsafe {
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:http/types@0.2.0")]
                        extern "C" {
                            #[link_name = "[constructor]outgoing-response"]
                            fn wit_import(_: i32) -> i32;
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32) -> i32 {
                            unreachable!()
                        }
                        let ret = wit_import((&headers).take_handle() as i32);
                        OutgoingResponse::from_handle(ret as u32)
                    }
                }
            }
            impl OutgoingResponse {
                #[allow(unused_unsafe, clippy::all)]
                pub fn status_code(&self) -> StatusCode {
                    unsafe {
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:http/types@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]outgoing-response.status-code"]
                            fn wit_import(_: i32) -> i32;
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32) -> i32 {
                            unreachable!()
                        }
                        let ret = wit_import((self).handle() as i32);
                        ret as u16
                    }
                }
            }
            impl OutgoingResponse {
                #[allow(unused_unsafe, clippy::all)]
                pub fn set_status_code(&self, status_code: StatusCode) -> Result<(), ()> {
                    unsafe {
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:http/types@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]outgoing-response.set-status-code"]
                            fn wit_import(_: i32, _: i32) -> i32;
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: i32) -> i32 {
                            unreachable!()
                        }
                        let ret = wit_import((self).handle() as i32, _rt::as_i32(status_code));
                        match ret {
                            0 => {
                                let e = ();
                                Ok(e)
                            }
                            1 => {
                                let e = ();
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl OutgoingResponse {
                #[allow(unused_unsafe, clippy::all)]
                pub fn headers(&self) -> Headers {
                    unsafe {
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:http/types@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]outgoing-response.headers"]
                            fn wit_import(_: i32) -> i32;
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32) -> i32 {
                            unreachable!()
                        }
                        let ret = wit_import((self).handle() as i32);
                        Fields::from_handle(ret as u32)
                    }
                }
            }
            impl OutgoingResponse {
                #[allow(unused_unsafe, clippy::all)]
                pub fn body(&self) -> Result<OutgoingBody, ()> {
                    unsafe {
                        #[repr(align(4))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 8]);
                        let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 8]);
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:http/types@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]outgoing-response.body"]
                            fn wit_import(_: i32, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, ptr0);
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        match l1 {
                            0 => {
                                let e = {
                                    let l2 = *ptr0.add(4).cast::<i32>();
                                    OutgoingBody::from_handle(l2 as u32)
                                };
                                Ok(e)
                            }
                            1 => {
                                let e = ();
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl OutgoingBody {
                #[allow(unused_unsafe, clippy::all)]
                pub fn write(&self) -> Result<OutputStream, ()> {
                    unsafe {
                        #[repr(align(4))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 8]);
                        let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 8]);
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:http/types@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]outgoing-body.write"]
                            fn wit_import(_: i32, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, ptr0);
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        match l1 {
                            0 => {
                                let e = {
                                    let l2 = *ptr0.add(4).cast::<i32>();
                                    super::super::super::wasi::io::streams::OutputStream::from_handle(
                                        l2 as u32,
                                    )
                                };
                                Ok(e)
                            }
                            1 => {
                                let e = ();
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl OutgoingBody {
                #[allow(unused_unsafe, clippy::all)]
                pub fn finish(
                    this: OutgoingBody,
                    trailers: Option<Trailers>,
                ) -> Result<(), ErrorCode> {
                    unsafe {
                        #[repr(align(8))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 40]);
                        let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 40]);
                        let (result0_0, result0_1) = match &trailers {
                            Some(e) => (1i32, (e).take_handle() as i32),
                            None => (0i32, 0i32),
                        };
                        let ptr1 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:http/types@0.2.0")]
                        extern "C" {
                            #[link_name = "[static]outgoing-body.finish"]
                            fn wit_import(_: i32, _: i32, _: i32, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: i32, _: i32, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((&this).take_handle() as i32, result0_0, result0_1, ptr1);
                        let l2 = i32::from(*ptr1.add(0).cast::<u8>());
                        match l2 {
                            0 => {
                                let e = ();
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l3 = i32::from(*ptr1.add(8).cast::<u8>());
                                    let v65 = match l3 {
                                        0 => ErrorCode::DnsTimeout,
                                        1 => {
                                            let e65 = {
                                                let l4 = i32::from(*ptr1.add(16).cast::<u8>());
                                                let l8 = i32::from(*ptr1.add(28).cast::<u8>());
                                                DnsErrorPayload {
                                                    rcode: match l4 {
                                                        0 => None,
                                                        1 => {
                                                            let e = {
                                                                let l5 =
                                                                    *ptr1.add(20).cast::<*mut u8>();
                                                                let l6 =
                                                                    *ptr1.add(24).cast::<usize>();
                                                                let len7 = l6;
                                                                let bytes7 =
                                                                    _rt::Vec::from_raw_parts(
                                                                        l5.cast(),
                                                                        len7,
                                                                        len7,
                                                                    );
                                                                _rt::string_lift(bytes7)
                                                            };
                                                            Some(e)
                                                        }
                                                        _ => _rt::invalid_enum_discriminant(),
                                                    },
                                                    info_code: match l8 {
                                                        0 => None,
                                                        1 => {
                                                            let e = {
                                                                let l9 = i32::from(
                                                                    *ptr1.add(30).cast::<u16>(),
                                                                );
                                                                l9 as u16
                                                            };
                                                            Some(e)
                                                        }
                                                        _ => _rt::invalid_enum_discriminant(),
                                                    },
                                                }
                                            };
                                            ErrorCode::DnsError(e65)
                                        }
                                        2 => ErrorCode::DestinationNotFound,
                                        3 => ErrorCode::DestinationUnavailable,
                                        4 => ErrorCode::DestinationIpProhibited,
                                        5 => ErrorCode::DestinationIpUnroutable,
                                        6 => ErrorCode::ConnectionRefused,
                                        7 => ErrorCode::ConnectionTerminated,
                                        8 => ErrorCode::ConnectionTimeout,
                                        9 => ErrorCode::ConnectionReadTimeout,
                                        10 => ErrorCode::ConnectionWriteTimeout,
                                        11 => ErrorCode::ConnectionLimitReached,
                                        12 => ErrorCode::TlsProtocolError,
                                        13 => ErrorCode::TlsCertificateError,
                                        14 => {
                                            let e65 = {
                                                let l10 = i32::from(*ptr1.add(16).cast::<u8>());
                                                let l12 = i32::from(*ptr1.add(20).cast::<u8>());
                                                TlsAlertReceivedPayload {
                                                    alert_id: match l10 {
                                                        0 => None,
                                                        1 => {
                                                            let e = {
                                                                let l11 = i32::from(
                                                                    *ptr1.add(17).cast::<u8>(),
                                                                );
                                                                l11 as u8
                                                            };
                                                            Some(e)
                                                        }
                                                        _ => _rt::invalid_enum_discriminant(),
                                                    },
                                                    alert_message: match l12 {
                                                        0 => None,
                                                        1 => {
                                                            let e = {
                                                                let l13 =
                                                                    *ptr1.add(24).cast::<*mut u8>();
                                                                let l14 =
                                                                    *ptr1.add(28).cast::<usize>();
                                                                let len15 = l14;
                                                                let bytes15 =
                                                                    _rt::Vec::from_raw_parts(
                                                                        l13.cast(),
                                                                        len15,
                                                                        len15,
                                                                    );
                                                                _rt::string_lift(bytes15)
                                                            };
                                                            Some(e)
                                                        }
                                                        _ => _rt::invalid_enum_discriminant(),
                                                    },
                                                }
                                            };
                                            ErrorCode::TlsAlertReceived(e65)
                                        }
                                        15 => ErrorCode::HttpRequestDenied,
                                        16 => ErrorCode::HttpRequestLengthRequired,
                                        17 => {
                                            let e65 = {
                                                let l16 = i32::from(*ptr1.add(16).cast::<u8>());
                                                match l16 {
                                                    0 => None,
                                                    1 => {
                                                        let e = {
                                                            let l17 = *ptr1.add(24).cast::<i64>();
                                                            l17 as u64
                                                        };
                                                        Some(e)
                                                    }
                                                    _ => _rt::invalid_enum_discriminant(),
                                                }
                                            };
                                            ErrorCode::HttpRequestBodySize(e65)
                                        }
                                        18 => ErrorCode::HttpRequestMethodInvalid,
                                        19 => ErrorCode::HttpRequestUriInvalid,
                                        20 => ErrorCode::HttpRequestUriTooLong,
                                        21 => {
                                            let e65 = {
                                                let l18 = i32::from(*ptr1.add(16).cast::<u8>());
                                                match l18 {
                                                    0 => None,
                                                    1 => {
                                                        let e = {
                                                            let l19 = *ptr1.add(20).cast::<i32>();
                                                            l19 as u32
                                                        };
                                                        Some(e)
                                                    }
                                                    _ => _rt::invalid_enum_discriminant(),
                                                }
                                            };
                                            ErrorCode::HttpRequestHeaderSectionSize(e65)
                                        }
                                        22 => {
                                            let e65 = {
                                                let l20 = i32::from(*ptr1.add(16).cast::<u8>());
                                                match l20 {
                                                    0 => None,
                                                    1 => {
                                                        let e = {
                                                            let l21 = i32::from(
                                                                *ptr1.add(20).cast::<u8>(),
                                                            );
                                                            let l25 = i32::from(
                                                                *ptr1.add(32).cast::<u8>(),
                                                            );
                                                            FieldSizePayload {
                                                                field_name: match l21 {
                                                                    0 => None,
                                                                    1 => {
                                                                        let e = {
                                                                            let l22 = *ptr1.add(24).cast::<*mut u8>();
                                                                            let l23 = *ptr1.add(28).cast::<usize>();
                                                                            let len24 = l23;
                                                                            let bytes24 = _rt::Vec::from_raw_parts(
                                                                                l22.cast(),
                                                                                len24,
                                                                                len24,
                                                                            );
                                                                            _rt::string_lift(bytes24)
                                                                        };
                                                                        Some(e)
                                                                    }
                                                                    _ => _rt::invalid_enum_discriminant(),
                                                                },
                                                                field_size: match l25 {
                                                                    0 => None,
                                                                    1 => {
                                                                        let e = {
                                                                            let l26 = *ptr1.add(36).cast::<i32>();
                                                                            l26 as u32
                                                                        };
                                                                        Some(e)
                                                                    }
                                                                    _ => _rt::invalid_enum_discriminant(),
                                                                },
                                                            }
                                                        };
                                                        Some(e)
                                                    }
                                                    _ => _rt::invalid_enum_discriminant(),
                                                }
                                            };
                                            ErrorCode::HttpRequestHeaderSize(e65)
                                        }
                                        23 => {
                                            let e65 = {
                                                let l27 = i32::from(*ptr1.add(16).cast::<u8>());
                                                match l27 {
                                                    0 => None,
                                                    1 => {
                                                        let e = {
                                                            let l28 = *ptr1.add(20).cast::<i32>();
                                                            l28 as u32
                                                        };
                                                        Some(e)
                                                    }
                                                    _ => _rt::invalid_enum_discriminant(),
                                                }
                                            };
                                            ErrorCode::HttpRequestTrailerSectionSize(e65)
                                        }
                                        24 => {
                                            let e65 = {
                                                let l29 = i32::from(*ptr1.add(16).cast::<u8>());
                                                let l33 = i32::from(*ptr1.add(28).cast::<u8>());
                                                FieldSizePayload {
                                                    field_name: match l29 {
                                                        0 => None,
                                                        1 => {
                                                            let e = {
                                                                let l30 =
                                                                    *ptr1.add(20).cast::<*mut u8>();
                                                                let l31 =
                                                                    *ptr1.add(24).cast::<usize>();
                                                                let len32 = l31;
                                                                let bytes32 =
                                                                    _rt::Vec::from_raw_parts(
                                                                        l30.cast(),
                                                                        len32,
                                                                        len32,
                                                                    );
                                                                _rt::string_lift(bytes32)
                                                            };
                                                            Some(e)
                                                        }
                                                        _ => _rt::invalid_enum_discriminant(),
                                                    },
                                                    field_size: match l33 {
                                                        0 => None,
                                                        1 => {
                                                            let e = {
                                                                let l34 =
                                                                    *ptr1.add(32).cast::<i32>();
                                                                l34 as u32
                                                            };
                                                            Some(e)
                                                        }
                                                        _ => _rt::invalid_enum_discriminant(),
                                                    },
                                                }
                                            };
                                            ErrorCode::HttpRequestTrailerSize(e65)
                                        }
                                        25 => ErrorCode::HttpResponseIncomplete,
                                        26 => {
                                            let e65 = {
                                                let l35 = i32::from(*ptr1.add(16).cast::<u8>());
                                                match l35 {
                                                    0 => None,
                                                    1 => {
                                                        let e = {
                                                            let l36 = *ptr1.add(20).cast::<i32>();
                                                            l36 as u32
                                                        };
                                                        Some(e)
                                                    }
                                                    _ => _rt::invalid_enum_discriminant(),
                                                }
                                            };
                                            ErrorCode::HttpResponseHeaderSectionSize(e65)
                                        }
                                        27 => {
                                            let e65 = {
                                                let l37 = i32::from(*ptr1.add(16).cast::<u8>());
                                                let l41 = i32::from(*ptr1.add(28).cast::<u8>());
                                                FieldSizePayload {
                                                    field_name: match l37 {
                                                        0 => None,
                                                        1 => {
                                                            let e = {
                                                                let l38 =
                                                                    *ptr1.add(20).cast::<*mut u8>();
                                                                let l39 =
                                                                    *ptr1.add(24).cast::<usize>();
                                                                let len40 = l39;
                                                                let bytes40 =
                                                                    _rt::Vec::from_raw_parts(
                                                                        l38.cast(),
                                                                        len40,
                                                                        len40,
                                                                    );
                                                                _rt::string_lift(bytes40)
                                                            };
                                                            Some(e)
                                                        }
                                                        _ => _rt::invalid_enum_discriminant(),
                                                    },
                                                    field_size: match l41 {
                                                        0 => None,
                                                        1 => {
                                                            let e = {
                                                                let l42 =
                                                                    *ptr1.add(32).cast::<i32>();
                                                                l42 as u32
                                                            };
                                                            Some(e)
                                                        }
                                                        _ => _rt::invalid_enum_discriminant(),
                                                    },
                                                }
                                            };
                                            ErrorCode::HttpResponseHeaderSize(e65)
                                        }
                                        28 => {
                                            let e65 = {
                                                let l43 = i32::from(*ptr1.add(16).cast::<u8>());
                                                match l43 {
                                                    0 => None,
                                                    1 => {
                                                        let e = {
                                                            let l44 = *ptr1.add(24).cast::<i64>();
                                                            l44 as u64
                                                        };
                                                        Some(e)
                                                    }
                                                    _ => _rt::invalid_enum_discriminant(),
                                                }
                                            };
                                            ErrorCode::HttpResponseBodySize(e65)
                                        }
                                        29 => {
                                            let e65 = {
                                                let l45 = i32::from(*ptr1.add(16).cast::<u8>());
                                                match l45 {
                                                    0 => None,
                                                    1 => {
                                                        let e = {
                                                            let l46 = *ptr1.add(20).cast::<i32>();
                                                            l46 as u32
                                                        };
                                                        Some(e)
                                                    }
                                                    _ => _rt::invalid_enum_discriminant(),
                                                }
                                            };
                                            ErrorCode::HttpResponseTrailerSectionSize(e65)
                                        }
                                        30 => {
                                            let e65 = {
                                                let l47 = i32::from(*ptr1.add(16).cast::<u8>());
                                                let l51 = i32::from(*ptr1.add(28).cast::<u8>());
                                                FieldSizePayload {
                                                    field_name: match l47 {
                                                        0 => None,
                                                        1 => {
                                                            let e = {
                                                                let l48 =
                                                                    *ptr1.add(20).cast::<*mut u8>();
                                                                let l49 =
                                                                    *ptr1.add(24).cast::<usize>();
                                                                let len50 = l49;
                                                                let bytes50 =
                                                                    _rt::Vec::from_raw_parts(
                                                                        l48.cast(),
                                                                        len50,
                                                                        len50,
                                                                    );
                                                                _rt::string_lift(bytes50)
                                                            };
                                                            Some(e)
                                                        }
                                                        _ => _rt::invalid_enum_discriminant(),
                                                    },
                                                    field_size: match l51 {
                                                        0 => None,
                                                        1 => {
                                                            let e = {
                                                                let l52 =
                                                                    *ptr1.add(32).cast::<i32>();
                                                                l52 as u32
                                                            };
                                                            Some(e)
                                                        }
                                                        _ => _rt::invalid_enum_discriminant(),
                                                    },
                                                }
                                            };
                                            ErrorCode::HttpResponseTrailerSize(e65)
                                        }
                                        31 => {
                                            let e65 = {
                                                let l53 = i32::from(*ptr1.add(16).cast::<u8>());
                                                match l53 {
                                                    0 => None,
                                                    1 => {
                                                        let e = {
                                                            let l54 =
                                                                *ptr1.add(20).cast::<*mut u8>();
                                                            let l55 = *ptr1.add(24).cast::<usize>();
                                                            let len56 = l55;
                                                            let bytes56 = _rt::Vec::from_raw_parts(
                                                                l54.cast(),
                                                                len56,
                                                                len56,
                                                            );
                                                            _rt::string_lift(bytes56)
                                                        };
                                                        Some(e)
                                                    }
                                                    _ => _rt::invalid_enum_discriminant(),
                                                }
                                            };
                                            ErrorCode::HttpResponseTransferCoding(e65)
                                        }
                                        32 => {
                                            let e65 = {
                                                let l57 = i32::from(*ptr1.add(16).cast::<u8>());
                                                match l57 {
                                                    0 => None,
                                                    1 => {
                                                        let e = {
                                                            let l58 =
                                                                *ptr1.add(20).cast::<*mut u8>();
                                                            let l59 = *ptr1.add(24).cast::<usize>();
                                                            let len60 = l59;
                                                            let bytes60 = _rt::Vec::from_raw_parts(
                                                                l58.cast(),
                                                                len60,
                                                                len60,
                                                            );
                                                            _rt::string_lift(bytes60)
                                                        };
                                                        Some(e)
                                                    }
                                                    _ => _rt::invalid_enum_discriminant(),
                                                }
                                            };
                                            ErrorCode::HttpResponseContentCoding(e65)
                                        }
                                        33 => ErrorCode::HttpResponseTimeout,
                                        34 => ErrorCode::HttpUpgradeFailed,
                                        35 => ErrorCode::HttpProtocolError,
                                        36 => ErrorCode::LoopDetected,
                                        37 => ErrorCode::ConfigurationError,
                                        n => {
                                            debug_assert_eq!(n, 38, "invalid enum discriminant");
                                            let e65 = {
                                                let l61 = i32::from(*ptr1.add(16).cast::<u8>());
                                                match l61 {
                                                    0 => None,
                                                    1 => {
                                                        let e = {
                                                            let l62 =
                                                                *ptr1.add(20).cast::<*mut u8>();
                                                            let l63 = *ptr1.add(24).cast::<usize>();
                                                            let len64 = l63;
                                                            let bytes64 = _rt::Vec::from_raw_parts(
                                                                l62.cast(),
                                                                len64,
                                                                len64,
                                                            );
                                                            _rt::string_lift(bytes64)
                                                        };
                                                        Some(e)
                                                    }
                                                    _ => _rt::invalid_enum_discriminant(),
                                                }
                                            };
                                            ErrorCode::InternalError(e65)
                                        }
                                    };
                                    v65
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl FutureIncomingResponse {
                #[allow(unused_unsafe, clippy::all)]
                pub fn subscribe(&self) -> Pollable {
                    unsafe {
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:http/types@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]future-incoming-response.subscribe"]
                            fn wit_import(_: i32) -> i32;
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32) -> i32 {
                            unreachable!()
                        }
                        let ret = wit_import((self).handle() as i32);
                        super::super::super::wasi::io::poll::Pollable::from_handle(ret as u32)
                    }
                }
            }
            impl FutureIncomingResponse {
                #[allow(unused_unsafe, clippy::all)]
                pub fn get(&self) -> Option<Result<Result<IncomingResponse, ErrorCode>, ()>> {
                    unsafe {
                        #[repr(align(8))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 56]);
                        let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 56]);
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:http/types@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]future-incoming-response.get"]
                            fn wit_import(_: i32, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, ptr0);
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        match l1 {
                            0 => None,
                            1 => {
                                let e = {
                                    let l2 = i32::from(*ptr0.add(8).cast::<u8>());
                                    match l2 {
                                        0 => {
                                            let e = {
                                                let l3 = i32::from(*ptr0.add(16).cast::<u8>());
                                                match l3 {
                                                    0 => {
                                                        let e = {
                                                            let l4 = *ptr0.add(24).cast::<i32>();
                                                            IncomingResponse::from_handle(l4 as u32)
                                                        };
                                                        Ok(e)
                                                    }
                                                    1 => {
                                                        let e = {
                                                            let l5 = i32::from(
                                                                *ptr0.add(24).cast::<u8>(),
                                                            );
                                                            let v67 = match l5 {
                                                                0 => ErrorCode::DnsTimeout,
                                                                1 => {
                                                                    let e67 = {
                                                                        let l6 = i32::from(*ptr0.add(32).cast::<u8>());
                                                                        let l10 = i32::from(*ptr0.add(44).cast::<u8>());
                                                                        DnsErrorPayload {
                                                                            rcode: match l6 {
                                                                                0 => None,
                                                                                1 => {
                                                                                    let e = {
                                                                                        let l7 = *ptr0.add(36).cast::<*mut u8>();
                                                                                        let l8 = *ptr0.add(40).cast::<usize>();
                                                                                        let len9 = l8;
                                                                                        let bytes9 = _rt::Vec::from_raw_parts(
                                                                                            l7.cast(),
                                                                                            len9,
                                                                                            len9,
                                                                                        );
                                                                                        _rt::string_lift(bytes9)
                                                                                    };
                                                                                    Some(e)
                                                                                }
                                                                                _ => _rt::invalid_enum_discriminant(),
                                                                            },
                                                                            info_code: match l10 {
                                                                                0 => None,
                                                                                1 => {
                                                                                    let e = {
                                                                                        let l11 = i32::from(*ptr0.add(46).cast::<u16>());
                                                                                        l11 as u16
                                                                                    };
                                                                                    Some(e)
                                                                                }
                                                                                _ => _rt::invalid_enum_discriminant(),
                                                                            },
                                                                        }
                                                                    };
                                                                    ErrorCode::DnsError(e67)
                                                                }
                                                                2 => ErrorCode::DestinationNotFound,
                                                                3 => ErrorCode::DestinationUnavailable,
                                                                4 => ErrorCode::DestinationIpProhibited,
                                                                5 => ErrorCode::DestinationIpUnroutable,
                                                                6 => ErrorCode::ConnectionRefused,
                                                                7 => ErrorCode::ConnectionTerminated,
                                                                8 => ErrorCode::ConnectionTimeout,
                                                                9 => ErrorCode::ConnectionReadTimeout,
                                                                10 => ErrorCode::ConnectionWriteTimeout,
                                                                11 => ErrorCode::ConnectionLimitReached,
                                                                12 => ErrorCode::TlsProtocolError,
                                                                13 => ErrorCode::TlsCertificateError,
                                                                14 => {
                                                                    let e67 = {
                                                                        let l12 = i32::from(*ptr0.add(32).cast::<u8>());
                                                                        let l14 = i32::from(*ptr0.add(36).cast::<u8>());
                                                                        TlsAlertReceivedPayload {
                                                                            alert_id: match l12 {
                                                                                0 => None,
                                                                                1 => {
                                                                                    let e = {
                                                                                        let l13 = i32::from(*ptr0.add(33).cast::<u8>());
                                                                                        l13 as u8
                                                                                    };
                                                                                    Some(e)
                                                                                }
                                                                                _ => _rt::invalid_enum_discriminant(),
                                                                            },
                                                                            alert_message: match l14 {
                                                                                0 => None,
                                                                                1 => {
                                                                                    let e = {
                                                                                        let l15 = *ptr0.add(40).cast::<*mut u8>();
                                                                                        let l16 = *ptr0.add(44).cast::<usize>();
                                                                                        let len17 = l16;
                                                                                        let bytes17 = _rt::Vec::from_raw_parts(
                                                                                            l15.cast(),
                                                                                            len17,
                                                                                            len17,
                                                                                        );
                                                                                        _rt::string_lift(bytes17)
                                                                                    };
                                                                                    Some(e)
                                                                                }
                                                                                _ => _rt::invalid_enum_discriminant(),
                                                                            },
                                                                        }
                                                                    };
                                                                    ErrorCode::TlsAlertReceived(e67)
                                                                }
                                                                15 => ErrorCode::HttpRequestDenied,
                                                                16 => ErrorCode::HttpRequestLengthRequired,
                                                                17 => {
                                                                    let e67 = {
                                                                        let l18 = i32::from(*ptr0.add(32).cast::<u8>());
                                                                        match l18 {
                                                                            0 => None,
                                                                            1 => {
                                                                                let e = {
                                                                                    let l19 = *ptr0.add(40).cast::<i64>();
                                                                                    l19 as u64
                                                                                };
                                                                                Some(e)
                                                                            }
                                                                            _ => _rt::invalid_enum_discriminant(),
                                                                        }
                                                                    };
                                                                    ErrorCode::HttpRequestBodySize(e67)
                                                                }
                                                                18 => ErrorCode::HttpRequestMethodInvalid,
                                                                19 => ErrorCode::HttpRequestUriInvalid,
                                                                20 => ErrorCode::HttpRequestUriTooLong,
                                                                21 => {
                                                                    let e67 = {
                                                                        let l20 = i32::from(*ptr0.add(32).cast::<u8>());
                                                                        match l20 {
                                                                            0 => None,
                                                                            1 => {
                                                                                let e = {
                                                                                    let l21 = *ptr0.add(36).cast::<i32>();
                                                                                    l21 as u32
                                                                                };
                                                                                Some(e)
                                                                            }
                                                                            _ => _rt::invalid_enum_discriminant(),
                                                                        }
                                                                    };
                                                                    ErrorCode::HttpRequestHeaderSectionSize(e67)
                                                                }
                                                                22 => {
                                                                    let e67 = {
                                                                        let l22 = i32::from(*ptr0.add(32).cast::<u8>());
                                                                        match l22 {
                                                                            0 => None,
                                                                            1 => {
                                                                                let e = {
                                                                                    let l23 = i32::from(*ptr0.add(36).cast::<u8>());
                                                                                    let l27 = i32::from(*ptr0.add(48).cast::<u8>());
                                                                                    FieldSizePayload {
                                                                                        field_name: match l23 {
                                                                                            0 => None,
                                                                                            1 => {
                                                                                                let e = {
                                                                                                    let l24 = *ptr0.add(40).cast::<*mut u8>();
                                                                                                    let l25 = *ptr0.add(44).cast::<usize>();
                                                                                                    let len26 = l25;
                                                                                                    let bytes26 = _rt::Vec::from_raw_parts(
                                                                                                        l24.cast(),
                                                                                                        len26,
                                                                                                        len26,
                                                                                                    );
                                                                                                    _rt::string_lift(bytes26)
                                                                                                };
                                                                                                Some(e)
                                                                                            }
                                                                                            _ => _rt::invalid_enum_discriminant(),
                                                                                        },
                                                                                        field_size: match l27 {
                                                                                            0 => None,
                                                                                            1 => {
                                                                                                let e = {
                                                                                                    let l28 = *ptr0.add(52).cast::<i32>();
                                                                                                    l28 as u32
                                                                                                };
                                                                                                Some(e)
                                                                                            }
                                                                                            _ => _rt::invalid_enum_discriminant(),
                                                                                        },
                                                                                    }
                                                                                };
                                                                                Some(e)
                                                                            }
                                                                            _ => _rt::invalid_enum_discriminant(),
                                                                        }
                                                                    };
                                                                    ErrorCode::HttpRequestHeaderSize(e67)
                                                                }
                                                                23 => {
                                                                    let e67 = {
                                                                        let l29 = i32::from(*ptr0.add(32).cast::<u8>());
                                                                        match l29 {
                                                                            0 => None,
                                                                            1 => {
                                                                                let e = {
                                                                                    let l30 = *ptr0.add(36).cast::<i32>();
                                                                                    l30 as u32
                                                                                };
                                                                                Some(e)
                                                                            }
                                                                            _ => _rt::invalid_enum_discriminant(),
                                                                        }
                                                                    };
                                                                    ErrorCode::HttpRequestTrailerSectionSize(e67)
                                                                }
                                                                24 => {
                                                                    let e67 = {
                                                                        let l31 = i32::from(*ptr0.add(32).cast::<u8>());
                                                                        let l35 = i32::from(*ptr0.add(44).cast::<u8>());
                                                                        FieldSizePayload {
                                                                            field_name: match l31 {
                                                                                0 => None,
                                                                                1 => {
                                                                                    let e = {
                                                                                        let l32 = *ptr0.add(36).cast::<*mut u8>();
                                                                                        let l33 = *ptr0.add(40).cast::<usize>();
                                                                                        let len34 = l33;
                                                                                        let bytes34 = _rt::Vec::from_raw_parts(
                                                                                            l32.cast(),
                                                                                            len34,
                                                                                            len34,
                                                                                        );
                                                                                        _rt::string_lift(bytes34)
                                                                                    };
                                                                                    Some(e)
                                                                                }
                                                                                _ => _rt::invalid_enum_discriminant(),
                                                                            },
                                                                            field_size: match l35 {
                                                                                0 => None,
                                                                                1 => {
                                                                                    let e = {
                                                                                        let l36 = *ptr0.add(48).cast::<i32>();
                                                                                        l36 as u32
                                                                                    };
                                                                                    Some(e)
                                                                                }
                                                                                _ => _rt::invalid_enum_discriminant(),
                                                                            },
                                                                        }
                                                                    };
                                                                    ErrorCode::HttpRequestTrailerSize(e67)
                                                                }
                                                                25 => ErrorCode::HttpResponseIncomplete,
                                                                26 => {
                                                                    let e67 = {
                                                                        let l37 = i32::from(*ptr0.add(32).cast::<u8>());
                                                                        match l37 {
                                                                            0 => None,
                                                                            1 => {
                                                                                let e = {
                                                                                    let l38 = *ptr0.add(36).cast::<i32>();
                                                                                    l38 as u32
                                                                                };
                                                                                Some(e)
                                                                            }
                                                                            _ => _rt::invalid_enum_discriminant(),
                                                                        }
                                                                    };
                                                                    ErrorCode::HttpResponseHeaderSectionSize(e67)
                                                                }
                                                                27 => {
                                                                    let e67 = {
                                                                        let l39 = i32::from(*ptr0.add(32).cast::<u8>());
                                                                        let l43 = i32::from(*ptr0.add(44).cast::<u8>());
                                                                        FieldSizePayload {
                                                                            field_name: match l39 {
                                                                                0 => None,
                                                                                1 => {
                                                                                    let e = {
                                                                                        let l40 = *ptr0.add(36).cast::<*mut u8>();
                                                                                        let l41 = *ptr0.add(40).cast::<usize>();
                                                                                        let len42 = l41;
                                                                                        let bytes42 = _rt::Vec::from_raw_parts(
                                                                                            l40.cast(),
                                                                                            len42,
                                                                                            len42,
                                                                                        );
                                                                                        _rt::string_lift(bytes42)
                                                                                    };
                                                                                    Some(e)
                                                                                }
                                                                                _ => _rt::invalid_enum_discriminant(),
                                                                            },
                                                                            field_size: match l43 {
                                                                                0 => None,
                                                                                1 => {
                                                                                    let e = {
                                                                                        let l44 = *ptr0.add(48).cast::<i32>();
                                                                                        l44 as u32
                                                                                    };
                                                                                    Some(e)
                                                                                }
                                                                                _ => _rt::invalid_enum_discriminant(),
                                                                            },
                                                                        }
                                                                    };
                                                                    ErrorCode::HttpResponseHeaderSize(e67)
                                                                }
                                                                28 => {
                                                                    let e67 = {
                                                                        let l45 = i32::from(*ptr0.add(32).cast::<u8>());
                                                                        match l45 {
                                                                            0 => None,
                                                                            1 => {
                                                                                let e = {
                                                                                    let l46 = *ptr0.add(40).cast::<i64>();
                                                                                    l46 as u64
                                                                                };
                                                                                Some(e)
                                                                            }
                                                                            _ => _rt::invalid_enum_discriminant(),
                                                                        }
                                                                    };
                                                                    ErrorCode::HttpResponseBodySize(e67)
                                                                }
                                                                29 => {
                                                                    let e67 = {
                                                                        let l47 = i32::from(*ptr0.add(32).cast::<u8>());
                                                                        match l47 {
                                                                            0 => None,
                                                                            1 => {
                                                                                let e = {
                                                                                    let l48 = *ptr0.add(36).cast::<i32>();
                                                                                    l48 as u32
                                                                                };
                                                                                Some(e)
                                                                            }
                                                                            _ => _rt::invalid_enum_discriminant(),
                                                                        }
                                                                    };
                                                                    ErrorCode::HttpResponseTrailerSectionSize(e67)
                                                                }
                                                                30 => {
                                                                    let e67 = {
                                                                        let l49 = i32::from(*ptr0.add(32).cast::<u8>());
                                                                        let l53 = i32::from(*ptr0.add(44).cast::<u8>());
                                                                        FieldSizePayload {
                                                                            field_name: match l49 {
                                                                                0 => None,
                                                                                1 => {
                                                                                    let e = {
                                                                                        let l50 = *ptr0.add(36).cast::<*mut u8>();
                                                                                        let l51 = *ptr0.add(40).cast::<usize>();
                                                                                        let len52 = l51;
                                                                                        let bytes52 = _rt::Vec::from_raw_parts(
                                                                                            l50.cast(),
                                                                                            len52,
                                                                                            len52,
                                                                                        );
                                                                                        _rt::string_lift(bytes52)
                                                                                    };
                                                                                    Some(e)
                                                                                }
                                                                                _ => _rt::invalid_enum_discriminant(),
                                                                            },
                                                                            field_size: match l53 {
                                                                                0 => None,
                                                                                1 => {
                                                                                    let e = {
                                                                                        let l54 = *ptr0.add(48).cast::<i32>();
                                                                                        l54 as u32
                                                                                    };
                                                                                    Some(e)
                                                                                }
                                                                                _ => _rt::invalid_enum_discriminant(),
                                                                            },
                                                                        }
                                                                    };
                                                                    ErrorCode::HttpResponseTrailerSize(e67)
                                                                }
                                                                31 => {
                                                                    let e67 = {
                                                                        let l55 = i32::from(*ptr0.add(32).cast::<u8>());
                                                                        match l55 {
                                                                            0 => None,
                                                                            1 => {
                                                                                let e = {
                                                                                    let l56 = *ptr0.add(36).cast::<*mut u8>();
                                                                                    let l57 = *ptr0.add(40).cast::<usize>();
                                                                                    let len58 = l57;
                                                                                    let bytes58 = _rt::Vec::from_raw_parts(
                                                                                        l56.cast(),
                                                                                        len58,
                                                                                        len58,
                                                                                    );
                                                                                    _rt::string_lift(bytes58)
                                                                                };
                                                                                Some(e)
                                                                            }
                                                                            _ => _rt::invalid_enum_discriminant(),
                                                                        }
                                                                    };
                                                                    ErrorCode::HttpResponseTransferCoding(e67)
                                                                }
                                                                32 => {
                                                                    let e67 = {
                                                                        let l59 = i32::from(*ptr0.add(32).cast::<u8>());
                                                                        match l59 {
                                                                            0 => None,
                                                                            1 => {
                                                                                let e = {
                                                                                    let l60 = *ptr0.add(36).cast::<*mut u8>();
                                                                                    let l61 = *ptr0.add(40).cast::<usize>();
                                                                                    let len62 = l61;
                                                                                    let bytes62 = _rt::Vec::from_raw_parts(
                                                                                        l60.cast(),
                                                                                        len62,
                                                                                        len62,
                                                                                    );
                                                                                    _rt::string_lift(bytes62)
                                                                                };
                                                                                Some(e)
                                                                            }
                                                                            _ => _rt::invalid_enum_discriminant(),
                                                                        }
                                                                    };
                                                                    ErrorCode::HttpResponseContentCoding(e67)
                                                                }
                                                                33 => ErrorCode::HttpResponseTimeout,
                                                                34 => ErrorCode::HttpUpgradeFailed,
                                                                35 => ErrorCode::HttpProtocolError,
                                                                36 => ErrorCode::LoopDetected,
                                                                37 => ErrorCode::ConfigurationError,
                                                                n => {
                                                                    debug_assert_eq!(n, 38, "invalid enum discriminant");
                                                                    let e67 = {
                                                                        let l63 = i32::from(*ptr0.add(32).cast::<u8>());
                                                                        match l63 {
                                                                            0 => None,
                                                                            1 => {
                                                                                let e = {
                                                                                    let l64 = *ptr0.add(36).cast::<*mut u8>();
                                                                                    let l65 = *ptr0.add(40).cast::<usize>();
                                                                                    let len66 = l65;
                                                                                    let bytes66 = _rt::Vec::from_raw_parts(
                                                                                        l64.cast(),
                                                                                        len66,
                                                                                        len66,
                                                                                    );
                                                                                    _rt::string_lift(bytes66)
                                                                                };
                                                                                Some(e)
                                                                            }
                                                                            _ => _rt::invalid_enum_discriminant(),
                                                                        }
                                                                    };
                                                                    ErrorCode::InternalError(e67)
                                                                }
                                                            };
                                                            v67
                                                        };
                                                        Err(e)
                                                    }
                                                    _ => _rt::invalid_enum_discriminant(),
                                                }
                                            };
                                            Ok(e)
                                        }
                                        1 => {
                                            let e = ();
                                            Err(e)
                                        }
                                        _ => _rt::invalid_enum_discriminant(),
                                    }
                                };
                                Some(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            #[allow(unused_unsafe, clippy::all)]
            pub fn http_error_code(err: &IoError) -> Option<ErrorCode> {
                unsafe {
                    #[repr(align(8))]
                    struct RetArea([::core::mem::MaybeUninit<u8>; 40]);
                    let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 40]);
                    let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                    #[cfg(target_arch = "wasm32")]
                    #[link(wasm_import_module = "wasi:http/types@0.2.0")]
                    extern "C" {
                        #[link_name = "http-error-code"]
                        fn wit_import(_: i32, _: *mut u8);
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import(_: i32, _: *mut u8) {
                        unreachable!()
                    }
                    wit_import((err).handle() as i32, ptr0);
                    let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                    match l1 {
                        0 => None,
                        1 => {
                            let e = {
                                let l2 = i32::from(*ptr0.add(8).cast::<u8>());
                                let v64 = match l2 {
                                    0 => ErrorCode::DnsTimeout,
                                    1 => {
                                        let e64 = {
                                            let l3 = i32::from(*ptr0.add(16).cast::<u8>());
                                            let l7 = i32::from(*ptr0.add(28).cast::<u8>());
                                            DnsErrorPayload {
                                                rcode: match l3 {
                                                    0 => None,
                                                    1 => {
                                                        let e = {
                                                            let l4 =
                                                                *ptr0.add(20).cast::<*mut u8>();
                                                            let l5 = *ptr0.add(24).cast::<usize>();
                                                            let len6 = l5;
                                                            let bytes6 = _rt::Vec::from_raw_parts(
                                                                l4.cast(),
                                                                len6,
                                                                len6,
                                                            );
                                                            _rt::string_lift(bytes6)
                                                        };
                                                        Some(e)
                                                    }
                                                    _ => _rt::invalid_enum_discriminant(),
                                                },
                                                info_code: match l7 {
                                                    0 => None,
                                                    1 => {
                                                        let e = {
                                                            let l8 = i32::from(
                                                                *ptr0.add(30).cast::<u16>(),
                                                            );
                                                            l8 as u16
                                                        };
                                                        Some(e)
                                                    }
                                                    _ => _rt::invalid_enum_discriminant(),
                                                },
                                            }
                                        };
                                        ErrorCode::DnsError(e64)
                                    }
                                    2 => ErrorCode::DestinationNotFound,
                                    3 => ErrorCode::DestinationUnavailable,
                                    4 => ErrorCode::DestinationIpProhibited,
                                    5 => ErrorCode::DestinationIpUnroutable,
                                    6 => ErrorCode::ConnectionRefused,
                                    7 => ErrorCode::ConnectionTerminated,
                                    8 => ErrorCode::ConnectionTimeout,
                                    9 => ErrorCode::ConnectionReadTimeout,
                                    10 => ErrorCode::ConnectionWriteTimeout,
                                    11 => ErrorCode::ConnectionLimitReached,
                                    12 => ErrorCode::TlsProtocolError,
                                    13 => ErrorCode::TlsCertificateError,
                                    14 => {
                                        let e64 = {
                                            let l9 = i32::from(*ptr0.add(16).cast::<u8>());
                                            let l11 = i32::from(*ptr0.add(20).cast::<u8>());
                                            TlsAlertReceivedPayload {
                                                alert_id: match l9 {
                                                    0 => None,
                                                    1 => {
                                                        let e = {
                                                            let l10 = i32::from(
                                                                *ptr0.add(17).cast::<u8>(),
                                                            );
                                                            l10 as u8
                                                        };
                                                        Some(e)
                                                    }
                                                    _ => _rt::invalid_enum_discriminant(),
                                                },
                                                alert_message: match l11 {
                                                    0 => None,
                                                    1 => {
                                                        let e = {
                                                            let l12 =
                                                                *ptr0.add(24).cast::<*mut u8>();
                                                            let l13 = *ptr0.add(28).cast::<usize>();
                                                            let len14 = l13;
                                                            let bytes14 = _rt::Vec::from_raw_parts(
                                                                l12.cast(),
                                                                len14,
                                                                len14,
                                                            );
                                                            _rt::string_lift(bytes14)
                                                        };
                                                        Some(e)
                                                    }
                                                    _ => _rt::invalid_enum_discriminant(),
                                                },
                                            }
                                        };
                                        ErrorCode::TlsAlertReceived(e64)
                                    }
                                    15 => ErrorCode::HttpRequestDenied,
                                    16 => ErrorCode::HttpRequestLengthRequired,
                                    17 => {
                                        let e64 = {
                                            let l15 = i32::from(*ptr0.add(16).cast::<u8>());
                                            match l15 {
                                                0 => None,
                                                1 => {
                                                    let e = {
                                                        let l16 = *ptr0.add(24).cast::<i64>();
                                                        l16 as u64
                                                    };
                                                    Some(e)
                                                }
                                                _ => _rt::invalid_enum_discriminant(),
                                            }
                                        };
                                        ErrorCode::HttpRequestBodySize(e64)
                                    }
                                    18 => ErrorCode::HttpRequestMethodInvalid,
                                    19 => ErrorCode::HttpRequestUriInvalid,
                                    20 => ErrorCode::HttpRequestUriTooLong,
                                    21 => {
                                        let e64 = {
                                            let l17 = i32::from(*ptr0.add(16).cast::<u8>());
                                            match l17 {
                                                0 => None,
                                                1 => {
                                                    let e = {
                                                        let l18 = *ptr0.add(20).cast::<i32>();
                                                        l18 as u32
                                                    };
                                                    Some(e)
                                                }
                                                _ => _rt::invalid_enum_discriminant(),
                                            }
                                        };
                                        ErrorCode::HttpRequestHeaderSectionSize(e64)
                                    }
                                    22 => {
                                        let e64 = {
                                            let l19 = i32::from(*ptr0.add(16).cast::<u8>());
                                            match l19 {
                                                0 => None,
                                                1 => {
                                                    let e = {
                                                        let l20 =
                                                            i32::from(*ptr0.add(20).cast::<u8>());
                                                        let l24 =
                                                            i32::from(*ptr0.add(32).cast::<u8>());
                                                        FieldSizePayload {
                                                            field_name: match l20 {
                                                                0 => None,
                                                                1 => {
                                                                    let e = {
                                                                        let l21 = *ptr0
                                                                            .add(24)
                                                                            .cast::<*mut u8>(
                                                                        );
                                                                        let l22 = *ptr0
                                                                            .add(28)
                                                                            .cast::<usize>();
                                                                        let len23 = l22;
                                                                        let bytes23 = _rt::Vec::from_raw_parts(
                                                                            l21.cast(),
                                                                            len23,
                                                                            len23,
                                                                        );
                                                                        _rt::string_lift(bytes23)
                                                                    };
                                                                    Some(e)
                                                                }
                                                                _ => {
                                                                    _rt::invalid_enum_discriminant()
                                                                }
                                                            },
                                                            field_size: match l24 {
                                                                0 => None,
                                                                1 => {
                                                                    let e = {
                                                                        let l25 = *ptr0
                                                                            .add(36)
                                                                            .cast::<i32>();
                                                                        l25 as u32
                                                                    };
                                                                    Some(e)
                                                                }
                                                                _ => {
                                                                    _rt::invalid_enum_discriminant()
                                                                }
                                                            },
                                                        }
                                                    };
                                                    Some(e)
                                                }
                                                _ => _rt::invalid_enum_discriminant(),
                                            }
                                        };
                                        ErrorCode::HttpRequestHeaderSize(e64)
                                    }
                                    23 => {
                                        let e64 = {
                                            let l26 = i32::from(*ptr0.add(16).cast::<u8>());
                                            match l26 {
                                                0 => None,
                                                1 => {
                                                    let e = {
                                                        let l27 = *ptr0.add(20).cast::<i32>();
                                                        l27 as u32
                                                    };
                                                    Some(e)
                                                }
                                                _ => _rt::invalid_enum_discriminant(),
                                            }
                                        };
                                        ErrorCode::HttpRequestTrailerSectionSize(e64)
                                    }
                                    24 => {
                                        let e64 = {
                                            let l28 = i32::from(*ptr0.add(16).cast::<u8>());
                                            let l32 = i32::from(*ptr0.add(28).cast::<u8>());
                                            FieldSizePayload {
                                                field_name: match l28 {
                                                    0 => None,
                                                    1 => {
                                                        let e = {
                                                            let l29 =
                                                                *ptr0.add(20).cast::<*mut u8>();
                                                            let l30 = *ptr0.add(24).cast::<usize>();
                                                            let len31 = l30;
                                                            let bytes31 = _rt::Vec::from_raw_parts(
                                                                l29.cast(),
                                                                len31,
                                                                len31,
                                                            );
                                                            _rt::string_lift(bytes31)
                                                        };
                                                        Some(e)
                                                    }
                                                    _ => _rt::invalid_enum_discriminant(),
                                                },
                                                field_size: match l32 {
                                                    0 => None,
                                                    1 => {
                                                        let e = {
                                                            let l33 = *ptr0.add(32).cast::<i32>();
                                                            l33 as u32
                                                        };
                                                        Some(e)
                                                    }
                                                    _ => _rt::invalid_enum_discriminant(),
                                                },
                                            }
                                        };
                                        ErrorCode::HttpRequestTrailerSize(e64)
                                    }
                                    25 => ErrorCode::HttpResponseIncomplete,
                                    26 => {
                                        let e64 = {
                                            let l34 = i32::from(*ptr0.add(16).cast::<u8>());
                                            match l34 {
                                                0 => None,
                                                1 => {
                                                    let e = {
                                                        let l35 = *ptr0.add(20).cast::<i32>();
                                                        l35 as u32
                                                    };
                                                    Some(e)
                                                }
                                                _ => _rt::invalid_enum_discriminant(),
                                            }
                                        };
                                        ErrorCode::HttpResponseHeaderSectionSize(e64)
                                    }
                                    27 => {
                                        let e64 = {
                                            let l36 = i32::from(*ptr0.add(16).cast::<u8>());
                                            let l40 = i32::from(*ptr0.add(28).cast::<u8>());
                                            FieldSizePayload {
                                                field_name: match l36 {
                                                    0 => None,
                                                    1 => {
                                                        let e = {
                                                            let l37 =
                                                                *ptr0.add(20).cast::<*mut u8>();
                                                            let l38 = *ptr0.add(24).cast::<usize>();
                                                            let len39 = l38;
                                                            let bytes39 = _rt::Vec::from_raw_parts(
                                                                l37.cast(),
                                                                len39,
                                                                len39,
                                                            );
                                                            _rt::string_lift(bytes39)
                                                        };
                                                        Some(e)
                                                    }
                                                    _ => _rt::invalid_enum_discriminant(),
                                                },
                                                field_size: match l40 {
                                                    0 => None,
                                                    1 => {
                                                        let e = {
                                                            let l41 = *ptr0.add(32).cast::<i32>();
                                                            l41 as u32
                                                        };
                                                        Some(e)
                                                    }
                                                    _ => _rt::invalid_enum_discriminant(),
                                                },
                                            }
                                        };
                                        ErrorCode::HttpResponseHeaderSize(e64)
                                    }
                                    28 => {
                                        let e64 = {
                                            let l42 = i32::from(*ptr0.add(16).cast::<u8>());
                                            match l42 {
                                                0 => None,
                                                1 => {
                                                    let e = {
                                                        let l43 = *ptr0.add(24).cast::<i64>();
                                                        l43 as u64
                                                    };
                                                    Some(e)
                                                }
                                                _ => _rt::invalid_enum_discriminant(),
                                            }
                                        };
                                        ErrorCode::HttpResponseBodySize(e64)
                                    }
                                    29 => {
                                        let e64 = {
                                            let l44 = i32::from(*ptr0.add(16).cast::<u8>());
                                            match l44 {
                                                0 => None,
                                                1 => {
                                                    let e = {
                                                        let l45 = *ptr0.add(20).cast::<i32>();
                                                        l45 as u32
                                                    };
                                                    Some(e)
                                                }
                                                _ => _rt::invalid_enum_discriminant(),
                                            }
                                        };
                                        ErrorCode::HttpResponseTrailerSectionSize(e64)
                                    }
                                    30 => {
                                        let e64 = {
                                            let l46 = i32::from(*ptr0.add(16).cast::<u8>());
                                            let l50 = i32::from(*ptr0.add(28).cast::<u8>());
                                            FieldSizePayload {
                                                field_name: match l46 {
                                                    0 => None,
                                                    1 => {
                                                        let e = {
                                                            let l47 =
                                                                *ptr0.add(20).cast::<*mut u8>();
                                                            let l48 = *ptr0.add(24).cast::<usize>();
                                                            let len49 = l48;
                                                            let bytes49 = _rt::Vec::from_raw_parts(
                                                                l47.cast(),
                                                                len49,
                                                                len49,
                                                            );
                                                            _rt::string_lift(bytes49)
                                                        };
                                                        Some(e)
                                                    }
                                                    _ => _rt::invalid_enum_discriminant(),
                                                },
                                                field_size: match l50 {
                                                    0 => None,
                                                    1 => {
                                                        let e = {
                                                            let l51 = *ptr0.add(32).cast::<i32>();
                                                            l51 as u32
                                                        };
                                                        Some(e)
                                                    }
                                                    _ => _rt::invalid_enum_discriminant(),
                                                },
                                            }
                                        };
                                        ErrorCode::HttpResponseTrailerSize(e64)
                                    }
                                    31 => {
                                        let e64 = {
                                            let l52 = i32::from(*ptr0.add(16).cast::<u8>());
                                            match l52 {
                                                0 => None,
                                                1 => {
                                                    let e = {
                                                        let l53 = *ptr0.add(20).cast::<*mut u8>();
                                                        let l54 = *ptr0.add(24).cast::<usize>();
                                                        let len55 = l54;
                                                        let bytes55 = _rt::Vec::from_raw_parts(
                                                            l53.cast(),
                                                            len55,
                                                            len55,
                                                        );
                                                        _rt::string_lift(bytes55)
                                                    };
                                                    Some(e)
                                                }
                                                _ => _rt::invalid_enum_discriminant(),
                                            }
                                        };
                                        ErrorCode::HttpResponseTransferCoding(e64)
                                    }
                                    32 => {
                                        let e64 = {
                                            let l56 = i32::from(*ptr0.add(16).cast::<u8>());
                                            match l56 {
                                                0 => None,
                                                1 => {
                                                    let e = {
                                                        let l57 = *ptr0.add(20).cast::<*mut u8>();
                                                        let l58 = *ptr0.add(24).cast::<usize>();
                                                        let len59 = l58;
                                                        let bytes59 = _rt::Vec::from_raw_parts(
                                                            l57.cast(),
                                                            len59,
                                                            len59,
                                                        );
                                                        _rt::string_lift(bytes59)
                                                    };
                                                    Some(e)
                                                }
                                                _ => _rt::invalid_enum_discriminant(),
                                            }
                                        };
                                        ErrorCode::HttpResponseContentCoding(e64)
                                    }
                                    33 => ErrorCode::HttpResponseTimeout,
                                    34 => ErrorCode::HttpUpgradeFailed,
                                    35 => ErrorCode::HttpProtocolError,
                                    36 => ErrorCode::LoopDetected,
                                    37 => ErrorCode::ConfigurationError,
                                    n => {
                                        debug_assert_eq!(n, 38, "invalid enum discriminant");
                                        let e64 = {
                                            let l60 = i32::from(*ptr0.add(16).cast::<u8>());
                                            match l60 {
                                                0 => None,
                                                1 => {
                                                    let e = {
                                                        let l61 = *ptr0.add(20).cast::<*mut u8>();
                                                        let l62 = *ptr0.add(24).cast::<usize>();
                                                        let len63 = l62;
                                                        let bytes63 = _rt::Vec::from_raw_parts(
                                                            l61.cast(),
                                                            len63,
                                                            len63,
                                                        );
                                                        _rt::string_lift(bytes63)
                                                    };
                                                    Some(e)
                                                }
                                                _ => _rt::invalid_enum_discriminant(),
                                            }
                                        };
                                        ErrorCode::InternalError(e64)
                                    }
                                };
                                v64
                            };
                            Some(e)
                        }
                        _ => _rt::invalid_enum_discriminant(),
                    }
                }
            }
        }
        #[allow(dead_code, clippy::all)]
        pub mod outgoing_handler {
            #[used]
            #[doc(hidden)]
            static __FORCE_SECTION_REF: fn() =
                super::super::super::__link_custom_section_describing_imports;
            use super::super::super::_rt;
            pub type OutgoingRequest = super::super::super::wasi::http::types::OutgoingRequest;
            pub type RequestOptions = super::super::super::wasi::http::types::RequestOptions;
            pub type FutureIncomingResponse =
                super::super::super::wasi::http::types::FutureIncomingResponse;
            pub type ErrorCode = super::super::super::wasi::http::types::ErrorCode;
            #[allow(unused_unsafe, clippy::all)]
            pub fn handle(
                request: OutgoingRequest,
                options: Option<RequestOptions>,
            ) -> Result<FutureIncomingResponse, ErrorCode> {
                unsafe {
                    #[repr(align(8))]
                    struct RetArea([::core::mem::MaybeUninit<u8>; 40]);
                    let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 40]);
                    let (result0_0, result0_1) = match &options {
                        Some(e) => (1i32, (e).take_handle() as i32),
                        None => (0i32, 0i32),
                    };
                    let ptr1 = ret_area.0.as_mut_ptr().cast::<u8>();
                    #[cfg(target_arch = "wasm32")]
                    #[link(wasm_import_module = "wasi:http/outgoing-handler@0.2.0")]
                    extern "C" {
                        #[link_name = "handle"]
                        fn wit_import(_: i32, _: i32, _: i32, _: *mut u8);
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import(_: i32, _: i32, _: i32, _: *mut u8) {
                        unreachable!()
                    }
                    wit_import((&request).take_handle() as i32, result0_0, result0_1, ptr1);
                    let l2 = i32::from(*ptr1.add(0).cast::<u8>());
                    match l2 {
                        0 => {
                            let e = {
                                let l3 = *ptr1.add(8).cast::<i32>();
                                super::super::super::wasi::http::types::FutureIncomingResponse::from_handle(
                                    l3 as u32,
                                )
                            };
                            Ok(e)
                        }
                        1 => {
                            let e = {
                                let l4 = i32::from(*ptr1.add(8).cast::<u8>());
                                use super::super::super::wasi::http::types::ErrorCode as V66;
                                let v66 = match l4 {
                                    0 => V66::DnsTimeout,
                                    1 => {
                                        let e66 = {
                                            let l5 = i32::from(*ptr1.add(16).cast::<u8>());
                                            let l9 = i32::from(*ptr1.add(28).cast::<u8>());
                                            super::super::super::wasi::http::types::DnsErrorPayload {
                                                rcode: match l5 {
                                                    0 => None,
                                                    1 => {
                                                        let e = {
                                                            let l6 = *ptr1.add(20).cast::<*mut u8>();
                                                            let l7 = *ptr1.add(24).cast::<usize>();
                                                            let len8 = l7;
                                                            let bytes8 = _rt::Vec::from_raw_parts(
                                                                l6.cast(),
                                                                len8,
                                                                len8,
                                                            );
                                                            _rt::string_lift(bytes8)
                                                        };
                                                        Some(e)
                                                    }
                                                    _ => _rt::invalid_enum_discriminant(),
                                                },
                                                info_code: match l9 {
                                                    0 => None,
                                                    1 => {
                                                        let e = {
                                                            let l10 = i32::from(*ptr1.add(30).cast::<u16>());
                                                            l10 as u16
                                                        };
                                                        Some(e)
                                                    }
                                                    _ => _rt::invalid_enum_discriminant(),
                                                },
                                            }
                                        };
                                        V66::DnsError(e66)
                                    }
                                    2 => V66::DestinationNotFound,
                                    3 => V66::DestinationUnavailable,
                                    4 => V66::DestinationIpProhibited,
                                    5 => V66::DestinationIpUnroutable,
                                    6 => V66::ConnectionRefused,
                                    7 => V66::ConnectionTerminated,
                                    8 => V66::ConnectionTimeout,
                                    9 => V66::ConnectionReadTimeout,
                                    10 => V66::ConnectionWriteTimeout,
                                    11 => V66::ConnectionLimitReached,
                                    12 => V66::TlsProtocolError,
                                    13 => V66::TlsCertificateError,
                                    14 => {
                                        let e66 = {
                                            let l11 = i32::from(*ptr1.add(16).cast::<u8>());
                                            let l13 = i32::from(*ptr1.add(20).cast::<u8>());
                                            super::super::super::wasi::http::types::TlsAlertReceivedPayload {
                                                alert_id: match l11 {
                                                    0 => None,
                                                    1 => {
                                                        let e = {
                                                            let l12 = i32::from(*ptr1.add(17).cast::<u8>());
                                                            l12 as u8
                                                        };
                                                        Some(e)
                                                    }
                                                    _ => _rt::invalid_enum_discriminant(),
                                                },
                                                alert_message: match l13 {
                                                    0 => None,
                                                    1 => {
                                                        let e = {
                                                            let l14 = *ptr1.add(24).cast::<*mut u8>();
                                                            let l15 = *ptr1.add(28).cast::<usize>();
                                                            let len16 = l15;
                                                            let bytes16 = _rt::Vec::from_raw_parts(
                                                                l14.cast(),
                                                                len16,
                                                                len16,
                                                            );
                                                            _rt::string_lift(bytes16)
                                                        };
                                                        Some(e)
                                                    }
                                                    _ => _rt::invalid_enum_discriminant(),
                                                },
                                            }
                                        };
                                        V66::TlsAlertReceived(e66)
                                    }
                                    15 => V66::HttpRequestDenied,
                                    16 => V66::HttpRequestLengthRequired,
                                    17 => {
                                        let e66 = {
                                            let l17 = i32::from(*ptr1.add(16).cast::<u8>());
                                            match l17 {
                                                0 => None,
                                                1 => {
                                                    let e = {
                                                        let l18 = *ptr1.add(24).cast::<i64>();
                                                        l18 as u64
                                                    };
                                                    Some(e)
                                                }
                                                _ => _rt::invalid_enum_discriminant(),
                                            }
                                        };
                                        V66::HttpRequestBodySize(e66)
                                    }
                                    18 => V66::HttpRequestMethodInvalid,
                                    19 => V66::HttpRequestUriInvalid,
                                    20 => V66::HttpRequestUriTooLong,
                                    21 => {
                                        let e66 = {
                                            let l19 = i32::from(*ptr1.add(16).cast::<u8>());
                                            match l19 {
                                                0 => None,
                                                1 => {
                                                    let e = {
                                                        let l20 = *ptr1.add(20).cast::<i32>();
                                                        l20 as u32
                                                    };
                                                    Some(e)
                                                }
                                                _ => _rt::invalid_enum_discriminant(),
                                            }
                                        };
                                        V66::HttpRequestHeaderSectionSize(e66)
                                    }
                                    22 => {
                                        let e66 = {
                                            let l21 = i32::from(*ptr1.add(16).cast::<u8>());
                                            match l21 {
                                                0 => None,
                                                1 => {
                                                    let e = {
                                                        let l22 =
                                                            i32::from(*ptr1.add(20).cast::<u8>());
                                                        let l26 =
                                                            i32::from(*ptr1.add(32).cast::<u8>());
                                                        super::super::super::wasi::http::types::FieldSizePayload {
                                                            field_name: match l22 {
                                                                0 => None,
                                                                1 => {
                                                                    let e = {
                                                                        let l23 = *ptr1.add(24).cast::<*mut u8>();
                                                                        let l24 = *ptr1.add(28).cast::<usize>();
                                                                        let len25 = l24;
                                                                        let bytes25 = _rt::Vec::from_raw_parts(
                                                                            l23.cast(),
                                                                            len25,
                                                                            len25,
                                                                        );
                                                                        _rt::string_lift(bytes25)
                                                                    };
                                                                    Some(e)
                                                                }
                                                                _ => _rt::invalid_enum_discriminant(),
                                                            },
                                                            field_size: match l26 {
                                                                0 => None,
                                                                1 => {
                                                                    let e = {
                                                                        let l27 = *ptr1.add(36).cast::<i32>();
                                                                        l27 as u32
                                                                    };
                                                                    Some(e)
                                                                }
                                                                _ => _rt::invalid_enum_discriminant(),
                                                            },
                                                        }
                                                    };
                                                    Some(e)
                                                }
                                                _ => _rt::invalid_enum_discriminant(),
                                            }
                                        };
                                        V66::HttpRequestHeaderSize(e66)
                                    }
                                    23 => {
                                        let e66 = {
                                            let l28 = i32::from(*ptr1.add(16).cast::<u8>());
                                            match l28 {
                                                0 => None,
                                                1 => {
                                                    let e = {
                                                        let l29 = *ptr1.add(20).cast::<i32>();
                                                        l29 as u32
                                                    };
                                                    Some(e)
                                                }
                                                _ => _rt::invalid_enum_discriminant(),
                                            }
                                        };
                                        V66::HttpRequestTrailerSectionSize(e66)
                                    }
                                    24 => {
                                        let e66 = {
                                            let l30 = i32::from(*ptr1.add(16).cast::<u8>());
                                            let l34 = i32::from(*ptr1.add(28).cast::<u8>());
                                            super::super::super::wasi::http::types::FieldSizePayload {
                                                field_name: match l30 {
                                                    0 => None,
                                                    1 => {
                                                        let e = {
                                                            let l31 = *ptr1.add(20).cast::<*mut u8>();
                                                            let l32 = *ptr1.add(24).cast::<usize>();
                                                            let len33 = l32;
                                                            let bytes33 = _rt::Vec::from_raw_parts(
                                                                l31.cast(),
                                                                len33,
                                                                len33,
                                                            );
                                                            _rt::string_lift(bytes33)
                                                        };
                                                        Some(e)
                                                    }
                                                    _ => _rt::invalid_enum_discriminant(),
                                                },
                                                field_size: match l34 {
                                                    0 => None,
                                                    1 => {
                                                        let e = {
                                                            let l35 = *ptr1.add(32).cast::<i32>();
                                                            l35 as u32
                                                        };
                                                        Some(e)
                                                    }
                                                    _ => _rt::invalid_enum_discriminant(),
                                                },
                                            }
                                        };
                                        V66::HttpRequestTrailerSize(e66)
                                    }
                                    25 => V66::HttpResponseIncomplete,
                                    26 => {
                                        let e66 = {
                                            let l36 = i32::from(*ptr1.add(16).cast::<u8>());
                                            match l36 {
                                                0 => None,
                                                1 => {
                                                    let e = {
                                                        let l37 = *ptr1.add(20).cast::<i32>();
                                                        l37 as u32
                                                    };
                                                    Some(e)
                                                }
                                                _ => _rt::invalid_enum_discriminant(),
                                            }
                                        };
                                        V66::HttpResponseHeaderSectionSize(e66)
                                    }
                                    27 => {
                                        let e66 = {
                                            let l38 = i32::from(*ptr1.add(16).cast::<u8>());
                                            let l42 = i32::from(*ptr1.add(28).cast::<u8>());
                                            super::super::super::wasi::http::types::FieldSizePayload {
                                                field_name: match l38 {
                                                    0 => None,
                                                    1 => {
                                                        let e = {
                                                            let l39 = *ptr1.add(20).cast::<*mut u8>();
                                                            let l40 = *ptr1.add(24).cast::<usize>();
                                                            let len41 = l40;
                                                            let bytes41 = _rt::Vec::from_raw_parts(
                                                                l39.cast(),
                                                                len41,
                                                                len41,
                                                            );
                                                            _rt::string_lift(bytes41)
                                                        };
                                                        Some(e)
                                                    }
                                                    _ => _rt::invalid_enum_discriminant(),
                                                },
                                                field_size: match l42 {
                                                    0 => None,
                                                    1 => {
                                                        let e = {
                                                            let l43 = *ptr1.add(32).cast::<i32>();
                                                            l43 as u32
                                                        };
                                                        Some(e)
                                                    }
                                                    _ => _rt::invalid_enum_discriminant(),
                                                },
                                            }
                                        };
                                        V66::HttpResponseHeaderSize(e66)
                                    }
                                    28 => {
                                        let e66 = {
                                            let l44 = i32::from(*ptr1.add(16).cast::<u8>());
                                            match l44 {
                                                0 => None,
                                                1 => {
                                                    let e = {
                                                        let l45 = *ptr1.add(24).cast::<i64>();
                                                        l45 as u64
                                                    };
                                                    Some(e)
                                                }
                                                _ => _rt::invalid_enum_discriminant(),
                                            }
                                        };
                                        V66::HttpResponseBodySize(e66)
                                    }
                                    29 => {
                                        let e66 = {
                                            let l46 = i32::from(*ptr1.add(16).cast::<u8>());
                                            match l46 {
                                                0 => None,
                                                1 => {
                                                    let e = {
                                                        let l47 = *ptr1.add(20).cast::<i32>();
                                                        l47 as u32
                                                    };
                                                    Some(e)
                                                }
                                                _ => _rt::invalid_enum_discriminant(),
                                            }
                                        };
                                        V66::HttpResponseTrailerSectionSize(e66)
                                    }
                                    30 => {
                                        let e66 = {
                                            let l48 = i32::from(*ptr1.add(16).cast::<u8>());
                                            let l52 = i32::from(*ptr1.add(28).cast::<u8>());
                                            super::super::super::wasi::http::types::FieldSizePayload {
                                                field_name: match l48 {
                                                    0 => None,
                                                    1 => {
                                                        let e = {
                                                            let l49 = *ptr1.add(20).cast::<*mut u8>();
                                                            let l50 = *ptr1.add(24).cast::<usize>();
                                                            let len51 = l50;
                                                            let bytes51 = _rt::Vec::from_raw_parts(
                                                                l49.cast(),
                                                                len51,
                                                                len51,
                                                            );
                                                            _rt::string_lift(bytes51)
                                                        };
                                                        Some(e)
                                                    }
                                                    _ => _rt::invalid_enum_discriminant(),
                                                },
                                                field_size: match l52 {
                                                    0 => None,
                                                    1 => {
                                                        let e = {
                                                            let l53 = *ptr1.add(32).cast::<i32>();
                                                            l53 as u32
                                                        };
                                                        Some(e)
                                                    }
                                                    _ => _rt::invalid_enum_discriminant(),
                                                },
                                            }
                                        };
                                        V66::HttpResponseTrailerSize(e66)
                                    }
                                    31 => {
                                        let e66 = {
                                            let l54 = i32::from(*ptr1.add(16).cast::<u8>());
                                            match l54 {
                                                0 => None,
                                                1 => {
                                                    let e = {
                                                        let l55 = *ptr1.add(20).cast::<*mut u8>();
                                                        let l56 = *ptr1.add(24).cast::<usize>();
                                                        let len57 = l56;
                                                        let bytes57 = _rt::Vec::from_raw_parts(
                                                            l55.cast(),
                                                            len57,
                                                            len57,
                                                        );
                                                        _rt::string_lift(bytes57)
                                                    };
                                                    Some(e)
                                                }
                                                _ => _rt::invalid_enum_discriminant(),
                                            }
                                        };
                                        V66::HttpResponseTransferCoding(e66)
                                    }
                                    32 => {
                                        let e66 = {
                                            let l58 = i32::from(*ptr1.add(16).cast::<u8>());
                                            match l58 {
                                                0 => None,
                                                1 => {
                                                    let e = {
                                                        let l59 = *ptr1.add(20).cast::<*mut u8>();
                                                        let l60 = *ptr1.add(24).cast::<usize>();
                                                        let len61 = l60;
                                                        let bytes61 = _rt::Vec::from_raw_parts(
                                                            l59.cast(),
                                                            len61,
                                                            len61,
                                                        );
                                                        _rt::string_lift(bytes61)
                                                    };
                                                    Some(e)
                                                }
                                                _ => _rt::invalid_enum_discriminant(),
                                            }
                                        };
                                        V66::HttpResponseContentCoding(e66)
                                    }
                                    33 => V66::HttpResponseTimeout,
                                    34 => V66::HttpUpgradeFailed,
                                    35 => V66::HttpProtocolError,
                                    36 => V66::LoopDetected,
                                    37 => V66::ConfigurationError,
                                    n => {
                                        debug_assert_eq!(n, 38, "invalid enum discriminant");
                                        let e66 = {
                                            let l62 = i32::from(*ptr1.add(16).cast::<u8>());
                                            match l62 {
                                                0 => None,
                                                1 => {
                                                    let e = {
                                                        let l63 = *ptr1.add(20).cast::<*mut u8>();
                                                        let l64 = *ptr1.add(24).cast::<usize>();
                                                        let len65 = l64;
                                                        let bytes65 = _rt::Vec::from_raw_parts(
                                                            l63.cast(),
                                                            len65,
                                                            len65,
                                                        );
                                                        _rt::string_lift(bytes65)
                                                    };
                                                    Some(e)
                                                }
                                                _ => _rt::invalid_enum_discriminant(),
                                            }
                                        };
                                        V66::InternalError(e66)
                                    }
                                };
                                v66
                            };
                            Err(e)
                        }
                        _ => _rt::invalid_enum_discriminant(),
                    }
                }
            }
        }
    }
    #[allow(dead_code)]
    pub mod io {
        #[allow(dead_code, clippy::all)]
        pub mod poll {
            #[used]
            #[doc(hidden)]
            static __FORCE_SECTION_REF: fn() =
                super::super::super::__link_custom_section_describing_imports;
            use super::super::super::_rt;
            #[derive(Debug)]
            #[repr(transparent)]
            pub struct Pollable {
                handle: _rt::Resource<Pollable>,
            }
            impl Pollable {
                #[doc(hidden)]
                pub unsafe fn from_handle(handle: u32) -> Self {
                    Self { handle: _rt::Resource::from_handle(handle) }
                }
                #[doc(hidden)]
                pub fn take_handle(&self) -> u32 {
                    _rt::Resource::take_handle(&self.handle)
                }
                #[doc(hidden)]
                pub fn handle(&self) -> u32 {
                    _rt::Resource::handle(&self.handle)
                }
            }
            unsafe impl _rt::WasmResource for Pollable {
                #[inline]
                unsafe fn drop(_handle: u32) {
                    #[cfg(not(target_arch = "wasm32"))]
                    unreachable!();
                    #[cfg(target_arch = "wasm32")]
                    {
                        #[link(wasm_import_module = "wasi:io/poll@0.2.0")]
                        extern "C" {
                            #[link_name = "[resource-drop]pollable"]
                            fn drop(_: u32);
                        }
                        drop(_handle);
                    }
                }
            }
            impl Pollable {
                #[allow(unused_unsafe, clippy::all)]
                pub fn ready(&self) -> bool {
                    unsafe {
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:io/poll@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]pollable.ready"]
                            fn wit_import(_: i32) -> i32;
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32) -> i32 {
                            unreachable!()
                        }
                        let ret = wit_import((self).handle() as i32);
                        _rt::bool_lift(ret as u8)
                    }
                }
            }
            impl Pollable {
                #[allow(unused_unsafe, clippy::all)]
                pub fn block(&self) {
                    unsafe {
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:io/poll@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]pollable.block"]
                            fn wit_import(_: i32);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32);
                    }
                }
            }
            #[allow(unused_unsafe, clippy::all)]
            pub fn poll(in_: &[&Pollable]) -> _rt::Vec<u32> {
                unsafe {
                    #[repr(align(4))]
                    struct RetArea([::core::mem::MaybeUninit<u8>; 8]);
                    let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 8]);
                    let vec0 = in_;
                    let len0 = vec0.len();
                    let layout0 = _rt::alloc::Layout::from_size_align_unchecked(vec0.len() * 4, 4);
                    let result0 = if layout0.size() != 0 {
                        let ptr = _rt::alloc::alloc(layout0).cast::<u8>();
                        if ptr.is_null() {
                            _rt::alloc::handle_alloc_error(layout0);
                        }
                        ptr
                    } else {
                        ::core::ptr::null_mut()
                    };
                    for (i, e) in vec0.into_iter().enumerate() {
                        let base = result0.add(i * 4);
                        {
                            *base.add(0).cast::<i32>() = (e).handle() as i32;
                        }
                    }
                    let ptr1 = ret_area.0.as_mut_ptr().cast::<u8>();
                    #[cfg(target_arch = "wasm32")]
                    #[link(wasm_import_module = "wasi:io/poll@0.2.0")]
                    extern "C" {
                        #[link_name = "poll"]
                        fn wit_import(_: *mut u8, _: usize, _: *mut u8);
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import(_: *mut u8, _: usize, _: *mut u8) {
                        unreachable!()
                    }
                    wit_import(result0, len0, ptr1);
                    let l2 = *ptr1.add(0).cast::<*mut u8>();
                    let l3 = *ptr1.add(4).cast::<usize>();
                    let len4 = l3;
                    if layout0.size() != 0 {
                        _rt::alloc::dealloc(result0.cast(), layout0);
                    }
                    _rt::Vec::from_raw_parts(l2.cast(), len4, len4)
                }
            }
        }
        #[allow(dead_code, clippy::all)]
        pub mod error {
            #[used]
            #[doc(hidden)]
            static __FORCE_SECTION_REF: fn() =
                super::super::super::__link_custom_section_describing_imports;
            use super::super::super::_rt;
            #[derive(Debug)]
            #[repr(transparent)]
            pub struct Error {
                handle: _rt::Resource<Error>,
            }
            impl Error {
                #[doc(hidden)]
                pub unsafe fn from_handle(handle: u32) -> Self {
                    Self { handle: _rt::Resource::from_handle(handle) }
                }
                #[doc(hidden)]
                pub fn take_handle(&self) -> u32 {
                    _rt::Resource::take_handle(&self.handle)
                }
                #[doc(hidden)]
                pub fn handle(&self) -> u32 {
                    _rt::Resource::handle(&self.handle)
                }
            }
            unsafe impl _rt::WasmResource for Error {
                #[inline]
                unsafe fn drop(_handle: u32) {
                    #[cfg(not(target_arch = "wasm32"))]
                    unreachable!();
                    #[cfg(target_arch = "wasm32")]
                    {
                        #[link(wasm_import_module = "wasi:io/error@0.2.0")]
                        extern "C" {
                            #[link_name = "[resource-drop]error"]
                            fn drop(_: u32);
                        }
                        drop(_handle);
                    }
                }
            }
            impl Error {
                #[allow(unused_unsafe, clippy::all)]
                pub fn to_debug_string(&self) -> _rt::String {
                    unsafe {
                        #[repr(align(4))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 8]);
                        let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 8]);
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:io/error@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]error.to-debug-string"]
                            fn wit_import(_: i32, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, ptr0);
                        let l1 = *ptr0.add(0).cast::<*mut u8>();
                        let l2 = *ptr0.add(4).cast::<usize>();
                        let len3 = l2;
                        let bytes3 = _rt::Vec::from_raw_parts(l1.cast(), len3, len3);
                        _rt::string_lift(bytes3)
                    }
                }
            }
        }
        #[allow(dead_code, clippy::all)]
        pub mod streams {
            #[used]
            #[doc(hidden)]
            static __FORCE_SECTION_REF: fn() =
                super::super::super::__link_custom_section_describing_imports;
            use super::super::super::_rt;
            pub type Error = super::super::super::wasi::io::error::Error;
            pub type Pollable = super::super::super::wasi::io::poll::Pollable;
            pub enum StreamError {
                LastOperationFailed(Error),
                Closed,
            }
            impl ::core::fmt::Debug for StreamError {
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    match self {
                        StreamError::LastOperationFailed(e) => {
                            f.debug_tuple("StreamError::LastOperationFailed").field(e).finish()
                        }
                        StreamError::Closed => f.debug_tuple("StreamError::Closed").finish(),
                    }
                }
            }
            impl ::core::fmt::Display for StreamError {
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    write!(f, "{:?}", self)
                }
            }
            impl std::error::Error for StreamError {}
            #[derive(Debug)]
            #[repr(transparent)]
            pub struct InputStream {
                handle: _rt::Resource<InputStream>,
            }
            impl InputStream {
                #[doc(hidden)]
                pub unsafe fn from_handle(handle: u32) -> Self {
                    Self { handle: _rt::Resource::from_handle(handle) }
                }
                #[doc(hidden)]
                pub fn take_handle(&self) -> u32 {
                    _rt::Resource::take_handle(&self.handle)
                }
                #[doc(hidden)]
                pub fn handle(&self) -> u32 {
                    _rt::Resource::handle(&self.handle)
                }
            }
            unsafe impl _rt::WasmResource for InputStream {
                #[inline]
                unsafe fn drop(_handle: u32) {
                    #[cfg(not(target_arch = "wasm32"))]
                    unreachable!();
                    #[cfg(target_arch = "wasm32")]
                    {
                        #[link(wasm_import_module = "wasi:io/streams@0.2.0")]
                        extern "C" {
                            #[link_name = "[resource-drop]input-stream"]
                            fn drop(_: u32);
                        }
                        drop(_handle);
                    }
                }
            }
            #[derive(Debug)]
            #[repr(transparent)]
            pub struct OutputStream {
                handle: _rt::Resource<OutputStream>,
            }
            impl OutputStream {
                #[doc(hidden)]
                pub unsafe fn from_handle(handle: u32) -> Self {
                    Self { handle: _rt::Resource::from_handle(handle) }
                }
                #[doc(hidden)]
                pub fn take_handle(&self) -> u32 {
                    _rt::Resource::take_handle(&self.handle)
                }
                #[doc(hidden)]
                pub fn handle(&self) -> u32 {
                    _rt::Resource::handle(&self.handle)
                }
            }
            unsafe impl _rt::WasmResource for OutputStream {
                #[inline]
                unsafe fn drop(_handle: u32) {
                    #[cfg(not(target_arch = "wasm32"))]
                    unreachable!();
                    #[cfg(target_arch = "wasm32")]
                    {
                        #[link(wasm_import_module = "wasi:io/streams@0.2.0")]
                        extern "C" {
                            #[link_name = "[resource-drop]output-stream"]
                            fn drop(_: u32);
                        }
                        drop(_handle);
                    }
                }
            }
            impl InputStream {
                #[allow(unused_unsafe, clippy::all)]
                pub fn read(&self, len: u64) -> Result<_rt::Vec<u8>, StreamError> {
                    unsafe {
                        #[repr(align(4))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 12]);
                        let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 12]);
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:io/streams@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]input-stream.read"]
                            fn wit_import(_: i32, _: i64, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: i64, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, _rt::as_i64(&len), ptr0);
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        match l1 {
                            0 => {
                                let e = {
                                    let l2 = *ptr0.add(4).cast::<*mut u8>();
                                    let l3 = *ptr0.add(8).cast::<usize>();
                                    let len4 = l3;
                                    _rt::Vec::from_raw_parts(l2.cast(), len4, len4)
                                };
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l5 = i32::from(*ptr0.add(4).cast::<u8>());
                                    let v7 = match l5 {
                                        0 => {
                                            let e7 = {
                                                let l6 = *ptr0.add(8).cast::<i32>();
                                                super::super::super::wasi::io::error::Error::from_handle(
                                                    l6 as u32,
                                                )
                                            };
                                            StreamError::LastOperationFailed(e7)
                                        }
                                        n => {
                                            debug_assert_eq!(n, 1, "invalid enum discriminant");
                                            StreamError::Closed
                                        }
                                    };
                                    v7
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl InputStream {
                #[allow(unused_unsafe, clippy::all)]
                pub fn blocking_read(&self, len: u64) -> Result<_rt::Vec<u8>, StreamError> {
                    unsafe {
                        #[repr(align(4))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 12]);
                        let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 12]);
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:io/streams@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]input-stream.blocking-read"]
                            fn wit_import(_: i32, _: i64, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: i64, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, _rt::as_i64(&len), ptr0);
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        match l1 {
                            0 => {
                                let e = {
                                    let l2 = *ptr0.add(4).cast::<*mut u8>();
                                    let l3 = *ptr0.add(8).cast::<usize>();
                                    let len4 = l3;
                                    _rt::Vec::from_raw_parts(l2.cast(), len4, len4)
                                };
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l5 = i32::from(*ptr0.add(4).cast::<u8>());
                                    let v7 = match l5 {
                                        0 => {
                                            let e7 = {
                                                let l6 = *ptr0.add(8).cast::<i32>();
                                                super::super::super::wasi::io::error::Error::from_handle(
                                                    l6 as u32,
                                                )
                                            };
                                            StreamError::LastOperationFailed(e7)
                                        }
                                        n => {
                                            debug_assert_eq!(n, 1, "invalid enum discriminant");
                                            StreamError::Closed
                                        }
                                    };
                                    v7
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl InputStream {
                #[allow(unused_unsafe, clippy::all)]
                pub fn skip(&self, len: u64) -> Result<u64, StreamError> {
                    unsafe {
                        #[repr(align(8))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 16]);
                        let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 16]);
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:io/streams@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]input-stream.skip"]
                            fn wit_import(_: i32, _: i64, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: i64, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, _rt::as_i64(&len), ptr0);
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        match l1 {
                            0 => {
                                let e = {
                                    let l2 = *ptr0.add(8).cast::<i64>();
                                    l2 as u64
                                };
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l3 = i32::from(*ptr0.add(8).cast::<u8>());
                                    let v5 = match l3 {
                                        0 => {
                                            let e5 = {
                                                let l4 = *ptr0.add(12).cast::<i32>();
                                                super::super::super::wasi::io::error::Error::from_handle(
                                                    l4 as u32,
                                                )
                                            };
                                            StreamError::LastOperationFailed(e5)
                                        }
                                        n => {
                                            debug_assert_eq!(n, 1, "invalid enum discriminant");
                                            StreamError::Closed
                                        }
                                    };
                                    v5
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl InputStream {
                #[allow(unused_unsafe, clippy::all)]
                pub fn blocking_skip(&self, len: u64) -> Result<u64, StreamError> {
                    unsafe {
                        #[repr(align(8))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 16]);
                        let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 16]);
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:io/streams@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]input-stream.blocking-skip"]
                            fn wit_import(_: i32, _: i64, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: i64, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, _rt::as_i64(&len), ptr0);
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        match l1 {
                            0 => {
                                let e = {
                                    let l2 = *ptr0.add(8).cast::<i64>();
                                    l2 as u64
                                };
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l3 = i32::from(*ptr0.add(8).cast::<u8>());
                                    let v5 = match l3 {
                                        0 => {
                                            let e5 = {
                                                let l4 = *ptr0.add(12).cast::<i32>();
                                                super::super::super::wasi::io::error::Error::from_handle(
                                                    l4 as u32,
                                                )
                                            };
                                            StreamError::LastOperationFailed(e5)
                                        }
                                        n => {
                                            debug_assert_eq!(n, 1, "invalid enum discriminant");
                                            StreamError::Closed
                                        }
                                    };
                                    v5
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl InputStream {
                #[allow(unused_unsafe, clippy::all)]
                pub fn subscribe(&self) -> Pollable {
                    unsafe {
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:io/streams@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]input-stream.subscribe"]
                            fn wit_import(_: i32) -> i32;
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32) -> i32 {
                            unreachable!()
                        }
                        let ret = wit_import((self).handle() as i32);
                        super::super::super::wasi::io::poll::Pollable::from_handle(ret as u32)
                    }
                }
            }
            impl OutputStream {
                #[allow(unused_unsafe, clippy::all)]
                pub fn check_write(&self) -> Result<u64, StreamError> {
                    unsafe {
                        #[repr(align(8))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 16]);
                        let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 16]);
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:io/streams@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]output-stream.check-write"]
                            fn wit_import(_: i32, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, ptr0);
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        match l1 {
                            0 => {
                                let e = {
                                    let l2 = *ptr0.add(8).cast::<i64>();
                                    l2 as u64
                                };
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l3 = i32::from(*ptr0.add(8).cast::<u8>());
                                    let v5 = match l3 {
                                        0 => {
                                            let e5 = {
                                                let l4 = *ptr0.add(12).cast::<i32>();
                                                super::super::super::wasi::io::error::Error::from_handle(
                                                    l4 as u32,
                                                )
                                            };
                                            StreamError::LastOperationFailed(e5)
                                        }
                                        n => {
                                            debug_assert_eq!(n, 1, "invalid enum discriminant");
                                            StreamError::Closed
                                        }
                                    };
                                    v5
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl OutputStream {
                #[allow(unused_unsafe, clippy::all)]
                pub fn write(&self, contents: &[u8]) -> Result<(), StreamError> {
                    unsafe {
                        #[repr(align(4))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 12]);
                        let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 12]);
                        let vec0 = contents;
                        let ptr0 = vec0.as_ptr().cast::<u8>();
                        let len0 = vec0.len();
                        let ptr1 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:io/streams@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]output-stream.write"]
                            fn wit_import(_: i32, _: *mut u8, _: usize, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: *mut u8, _: usize, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, ptr0.cast_mut(), len0, ptr1);
                        let l2 = i32::from(*ptr1.add(0).cast::<u8>());
                        match l2 {
                            0 => {
                                let e = ();
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l3 = i32::from(*ptr1.add(4).cast::<u8>());
                                    let v5 = match l3 {
                                        0 => {
                                            let e5 = {
                                                let l4 = *ptr1.add(8).cast::<i32>();
                                                super::super::super::wasi::io::error::Error::from_handle(
                                                    l4 as u32,
                                                )
                                            };
                                            StreamError::LastOperationFailed(e5)
                                        }
                                        n => {
                                            debug_assert_eq!(n, 1, "invalid enum discriminant");
                                            StreamError::Closed
                                        }
                                    };
                                    v5
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl OutputStream {
                #[allow(unused_unsafe, clippy::all)]
                pub fn blocking_write_and_flush(&self, contents: &[u8]) -> Result<(), StreamError> {
                    unsafe {
                        #[repr(align(4))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 12]);
                        let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 12]);
                        let vec0 = contents;
                        let ptr0 = vec0.as_ptr().cast::<u8>();
                        let len0 = vec0.len();
                        let ptr1 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:io/streams@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]output-stream.blocking-write-and-flush"]
                            fn wit_import(_: i32, _: *mut u8, _: usize, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: *mut u8, _: usize, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, ptr0.cast_mut(), len0, ptr1);
                        let l2 = i32::from(*ptr1.add(0).cast::<u8>());
                        match l2 {
                            0 => {
                                let e = ();
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l3 = i32::from(*ptr1.add(4).cast::<u8>());
                                    let v5 = match l3 {
                                        0 => {
                                            let e5 = {
                                                let l4 = *ptr1.add(8).cast::<i32>();
                                                super::super::super::wasi::io::error::Error::from_handle(
                                                    l4 as u32,
                                                )
                                            };
                                            StreamError::LastOperationFailed(e5)
                                        }
                                        n => {
                                            debug_assert_eq!(n, 1, "invalid enum discriminant");
                                            StreamError::Closed
                                        }
                                    };
                                    v5
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl OutputStream {
                #[allow(unused_unsafe, clippy::all)]
                pub fn flush(&self) -> Result<(), StreamError> {
                    unsafe {
                        #[repr(align(4))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 12]);
                        let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 12]);
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:io/streams@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]output-stream.flush"]
                            fn wit_import(_: i32, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, ptr0);
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        match l1 {
                            0 => {
                                let e = ();
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l2 = i32::from(*ptr0.add(4).cast::<u8>());
                                    let v4 = match l2 {
                                        0 => {
                                            let e4 = {
                                                let l3 = *ptr0.add(8).cast::<i32>();
                                                super::super::super::wasi::io::error::Error::from_handle(
                                                    l3 as u32,
                                                )
                                            };
                                            StreamError::LastOperationFailed(e4)
                                        }
                                        n => {
                                            debug_assert_eq!(n, 1, "invalid enum discriminant");
                                            StreamError::Closed
                                        }
                                    };
                                    v4
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl OutputStream {
                #[allow(unused_unsafe, clippy::all)]
                pub fn blocking_flush(&self) -> Result<(), StreamError> {
                    unsafe {
                        #[repr(align(4))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 12]);
                        let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 12]);
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:io/streams@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]output-stream.blocking-flush"]
                            fn wit_import(_: i32, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, ptr0);
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        match l1 {
                            0 => {
                                let e = ();
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l2 = i32::from(*ptr0.add(4).cast::<u8>());
                                    let v4 = match l2 {
                                        0 => {
                                            let e4 = {
                                                let l3 = *ptr0.add(8).cast::<i32>();
                                                super::super::super::wasi::io::error::Error::from_handle(
                                                    l3 as u32,
                                                )
                                            };
                                            StreamError::LastOperationFailed(e4)
                                        }
                                        n => {
                                            debug_assert_eq!(n, 1, "invalid enum discriminant");
                                            StreamError::Closed
                                        }
                                    };
                                    v4
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl OutputStream {
                #[allow(unused_unsafe, clippy::all)]
                pub fn subscribe(&self) -> Pollable {
                    unsafe {
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:io/streams@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]output-stream.subscribe"]
                            fn wit_import(_: i32) -> i32;
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32) -> i32 {
                            unreachable!()
                        }
                        let ret = wit_import((self).handle() as i32);
                        super::super::super::wasi::io::poll::Pollable::from_handle(ret as u32)
                    }
                }
            }
            impl OutputStream {
                #[allow(unused_unsafe, clippy::all)]
                pub fn write_zeroes(&self, len: u64) -> Result<(), StreamError> {
                    unsafe {
                        #[repr(align(4))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 12]);
                        let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 12]);
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:io/streams@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]output-stream.write-zeroes"]
                            fn wit_import(_: i32, _: i64, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: i64, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, _rt::as_i64(&len), ptr0);
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        match l1 {
                            0 => {
                                let e = ();
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l2 = i32::from(*ptr0.add(4).cast::<u8>());
                                    let v4 = match l2 {
                                        0 => {
                                            let e4 = {
                                                let l3 = *ptr0.add(8).cast::<i32>();
                                                super::super::super::wasi::io::error::Error::from_handle(
                                                    l3 as u32,
                                                )
                                            };
                                            StreamError::LastOperationFailed(e4)
                                        }
                                        n => {
                                            debug_assert_eq!(n, 1, "invalid enum discriminant");
                                            StreamError::Closed
                                        }
                                    };
                                    v4
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl OutputStream {
                #[allow(unused_unsafe, clippy::all)]
                pub fn blocking_write_zeroes_and_flush(&self, len: u64) -> Result<(), StreamError> {
                    unsafe {
                        #[repr(align(4))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 12]);
                        let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 12]);
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:io/streams@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]output-stream.blocking-write-zeroes-and-flush"]
                            fn wit_import(_: i32, _: i64, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: i64, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, _rt::as_i64(&len), ptr0);
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        match l1 {
                            0 => {
                                let e = ();
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l2 = i32::from(*ptr0.add(4).cast::<u8>());
                                    let v4 = match l2 {
                                        0 => {
                                            let e4 = {
                                                let l3 = *ptr0.add(8).cast::<i32>();
                                                super::super::super::wasi::io::error::Error::from_handle(
                                                    l3 as u32,
                                                )
                                            };
                                            StreamError::LastOperationFailed(e4)
                                        }
                                        n => {
                                            debug_assert_eq!(n, 1, "invalid enum discriminant");
                                            StreamError::Closed
                                        }
                                    };
                                    v4
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl OutputStream {
                #[allow(unused_unsafe, clippy::all)]
                pub fn splice(&self, src: &InputStream, len: u64) -> Result<u64, StreamError> {
                    unsafe {
                        #[repr(align(8))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 16]);
                        let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 16]);
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:io/streams@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]output-stream.splice"]
                            fn wit_import(_: i32, _: i32, _: i64, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: i32, _: i64, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import(
                            (self).handle() as i32,
                            (src).handle() as i32,
                            _rt::as_i64(&len),
                            ptr0,
                        );
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        match l1 {
                            0 => {
                                let e = {
                                    let l2 = *ptr0.add(8).cast::<i64>();
                                    l2 as u64
                                };
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l3 = i32::from(*ptr0.add(8).cast::<u8>());
                                    let v5 = match l3 {
                                        0 => {
                                            let e5 = {
                                                let l4 = *ptr0.add(12).cast::<i32>();
                                                super::super::super::wasi::io::error::Error::from_handle(
                                                    l4 as u32,
                                                )
                                            };
                                            StreamError::LastOperationFailed(e5)
                                        }
                                        n => {
                                            debug_assert_eq!(n, 1, "invalid enum discriminant");
                                            StreamError::Closed
                                        }
                                    };
                                    v5
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl OutputStream {
                #[allow(unused_unsafe, clippy::all)]
                pub fn blocking_splice(
                    &self,
                    src: &InputStream,
                    len: u64,
                ) -> Result<u64, StreamError> {
                    unsafe {
                        #[repr(align(8))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 16]);
                        let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 16]);
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:io/streams@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]output-stream.blocking-splice"]
                            fn wit_import(_: i32, _: i32, _: i64, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: i32, _: i64, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import(
                            (self).handle() as i32,
                            (src).handle() as i32,
                            _rt::as_i64(&len),
                            ptr0,
                        );
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        match l1 {
                            0 => {
                                let e = {
                                    let l2 = *ptr0.add(8).cast::<i64>();
                                    l2 as u64
                                };
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l3 = i32::from(*ptr0.add(8).cast::<u8>());
                                    let v5 = match l3 {
                                        0 => {
                                            let e5 = {
                                                let l4 = *ptr0.add(12).cast::<i32>();
                                                super::super::super::wasi::io::error::Error::from_handle(
                                                    l4 as u32,
                                                )
                                            };
                                            StreamError::LastOperationFailed(e5)
                                        }
                                        n => {
                                            debug_assert_eq!(n, 1, "invalid enum discriminant");
                                            StreamError::Closed
                                        }
                                    };
                                    v5
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
        }
    }
    #[allow(dead_code)]
    pub mod random {
        #[allow(dead_code, clippy::all)]
        pub mod random {
            #[used]
            #[doc(hidden)]
            static __FORCE_SECTION_REF: fn() =
                super::super::super::__link_custom_section_describing_imports;
            use super::super::super::_rt;
            #[allow(unused_unsafe, clippy::all)]
            pub fn get_random_bytes(len: u64) -> _rt::Vec<u8> {
                unsafe {
                    #[repr(align(4))]
                    struct RetArea([::core::mem::MaybeUninit<u8>; 8]);
                    let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 8]);
                    let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                    #[cfg(target_arch = "wasm32")]
                    #[link(wasm_import_module = "wasi:random/random@0.2.0")]
                    extern "C" {
                        #[link_name = "get-random-bytes"]
                        fn wit_import(_: i64, _: *mut u8);
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import(_: i64, _: *mut u8) {
                        unreachable!()
                    }
                    wit_import(_rt::as_i64(&len), ptr0);
                    let l1 = *ptr0.add(0).cast::<*mut u8>();
                    let l2 = *ptr0.add(4).cast::<usize>();
                    let len3 = l2;
                    _rt::Vec::from_raw_parts(l1.cast(), len3, len3)
                }
            }
            #[allow(unused_unsafe, clippy::all)]
            pub fn get_random_u64() -> u64 {
                unsafe {
                    #[cfg(target_arch = "wasm32")]
                    #[link(wasm_import_module = "wasi:random/random@0.2.0")]
                    extern "C" {
                        #[link_name = "get-random-u64"]
                        fn wit_import() -> i64;
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import() -> i64 {
                        unreachable!()
                    }
                    let ret = wit_import();
                    ret as u64
                }
            }
        }
        #[allow(dead_code, clippy::all)]
        pub mod insecure {
            #[used]
            #[doc(hidden)]
            static __FORCE_SECTION_REF: fn() =
                super::super::super::__link_custom_section_describing_imports;
            use super::super::super::_rt;
            #[allow(unused_unsafe, clippy::all)]
            pub fn get_insecure_random_bytes(len: u64) -> _rt::Vec<u8> {
                unsafe {
                    #[repr(align(4))]
                    struct RetArea([::core::mem::MaybeUninit<u8>; 8]);
                    let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 8]);
                    let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                    #[cfg(target_arch = "wasm32")]
                    #[link(wasm_import_module = "wasi:random/insecure@0.2.0")]
                    extern "C" {
                        #[link_name = "get-insecure-random-bytes"]
                        fn wit_import(_: i64, _: *mut u8);
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import(_: i64, _: *mut u8) {
                        unreachable!()
                    }
                    wit_import(_rt::as_i64(&len), ptr0);
                    let l1 = *ptr0.add(0).cast::<*mut u8>();
                    let l2 = *ptr0.add(4).cast::<usize>();
                    let len3 = l2;
                    _rt::Vec::from_raw_parts(l1.cast(), len3, len3)
                }
            }
            #[allow(unused_unsafe, clippy::all)]
            pub fn get_insecure_random_u64() -> u64 {
                unsafe {
                    #[cfg(target_arch = "wasm32")]
                    #[link(wasm_import_module = "wasi:random/insecure@0.2.0")]
                    extern "C" {
                        #[link_name = "get-insecure-random-u64"]
                        fn wit_import() -> i64;
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import() -> i64 {
                        unreachable!()
                    }
                    let ret = wit_import();
                    ret as u64
                }
            }
        }
        #[allow(dead_code, clippy::all)]
        pub mod insecure_seed {
            #[used]
            #[doc(hidden)]
            static __FORCE_SECTION_REF: fn() =
                super::super::super::__link_custom_section_describing_imports;
            #[allow(unused_unsafe, clippy::all)]
            pub fn insecure_seed() -> (u64, u64) {
                unsafe {
                    #[repr(align(8))]
                    struct RetArea([::core::mem::MaybeUninit<u8>; 16]);
                    let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 16]);
                    let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                    #[cfg(target_arch = "wasm32")]
                    #[link(wasm_import_module = "wasi:random/insecure-seed@0.2.0")]
                    extern "C" {
                        #[link_name = "insecure-seed"]
                        fn wit_import(_: *mut u8);
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import(_: *mut u8) {
                        unreachable!()
                    }
                    wit_import(ptr0);
                    let l1 = *ptr0.add(0).cast::<i64>();
                    let l2 = *ptr0.add(8).cast::<i64>();
                    (l1 as u64, l2 as u64)
                }
            }
        }
    }
    #[allow(dead_code)]
    pub mod sockets {
        #[allow(dead_code, clippy::all)]
        pub mod network {
            #[used]
            #[doc(hidden)]
            static __FORCE_SECTION_REF: fn() =
                super::super::super::__link_custom_section_describing_imports;
            use super::super::super::_rt;
            #[derive(Debug)]
            #[repr(transparent)]
            pub struct Network {
                handle: _rt::Resource<Network>,
            }
            impl Network {
                #[doc(hidden)]
                pub unsafe fn from_handle(handle: u32) -> Self {
                    Self { handle: _rt::Resource::from_handle(handle) }
                }
                #[doc(hidden)]
                pub fn take_handle(&self) -> u32 {
                    _rt::Resource::take_handle(&self.handle)
                }
                #[doc(hidden)]
                pub fn handle(&self) -> u32 {
                    _rt::Resource::handle(&self.handle)
                }
            }
            unsafe impl _rt::WasmResource for Network {
                #[inline]
                unsafe fn drop(_handle: u32) {
                    #[cfg(not(target_arch = "wasm32"))]
                    unreachable!();
                    #[cfg(target_arch = "wasm32")]
                    {
                        #[link(wasm_import_module = "wasi:sockets/network@0.2.0")]
                        extern "C" {
                            #[link_name = "[resource-drop]network"]
                            fn drop(_: u32);
                        }
                        drop(_handle);
                    }
                }
            }
            #[repr(u8)]
            #[derive(Clone, Copy, Eq, Ord, PartialEq, PartialOrd)]
            pub enum ErrorCode {
                Unknown,
                AccessDenied,
                NotSupported,
                InvalidArgument,
                OutOfMemory,
                Timeout,
                ConcurrencyConflict,
                NotInProgress,
                WouldBlock,
                InvalidState,
                NewSocketLimit,
                AddressNotBindable,
                AddressInUse,
                RemoteUnreachable,
                ConnectionRefused,
                ConnectionReset,
                ConnectionAborted,
                DatagramTooLarge,
                NameUnresolvable,
                TemporaryResolverFailure,
                PermanentResolverFailure,
            }
            impl ErrorCode {
                pub fn name(&self) -> &'static str {
                    match self {
                        ErrorCode::Unknown => "unknown",
                        ErrorCode::AccessDenied => "access-denied",
                        ErrorCode::NotSupported => "not-supported",
                        ErrorCode::InvalidArgument => "invalid-argument",
                        ErrorCode::OutOfMemory => "out-of-memory",
                        ErrorCode::Timeout => "timeout",
                        ErrorCode::ConcurrencyConflict => "concurrency-conflict",
                        ErrorCode::NotInProgress => "not-in-progress",
                        ErrorCode::WouldBlock => "would-block",
                        ErrorCode::InvalidState => "invalid-state",
                        ErrorCode::NewSocketLimit => "new-socket-limit",
                        ErrorCode::AddressNotBindable => "address-not-bindable",
                        ErrorCode::AddressInUse => "address-in-use",
                        ErrorCode::RemoteUnreachable => "remote-unreachable",
                        ErrorCode::ConnectionRefused => "connection-refused",
                        ErrorCode::ConnectionReset => "connection-reset",
                        ErrorCode::ConnectionAborted => "connection-aborted",
                        ErrorCode::DatagramTooLarge => "datagram-too-large",
                        ErrorCode::NameUnresolvable => "name-unresolvable",
                        ErrorCode::TemporaryResolverFailure => "temporary-resolver-failure",
                        ErrorCode::PermanentResolverFailure => "permanent-resolver-failure",
                    }
                }
                pub fn message(&self) -> &'static str {
                    match self {
                        ErrorCode::Unknown => "",
                        ErrorCode::AccessDenied => "",
                        ErrorCode::NotSupported => "",
                        ErrorCode::InvalidArgument => "",
                        ErrorCode::OutOfMemory => "",
                        ErrorCode::Timeout => "",
                        ErrorCode::ConcurrencyConflict => "",
                        ErrorCode::NotInProgress => "",
                        ErrorCode::WouldBlock => "",
                        ErrorCode::InvalidState => "",
                        ErrorCode::NewSocketLimit => "",
                        ErrorCode::AddressNotBindable => "",
                        ErrorCode::AddressInUse => "",
                        ErrorCode::RemoteUnreachable => "",
                        ErrorCode::ConnectionRefused => "",
                        ErrorCode::ConnectionReset => "",
                        ErrorCode::ConnectionAborted => "",
                        ErrorCode::DatagramTooLarge => "",
                        ErrorCode::NameUnresolvable => "",
                        ErrorCode::TemporaryResolverFailure => "",
                        ErrorCode::PermanentResolverFailure => "",
                    }
                }
            }
            impl ::core::fmt::Debug for ErrorCode {
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    f.debug_struct("ErrorCode")
                        .field("code", &(*self as i32))
                        .field("name", &self.name())
                        .field("message", &self.message())
                        .finish()
                }
            }
            impl ::core::fmt::Display for ErrorCode {
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    write!(f, "{} (error {})", self.name(), *self as i32)
                }
            }
            impl std::error::Error for ErrorCode {}
            impl ErrorCode {
                #[doc(hidden)]
                pub unsafe fn _lift(val: u8) -> ErrorCode {
                    if !cfg!(debug_assertions) {
                        return ::core::mem::transmute(val);
                    }
                    match val {
                        0 => ErrorCode::Unknown,
                        1 => ErrorCode::AccessDenied,
                        2 => ErrorCode::NotSupported,
                        3 => ErrorCode::InvalidArgument,
                        4 => ErrorCode::OutOfMemory,
                        5 => ErrorCode::Timeout,
                        6 => ErrorCode::ConcurrencyConflict,
                        7 => ErrorCode::NotInProgress,
                        8 => ErrorCode::WouldBlock,
                        9 => ErrorCode::InvalidState,
                        10 => ErrorCode::NewSocketLimit,
                        11 => ErrorCode::AddressNotBindable,
                        12 => ErrorCode::AddressInUse,
                        13 => ErrorCode::RemoteUnreachable,
                        14 => ErrorCode::ConnectionRefused,
                        15 => ErrorCode::ConnectionReset,
                        16 => ErrorCode::ConnectionAborted,
                        17 => ErrorCode::DatagramTooLarge,
                        18 => ErrorCode::NameUnresolvable,
                        19 => ErrorCode::TemporaryResolverFailure,
                        20 => ErrorCode::PermanentResolverFailure,
                        _ => panic!("invalid enum discriminant"),
                    }
                }
            }
            #[repr(u8)]
            #[derive(Clone, Copy, Eq, Ord, PartialEq, PartialOrd)]
            pub enum IpAddressFamily {
                Ipv4,
                Ipv6,
            }
            impl ::core::fmt::Debug for IpAddressFamily {
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    match self {
                        IpAddressFamily::Ipv4 => f.debug_tuple("IpAddressFamily::Ipv4").finish(),
                        IpAddressFamily::Ipv6 => f.debug_tuple("IpAddressFamily::Ipv6").finish(),
                    }
                }
            }
            impl IpAddressFamily {
                #[doc(hidden)]
                pub unsafe fn _lift(val: u8) -> IpAddressFamily {
                    if !cfg!(debug_assertions) {
                        return ::core::mem::transmute(val);
                    }
                    match val {
                        0 => IpAddressFamily::Ipv4,
                        1 => IpAddressFamily::Ipv6,
                        _ => panic!("invalid enum discriminant"),
                    }
                }
            }
            pub type Ipv4Address = (u8, u8, u8, u8);
            pub type Ipv6Address = (u16, u16, u16, u16, u16, u16, u16, u16);
            #[derive(Clone, Copy)]
            pub enum IpAddress {
                Ipv4(Ipv4Address),
                Ipv6(Ipv6Address),
            }
            impl ::core::fmt::Debug for IpAddress {
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    match self {
                        IpAddress::Ipv4(e) => f.debug_tuple("IpAddress::Ipv4").field(e).finish(),
                        IpAddress::Ipv6(e) => f.debug_tuple("IpAddress::Ipv6").field(e).finish(),
                    }
                }
            }
            #[repr(C)]
            #[derive(Clone, Copy)]
            pub struct Ipv4SocketAddress {
                pub port: u16,
                pub address: Ipv4Address,
            }
            impl ::core::fmt::Debug for Ipv4SocketAddress {
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    f.debug_struct("Ipv4SocketAddress")
                        .field("port", &self.port)
                        .field("address", &self.address)
                        .finish()
                }
            }
            #[repr(C)]
            #[derive(Clone, Copy)]
            pub struct Ipv6SocketAddress {
                pub port: u16,
                pub flow_info: u32,
                pub address: Ipv6Address,
                pub scope_id: u32,
            }
            impl ::core::fmt::Debug for Ipv6SocketAddress {
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    f.debug_struct("Ipv6SocketAddress")
                        .field("port", &self.port)
                        .field("flow-info", &self.flow_info)
                        .field("address", &self.address)
                        .field("scope-id", &self.scope_id)
                        .finish()
                }
            }
            #[derive(Clone, Copy)]
            pub enum IpSocketAddress {
                Ipv4(Ipv4SocketAddress),
                Ipv6(Ipv6SocketAddress),
            }
            impl ::core::fmt::Debug for IpSocketAddress {
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    match self {
                        IpSocketAddress::Ipv4(e) => {
                            f.debug_tuple("IpSocketAddress::Ipv4").field(e).finish()
                        }
                        IpSocketAddress::Ipv6(e) => {
                            f.debug_tuple("IpSocketAddress::Ipv6").field(e).finish()
                        }
                    }
                }
            }
        }
        #[allow(dead_code, clippy::all)]
        pub mod instance_network {
            #[used]
            #[doc(hidden)]
            static __FORCE_SECTION_REF: fn() =
                super::super::super::__link_custom_section_describing_imports;
            pub type Network = super::super::super::wasi::sockets::network::Network;
            #[allow(unused_unsafe, clippy::all)]
            pub fn instance_network() -> Network {
                unsafe {
                    #[cfg(target_arch = "wasm32")]
                    #[link(wasm_import_module = "wasi:sockets/instance-network@0.2.0")]
                    extern "C" {
                        #[link_name = "instance-network"]
                        fn wit_import() -> i32;
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import() -> i32 {
                        unreachable!()
                    }
                    let ret = wit_import();
                    super::super::super::wasi::sockets::network::Network::from_handle(ret as u32)
                }
            }
        }
        #[allow(dead_code, clippy::all)]
        pub mod udp {
            #[used]
            #[doc(hidden)]
            static __FORCE_SECTION_REF: fn() =
                super::super::super::__link_custom_section_describing_imports;
            use super::super::super::_rt;
            pub type Pollable = super::super::super::wasi::io::poll::Pollable;
            pub type Network = super::super::super::wasi::sockets::network::Network;
            pub type ErrorCode = super::super::super::wasi::sockets::network::ErrorCode;
            pub type IpSocketAddress = super::super::super::wasi::sockets::network::IpSocketAddress;
            pub type IpAddressFamily = super::super::super::wasi::sockets::network::IpAddressFamily;
            #[derive(Clone)]
            pub struct IncomingDatagram {
                pub data: _rt::Vec<u8>,
                pub remote_address: IpSocketAddress,
            }
            impl ::core::fmt::Debug for IncomingDatagram {
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    f.debug_struct("IncomingDatagram")
                        .field("data", &self.data)
                        .field("remote-address", &self.remote_address)
                        .finish()
                }
            }
            #[derive(Clone)]
            pub struct OutgoingDatagram {
                pub data: _rt::Vec<u8>,
                pub remote_address: Option<IpSocketAddress>,
            }
            impl ::core::fmt::Debug for OutgoingDatagram {
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    f.debug_struct("OutgoingDatagram")
                        .field("data", &self.data)
                        .field("remote-address", &self.remote_address)
                        .finish()
                }
            }
            #[derive(Debug)]
            #[repr(transparent)]
            pub struct UdpSocket {
                handle: _rt::Resource<UdpSocket>,
            }
            impl UdpSocket {
                #[doc(hidden)]
                pub unsafe fn from_handle(handle: u32) -> Self {
                    Self { handle: _rt::Resource::from_handle(handle) }
                }
                #[doc(hidden)]
                pub fn take_handle(&self) -> u32 {
                    _rt::Resource::take_handle(&self.handle)
                }
                #[doc(hidden)]
                pub fn handle(&self) -> u32 {
                    _rt::Resource::handle(&self.handle)
                }
            }
            unsafe impl _rt::WasmResource for UdpSocket {
                #[inline]
                unsafe fn drop(_handle: u32) {
                    #[cfg(not(target_arch = "wasm32"))]
                    unreachable!();
                    #[cfg(target_arch = "wasm32")]
                    {
                        #[link(wasm_import_module = "wasi:sockets/udp@0.2.0")]
                        extern "C" {
                            #[link_name = "[resource-drop]udp-socket"]
                            fn drop(_: u32);
                        }
                        drop(_handle);
                    }
                }
            }
            #[derive(Debug)]
            #[repr(transparent)]
            pub struct IncomingDatagramStream {
                handle: _rt::Resource<IncomingDatagramStream>,
            }
            impl IncomingDatagramStream {
                #[doc(hidden)]
                pub unsafe fn from_handle(handle: u32) -> Self {
                    Self { handle: _rt::Resource::from_handle(handle) }
                }
                #[doc(hidden)]
                pub fn take_handle(&self) -> u32 {
                    _rt::Resource::take_handle(&self.handle)
                }
                #[doc(hidden)]
                pub fn handle(&self) -> u32 {
                    _rt::Resource::handle(&self.handle)
                }
            }
            unsafe impl _rt::WasmResource for IncomingDatagramStream {
                #[inline]
                unsafe fn drop(_handle: u32) {
                    #[cfg(not(target_arch = "wasm32"))]
                    unreachable!();
                    #[cfg(target_arch = "wasm32")]
                    {
                        #[link(wasm_import_module = "wasi:sockets/udp@0.2.0")]
                        extern "C" {
                            #[link_name = "[resource-drop]incoming-datagram-stream"]
                            fn drop(_: u32);
                        }
                        drop(_handle);
                    }
                }
            }
            #[derive(Debug)]
            #[repr(transparent)]
            pub struct OutgoingDatagramStream {
                handle: _rt::Resource<OutgoingDatagramStream>,
            }
            impl OutgoingDatagramStream {
                #[doc(hidden)]
                pub unsafe fn from_handle(handle: u32) -> Self {
                    Self { handle: _rt::Resource::from_handle(handle) }
                }
                #[doc(hidden)]
                pub fn take_handle(&self) -> u32 {
                    _rt::Resource::take_handle(&self.handle)
                }
                #[doc(hidden)]
                pub fn handle(&self) -> u32 {
                    _rt::Resource::handle(&self.handle)
                }
            }
            unsafe impl _rt::WasmResource for OutgoingDatagramStream {
                #[inline]
                unsafe fn drop(_handle: u32) {
                    #[cfg(not(target_arch = "wasm32"))]
                    unreachable!();
                    #[cfg(target_arch = "wasm32")]
                    {
                        #[link(wasm_import_module = "wasi:sockets/udp@0.2.0")]
                        extern "C" {
                            #[link_name = "[resource-drop]outgoing-datagram-stream"]
                            fn drop(_: u32);
                        }
                        drop(_handle);
                    }
                }
            }
            impl UdpSocket {
                #[allow(unused_unsafe, clippy::all)]
                pub fn start_bind(
                    &self,
                    network: &Network,
                    local_address: IpSocketAddress,
                ) -> Result<(), ErrorCode> {
                    unsafe {
                        #[repr(align(1))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 2]);
                        let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 2]);
                        use super::super::super::wasi::sockets::network::IpSocketAddress as V4;
                        let (
                            result5_0,
                            result5_1,
                            result5_2,
                            result5_3,
                            result5_4,
                            result5_5,
                            result5_6,
                            result5_7,
                            result5_8,
                            result5_9,
                            result5_10,
                            result5_11,
                        ) = match local_address {
                            V4::Ipv4(e) => {
                                let super::super::super::wasi::sockets::network::Ipv4SocketAddress {
                                    port: port0,
                                    address: address0,
                                } = e;
                                let (t1_0, t1_1, t1_2, t1_3) = address0;
                                (
                                    0i32,
                                    _rt::as_i32(port0),
                                    _rt::as_i32(t1_0),
                                    _rt::as_i32(t1_1),
                                    _rt::as_i32(t1_2),
                                    _rt::as_i32(t1_3),
                                    0i32,
                                    0i32,
                                    0i32,
                                    0i32,
                                    0i32,
                                    0i32,
                                )
                            }
                            V4::Ipv6(e) => {
                                let super::super::super::wasi::sockets::network::Ipv6SocketAddress {
                                    port: port2,
                                    flow_info: flow_info2,
                                    address: address2,
                                    scope_id: scope_id2,
                                } = e;
                                let (t3_0, t3_1, t3_2, t3_3, t3_4, t3_5, t3_6, t3_7) = address2;
                                (
                                    1i32,
                                    _rt::as_i32(port2),
                                    _rt::as_i32(flow_info2),
                                    _rt::as_i32(t3_0),
                                    _rt::as_i32(t3_1),
                                    _rt::as_i32(t3_2),
                                    _rt::as_i32(t3_3),
                                    _rt::as_i32(t3_4),
                                    _rt::as_i32(t3_5),
                                    _rt::as_i32(t3_6),
                                    _rt::as_i32(t3_7),
                                    _rt::as_i32(scope_id2),
                                )
                            }
                        };
                        let ptr6 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:sockets/udp@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]udp-socket.start-bind"]
                            fn wit_import(
                                _: i32,
                                _: i32,
                                _: i32,
                                _: i32,
                                _: i32,
                                _: i32,
                                _: i32,
                                _: i32,
                                _: i32,
                                _: i32,
                                _: i32,
                                _: i32,
                                _: i32,
                                _: i32,
                                _: *mut u8,
                            );
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(
                            _: i32,
                            _: i32,
                            _: i32,
                            _: i32,
                            _: i32,
                            _: i32,
                            _: i32,
                            _: i32,
                            _: i32,
                            _: i32,
                            _: i32,
                            _: i32,
                            _: i32,
                            _: i32,
                            _: *mut u8,
                        ) {
                            unreachable!()
                        }
                        wit_import(
                            (self).handle() as i32,
                            (network).handle() as i32,
                            result5_0,
                            result5_1,
                            result5_2,
                            result5_3,
                            result5_4,
                            result5_5,
                            result5_6,
                            result5_7,
                            result5_8,
                            result5_9,
                            result5_10,
                            result5_11,
                            ptr6,
                        );
                        let l7 = i32::from(*ptr6.add(0).cast::<u8>());
                        match l7 {
                            0 => {
                                let e = ();
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l8 = i32::from(*ptr6.add(1).cast::<u8>());
                                    super::super::super::wasi::sockets::network::ErrorCode::_lift(
                                        l8 as u8,
                                    )
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl UdpSocket {
                #[allow(unused_unsafe, clippy::all)]
                pub fn finish_bind(&self) -> Result<(), ErrorCode> {
                    unsafe {
                        #[repr(align(1))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 2]);
                        let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 2]);
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:sockets/udp@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]udp-socket.finish-bind"]
                            fn wit_import(_: i32, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, ptr0);
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        match l1 {
                            0 => {
                                let e = ();
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l2 = i32::from(*ptr0.add(1).cast::<u8>());
                                    super::super::super::wasi::sockets::network::ErrorCode::_lift(
                                        l2 as u8,
                                    )
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl UdpSocket {
                #[allow(unused_unsafe, clippy::all)]
                pub fn stream(
                    &self,
                    remote_address: Option<IpSocketAddress>,
                ) -> Result<(IncomingDatagramStream, OutgoingDatagramStream), ErrorCode>
                {
                    unsafe {
                        #[repr(align(4))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 12]);
                        let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 12]);
                        let (
                            result6_0,
                            result6_1,
                            result6_2,
                            result6_3,
                            result6_4,
                            result6_5,
                            result6_6,
                            result6_7,
                            result6_8,
                            result6_9,
                            result6_10,
                            result6_11,
                            result6_12,
                        ) = match remote_address {
                            Some(e) => {
                                use super::super::super::wasi::sockets::network::IpSocketAddress as V4;
                                let (
                                    result5_0,
                                    result5_1,
                                    result5_2,
                                    result5_3,
                                    result5_4,
                                    result5_5,
                                    result5_6,
                                    result5_7,
                                    result5_8,
                                    result5_9,
                                    result5_10,
                                    result5_11,
                                ) = match e {
                                    V4::Ipv4(e) => {
                                        let super::super::super::wasi::sockets::network::Ipv4SocketAddress {
                                            port: port0,
                                            address: address0,
                                        } = e;
                                        let (t1_0, t1_1, t1_2, t1_3) = address0;
                                        (
                                            0i32,
                                            _rt::as_i32(port0),
                                            _rt::as_i32(t1_0),
                                            _rt::as_i32(t1_1),
                                            _rt::as_i32(t1_2),
                                            _rt::as_i32(t1_3),
                                            0i32,
                                            0i32,
                                            0i32,
                                            0i32,
                                            0i32,
                                            0i32,
                                        )
                                    }
                                    V4::Ipv6(e) => {
                                        let super::super::super::wasi::sockets::network::Ipv6SocketAddress {
                                            port: port2,
                                            flow_info: flow_info2,
                                            address: address2,
                                            scope_id: scope_id2,
                                        } = e;
                                        let (t3_0, t3_1, t3_2, t3_3, t3_4, t3_5, t3_6, t3_7) =
                                            address2;
                                        (
                                            1i32,
                                            _rt::as_i32(port2),
                                            _rt::as_i32(flow_info2),
                                            _rt::as_i32(t3_0),
                                            _rt::as_i32(t3_1),
                                            _rt::as_i32(t3_2),
                                            _rt::as_i32(t3_3),
                                            _rt::as_i32(t3_4),
                                            _rt::as_i32(t3_5),
                                            _rt::as_i32(t3_6),
                                            _rt::as_i32(t3_7),
                                            _rt::as_i32(scope_id2),
                                        )
                                    }
                                };
                                (
                                    1i32, result5_0, result5_1, result5_2, result5_3, result5_4,
                                    result5_5, result5_6, result5_7, result5_8, result5_9,
                                    result5_10, result5_11,
                                )
                            }
                            None => (
                                0i32, 0i32, 0i32, 0i32, 0i32, 0i32, 0i32, 0i32, 0i32, 0i32, 0i32,
                                0i32, 0i32,
                            ),
                        };
                        let ptr7 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:sockets/udp@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]udp-socket.stream"]
                            fn wit_import(
                                _: i32,
                                _: i32,
                                _: i32,
                                _: i32,
                                _: i32,
                                _: i32,
                                _: i32,
                                _: i32,
                                _: i32,
                                _: i32,
                                _: i32,
                                _: i32,
                                _: i32,
                                _: i32,
                                _: *mut u8,
                            );
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(
                            _: i32,
                            _: i32,
                            _: i32,
                            _: i32,
                            _: i32,
                            _: i32,
                            _: i32,
                            _: i32,
                            _: i32,
                            _: i32,
                            _: i32,
                            _: i32,
                            _: i32,
                            _: i32,
                            _: *mut u8,
                        ) {
                            unreachable!()
                        }
                        wit_import(
                            (self).handle() as i32,
                            result6_0,
                            result6_1,
                            result6_2,
                            result6_3,
                            result6_4,
                            result6_5,
                            result6_6,
                            result6_7,
                            result6_8,
                            result6_9,
                            result6_10,
                            result6_11,
                            result6_12,
                            ptr7,
                        );
                        let l8 = i32::from(*ptr7.add(0).cast::<u8>());
                        match l8 {
                            0 => {
                                let e = {
                                    let l9 = *ptr7.add(4).cast::<i32>();
                                    let l10 = *ptr7.add(8).cast::<i32>();
                                    (
                                        IncomingDatagramStream::from_handle(l9 as u32),
                                        OutgoingDatagramStream::from_handle(l10 as u32),
                                    )
                                };
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l11 = i32::from(*ptr7.add(4).cast::<u8>());
                                    super::super::super::wasi::sockets::network::ErrorCode::_lift(
                                        l11 as u8,
                                    )
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl UdpSocket {
                #[allow(unused_unsafe, clippy::all)]
                pub fn local_address(&self) -> Result<IpSocketAddress, ErrorCode> {
                    unsafe {
                        #[repr(align(4))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 36]);
                        let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 36]);
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:sockets/udp@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]udp-socket.local-address"]
                            fn wit_import(_: i32, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, ptr0);
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        match l1 {
                            0 => {
                                let e = {
                                    let l2 = i32::from(*ptr0.add(4).cast::<u8>());
                                    use super::super::super::wasi::sockets::network::IpSocketAddress as V19;
                                    let v19 = match l2 {
                                        0 => {
                                            let e19 = {
                                                let l3 = i32::from(*ptr0.add(8).cast::<u16>());
                                                let l4 = i32::from(*ptr0.add(10).cast::<u8>());
                                                let l5 = i32::from(*ptr0.add(11).cast::<u8>());
                                                let l6 = i32::from(*ptr0.add(12).cast::<u8>());
                                                let l7 = i32::from(*ptr0.add(13).cast::<u8>());
                                                super::super::super::wasi::sockets::network::Ipv4SocketAddress {
                                                    port: l3 as u16,
                                                    address: (l4 as u8, l5 as u8, l6 as u8, l7 as u8),
                                                }
                                            };
                                            V19::Ipv4(e19)
                                        }
                                        n => {
                                            debug_assert_eq!(n, 1, "invalid enum discriminant");
                                            let e19 = {
                                                let l8 = i32::from(*ptr0.add(8).cast::<u16>());
                                                let l9 = *ptr0.add(12).cast::<i32>();
                                                let l10 = i32::from(*ptr0.add(16).cast::<u16>());
                                                let l11 = i32::from(*ptr0.add(18).cast::<u16>());
                                                let l12 = i32::from(*ptr0.add(20).cast::<u16>());
                                                let l13 = i32::from(*ptr0.add(22).cast::<u16>());
                                                let l14 = i32::from(*ptr0.add(24).cast::<u16>());
                                                let l15 = i32::from(*ptr0.add(26).cast::<u16>());
                                                let l16 = i32::from(*ptr0.add(28).cast::<u16>());
                                                let l17 = i32::from(*ptr0.add(30).cast::<u16>());
                                                let l18 = *ptr0.add(32).cast::<i32>();
                                                super::super::super::wasi::sockets::network::Ipv6SocketAddress {
                                                    port: l8 as u16,
                                                    flow_info: l9 as u32,
                                                    address: (
                                                        l10 as u16,
                                                        l11 as u16,
                                                        l12 as u16,
                                                        l13 as u16,
                                                        l14 as u16,
                                                        l15 as u16,
                                                        l16 as u16,
                                                        l17 as u16,
                                                    ),
                                                    scope_id: l18 as u32,
                                                }
                                            };
                                            V19::Ipv6(e19)
                                        }
                                    };
                                    v19
                                };
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l20 = i32::from(*ptr0.add(4).cast::<u8>());
                                    super::super::super::wasi::sockets::network::ErrorCode::_lift(
                                        l20 as u8,
                                    )
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl UdpSocket {
                #[allow(unused_unsafe, clippy::all)]
                pub fn remote_address(&self) -> Result<IpSocketAddress, ErrorCode> {
                    unsafe {
                        #[repr(align(4))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 36]);
                        let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 36]);
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:sockets/udp@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]udp-socket.remote-address"]
                            fn wit_import(_: i32, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, ptr0);
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        match l1 {
                            0 => {
                                let e = {
                                    let l2 = i32::from(*ptr0.add(4).cast::<u8>());
                                    use super::super::super::wasi::sockets::network::IpSocketAddress as V19;
                                    let v19 = match l2 {
                                        0 => {
                                            let e19 = {
                                                let l3 = i32::from(*ptr0.add(8).cast::<u16>());
                                                let l4 = i32::from(*ptr0.add(10).cast::<u8>());
                                                let l5 = i32::from(*ptr0.add(11).cast::<u8>());
                                                let l6 = i32::from(*ptr0.add(12).cast::<u8>());
                                                let l7 = i32::from(*ptr0.add(13).cast::<u8>());
                                                super::super::super::wasi::sockets::network::Ipv4SocketAddress {
                                                    port: l3 as u16,
                                                    address: (l4 as u8, l5 as u8, l6 as u8, l7 as u8),
                                                }
                                            };
                                            V19::Ipv4(e19)
                                        }
                                        n => {
                                            debug_assert_eq!(n, 1, "invalid enum discriminant");
                                            let e19 = {
                                                let l8 = i32::from(*ptr0.add(8).cast::<u16>());
                                                let l9 = *ptr0.add(12).cast::<i32>();
                                                let l10 = i32::from(*ptr0.add(16).cast::<u16>());
                                                let l11 = i32::from(*ptr0.add(18).cast::<u16>());
                                                let l12 = i32::from(*ptr0.add(20).cast::<u16>());
                                                let l13 = i32::from(*ptr0.add(22).cast::<u16>());
                                                let l14 = i32::from(*ptr0.add(24).cast::<u16>());
                                                let l15 = i32::from(*ptr0.add(26).cast::<u16>());
                                                let l16 = i32::from(*ptr0.add(28).cast::<u16>());
                                                let l17 = i32::from(*ptr0.add(30).cast::<u16>());
                                                let l18 = *ptr0.add(32).cast::<i32>();
                                                super::super::super::wasi::sockets::network::Ipv6SocketAddress {
                                                    port: l8 as u16,
                                                    flow_info: l9 as u32,
                                                    address: (
                                                        l10 as u16,
                                                        l11 as u16,
                                                        l12 as u16,
                                                        l13 as u16,
                                                        l14 as u16,
                                                        l15 as u16,
                                                        l16 as u16,
                                                        l17 as u16,
                                                    ),
                                                    scope_id: l18 as u32,
                                                }
                                            };
                                            V19::Ipv6(e19)
                                        }
                                    };
                                    v19
                                };
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l20 = i32::from(*ptr0.add(4).cast::<u8>());
                                    super::super::super::wasi::sockets::network::ErrorCode::_lift(
                                        l20 as u8,
                                    )
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl UdpSocket {
                #[allow(unused_unsafe, clippy::all)]
                pub fn address_family(&self) -> IpAddressFamily {
                    unsafe {
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:sockets/udp@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]udp-socket.address-family"]
                            fn wit_import(_: i32) -> i32;
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32) -> i32 {
                            unreachable!()
                        }
                        let ret = wit_import((self).handle() as i32);
                        super::super::super::wasi::sockets::network::IpAddressFamily::_lift(
                            ret as u8,
                        )
                    }
                }
            }
            impl UdpSocket {
                #[allow(unused_unsafe, clippy::all)]
                pub fn unicast_hop_limit(&self) -> Result<u8, ErrorCode> {
                    unsafe {
                        #[repr(align(1))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 2]);
                        let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 2]);
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:sockets/udp@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]udp-socket.unicast-hop-limit"]
                            fn wit_import(_: i32, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, ptr0);
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        match l1 {
                            0 => {
                                let e = {
                                    let l2 = i32::from(*ptr0.add(1).cast::<u8>());
                                    l2 as u8
                                };
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l3 = i32::from(*ptr0.add(1).cast::<u8>());
                                    super::super::super::wasi::sockets::network::ErrorCode::_lift(
                                        l3 as u8,
                                    )
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl UdpSocket {
                #[allow(unused_unsafe, clippy::all)]
                pub fn set_unicast_hop_limit(&self, value: u8) -> Result<(), ErrorCode> {
                    unsafe {
                        #[repr(align(1))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 2]);
                        let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 2]);
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:sockets/udp@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]udp-socket.set-unicast-hop-limit"]
                            fn wit_import(_: i32, _: i32, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: i32, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, _rt::as_i32(&value), ptr0);
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        match l1 {
                            0 => {
                                let e = ();
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l2 = i32::from(*ptr0.add(1).cast::<u8>());
                                    super::super::super::wasi::sockets::network::ErrorCode::_lift(
                                        l2 as u8,
                                    )
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl UdpSocket {
                #[allow(unused_unsafe, clippy::all)]
                pub fn receive_buffer_size(&self) -> Result<u64, ErrorCode> {
                    unsafe {
                        #[repr(align(8))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 16]);
                        let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 16]);
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:sockets/udp@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]udp-socket.receive-buffer-size"]
                            fn wit_import(_: i32, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, ptr0);
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        match l1 {
                            0 => {
                                let e = {
                                    let l2 = *ptr0.add(8).cast::<i64>();
                                    l2 as u64
                                };
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l3 = i32::from(*ptr0.add(8).cast::<u8>());
                                    super::super::super::wasi::sockets::network::ErrorCode::_lift(
                                        l3 as u8,
                                    )
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl UdpSocket {
                #[allow(unused_unsafe, clippy::all)]
                pub fn set_receive_buffer_size(&self, value: u64) -> Result<(), ErrorCode> {
                    unsafe {
                        #[repr(align(1))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 2]);
                        let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 2]);
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:sockets/udp@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]udp-socket.set-receive-buffer-size"]
                            fn wit_import(_: i32, _: i64, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: i64, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, _rt::as_i64(&value), ptr0);
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        match l1 {
                            0 => {
                                let e = ();
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l2 = i32::from(*ptr0.add(1).cast::<u8>());
                                    super::super::super::wasi::sockets::network::ErrorCode::_lift(
                                        l2 as u8,
                                    )
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl UdpSocket {
                #[allow(unused_unsafe, clippy::all)]
                pub fn send_buffer_size(&self) -> Result<u64, ErrorCode> {
                    unsafe {
                        #[repr(align(8))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 16]);
                        let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 16]);
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:sockets/udp@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]udp-socket.send-buffer-size"]
                            fn wit_import(_: i32, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, ptr0);
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        match l1 {
                            0 => {
                                let e = {
                                    let l2 = *ptr0.add(8).cast::<i64>();
                                    l2 as u64
                                };
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l3 = i32::from(*ptr0.add(8).cast::<u8>());
                                    super::super::super::wasi::sockets::network::ErrorCode::_lift(
                                        l3 as u8,
                                    )
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl UdpSocket {
                #[allow(unused_unsafe, clippy::all)]
                pub fn set_send_buffer_size(&self, value: u64) -> Result<(), ErrorCode> {
                    unsafe {
                        #[repr(align(1))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 2]);
                        let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 2]);
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:sockets/udp@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]udp-socket.set-send-buffer-size"]
                            fn wit_import(_: i32, _: i64, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: i64, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, _rt::as_i64(&value), ptr0);
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        match l1 {
                            0 => {
                                let e = ();
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l2 = i32::from(*ptr0.add(1).cast::<u8>());
                                    super::super::super::wasi::sockets::network::ErrorCode::_lift(
                                        l2 as u8,
                                    )
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl UdpSocket {
                #[allow(unused_unsafe, clippy::all)]
                pub fn subscribe(&self) -> Pollable {
                    unsafe {
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:sockets/udp@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]udp-socket.subscribe"]
                            fn wit_import(_: i32) -> i32;
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32) -> i32 {
                            unreachable!()
                        }
                        let ret = wit_import((self).handle() as i32);
                        super::super::super::wasi::io::poll::Pollable::from_handle(ret as u32)
                    }
                }
            }
            impl IncomingDatagramStream {
                #[allow(unused_unsafe, clippy::all)]
                pub fn receive(
                    &self,
                    max_results: u64,
                ) -> Result<_rt::Vec<IncomingDatagram>, ErrorCode> {
                    unsafe {
                        #[repr(align(4))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 12]);
                        let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 12]);
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:sockets/udp@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]incoming-datagram-stream.receive"]
                            fn wit_import(_: i32, _: i64, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: i64, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, _rt::as_i64(&max_results), ptr0);
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        match l1 {
                            0 => {
                                let e = {
                                    let l2 = *ptr0.add(4).cast::<*mut u8>();
                                    let l3 = *ptr0.add(8).cast::<usize>();
                                    let base25 = l2;
                                    let len25 = l3;
                                    let mut result25 = _rt::Vec::with_capacity(len25);
                                    for i in 0..len25 {
                                        let base = base25.add(i * 40);
                                        let e25 = {
                                            let l4 = *base.add(0).cast::<*mut u8>();
                                            let l5 = *base.add(4).cast::<usize>();
                                            let len6 = l5;
                                            let l7 = i32::from(*base.add(8).cast::<u8>());
                                            use super::super::super::wasi::sockets::network::IpSocketAddress as V24;
                                            let v24 = match l7 {
                                                0 => {
                                                    let e24 = {
                                                        let l8 =
                                                            i32::from(*base.add(12).cast::<u16>());
                                                        let l9 =
                                                            i32::from(*base.add(14).cast::<u8>());
                                                        let l10 =
                                                            i32::from(*base.add(15).cast::<u8>());
                                                        let l11 =
                                                            i32::from(*base.add(16).cast::<u8>());
                                                        let l12 =
                                                            i32::from(*base.add(17).cast::<u8>());
                                                        super::super::super::wasi::sockets::network::Ipv4SocketAddress {
                                                            port: l8 as u16,
                                                            address: (l9 as u8, l10 as u8, l11 as u8, l12 as u8),
                                                        }
                                                    };
                                                    V24::Ipv4(e24)
                                                }
                                                n => {
                                                    debug_assert_eq!(
                                                        n, 1,
                                                        "invalid enum discriminant"
                                                    );
                                                    let e24 = {
                                                        let l13 =
                                                            i32::from(*base.add(12).cast::<u16>());
                                                        let l14 = *base.add(16).cast::<i32>();
                                                        let l15 =
                                                            i32::from(*base.add(20).cast::<u16>());
                                                        let l16 =
                                                            i32::from(*base.add(22).cast::<u16>());
                                                        let l17 =
                                                            i32::from(*base.add(24).cast::<u16>());
                                                        let l18 =
                                                            i32::from(*base.add(26).cast::<u16>());
                                                        let l19 =
                                                            i32::from(*base.add(28).cast::<u16>());
                                                        let l20 =
                                                            i32::from(*base.add(30).cast::<u16>());
                                                        let l21 =
                                                            i32::from(*base.add(32).cast::<u16>());
                                                        let l22 =
                                                            i32::from(*base.add(34).cast::<u16>());
                                                        let l23 = *base.add(36).cast::<i32>();
                                                        super::super::super::wasi::sockets::network::Ipv6SocketAddress {
                                                            port: l13 as u16,
                                                            flow_info: l14 as u32,
                                                            address: (
                                                                l15 as u16,
                                                                l16 as u16,
                                                                l17 as u16,
                                                                l18 as u16,
                                                                l19 as u16,
                                                                l20 as u16,
                                                                l21 as u16,
                                                                l22 as u16,
                                                            ),
                                                            scope_id: l23 as u32,
                                                        }
                                                    };
                                                    V24::Ipv6(e24)
                                                }
                                            };
                                            IncomingDatagram {
                                                data: _rt::Vec::from_raw_parts(
                                                    l4.cast(),
                                                    len6,
                                                    len6,
                                                ),
                                                remote_address: v24,
                                            }
                                        };
                                        result25.push(e25);
                                    }
                                    _rt::cabi_dealloc(base25, len25 * 40, 4);
                                    result25
                                };
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l26 = i32::from(*ptr0.add(4).cast::<u8>());
                                    super::super::super::wasi::sockets::network::ErrorCode::_lift(
                                        l26 as u8,
                                    )
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl IncomingDatagramStream {
                #[allow(unused_unsafe, clippy::all)]
                pub fn subscribe(&self) -> Pollable {
                    unsafe {
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:sockets/udp@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]incoming-datagram-stream.subscribe"]
                            fn wit_import(_: i32) -> i32;
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32) -> i32 {
                            unreachable!()
                        }
                        let ret = wit_import((self).handle() as i32);
                        super::super::super::wasi::io::poll::Pollable::from_handle(ret as u32)
                    }
                }
            }
            impl OutgoingDatagramStream {
                #[allow(unused_unsafe, clippy::all)]
                pub fn check_send(&self) -> Result<u64, ErrorCode> {
                    unsafe {
                        #[repr(align(8))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 16]);
                        let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 16]);
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:sockets/udp@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]outgoing-datagram-stream.check-send"]
                            fn wit_import(_: i32, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, ptr0);
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        match l1 {
                            0 => {
                                let e = {
                                    let l2 = *ptr0.add(8).cast::<i64>();
                                    l2 as u64
                                };
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l3 = i32::from(*ptr0.add(8).cast::<u8>());
                                    super::super::super::wasi::sockets::network::ErrorCode::_lift(
                                        l3 as u8,
                                    )
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl OutgoingDatagramStream {
                #[allow(unused_unsafe, clippy::all)]
                pub fn send(&self, datagrams: &[OutgoingDatagram]) -> Result<u64, ErrorCode> {
                    unsafe {
                        #[repr(align(8))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 16]);
                        let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 16]);
                        let vec7 = datagrams;
                        let len7 = vec7.len();
                        let layout7 =
                            _rt::alloc::Layout::from_size_align_unchecked(vec7.len() * 44, 4);
                        let result7 = if layout7.size() != 0 {
                            let ptr = _rt::alloc::alloc(layout7).cast::<u8>();
                            if ptr.is_null() {
                                _rt::alloc::handle_alloc_error(layout7);
                            }
                            ptr
                        } else {
                            ::core::ptr::null_mut()
                        };
                        for (i, e) in vec7.into_iter().enumerate() {
                            let base = result7.add(i * 44);
                            {
                                let OutgoingDatagram {
                                    data: data0,
                                    remote_address: remote_address0,
                                } = e;
                                let vec1 = data0;
                                let ptr1 = vec1.as_ptr().cast::<u8>();
                                let len1 = vec1.len();
                                *base.add(4).cast::<usize>() = len1;
                                *base.add(0).cast::<*mut u8>() = ptr1.cast_mut();
                                match remote_address0 {
                                    Some(e) => {
                                        *base.add(8).cast::<u8>() = (1i32) as u8;
                                        use super::super::super::wasi::sockets::network::IpSocketAddress as V6;
                                        match e {
                                            V6::Ipv4(e) => {
                                                *base.add(12).cast::<u8>() = (0i32) as u8;
                                                let super::super::super::wasi::sockets::network::Ipv4SocketAddress {
                                                    port: port2,
                                                    address: address2,
                                                } = e;
                                                *base.add(16).cast::<u16>() =
                                                    (_rt::as_i32(port2)) as u16;
                                                let (t3_0, t3_1, t3_2, t3_3) = address2;
                                                *base.add(18).cast::<u8>() =
                                                    (_rt::as_i32(t3_0)) as u8;
                                                *base.add(19).cast::<u8>() =
                                                    (_rt::as_i32(t3_1)) as u8;
                                                *base.add(20).cast::<u8>() =
                                                    (_rt::as_i32(t3_2)) as u8;
                                                *base.add(21).cast::<u8>() =
                                                    (_rt::as_i32(t3_3)) as u8;
                                            }
                                            V6::Ipv6(e) => {
                                                *base.add(12).cast::<u8>() = (1i32) as u8;
                                                let super::super::super::wasi::sockets::network::Ipv6SocketAddress {
                                                    port: port4,
                                                    flow_info: flow_info4,
                                                    address: address4,
                                                    scope_id: scope_id4,
                                                } = e;
                                                *base.add(16).cast::<u16>() =
                                                    (_rt::as_i32(port4)) as u16;
                                                *base.add(20).cast::<i32>() =
                                                    _rt::as_i32(flow_info4);
                                                let (
                                                    t5_0,
                                                    t5_1,
                                                    t5_2,
                                                    t5_3,
                                                    t5_4,
                                                    t5_5,
                                                    t5_6,
                                                    t5_7,
                                                ) = address4;
                                                *base.add(24).cast::<u16>() =
                                                    (_rt::as_i32(t5_0)) as u16;
                                                *base.add(26).cast::<u16>() =
                                                    (_rt::as_i32(t5_1)) as u16;
                                                *base.add(28).cast::<u16>() =
                                                    (_rt::as_i32(t5_2)) as u16;
                                                *base.add(30).cast::<u16>() =
                                                    (_rt::as_i32(t5_3)) as u16;
                                                *base.add(32).cast::<u16>() =
                                                    (_rt::as_i32(t5_4)) as u16;
                                                *base.add(34).cast::<u16>() =
                                                    (_rt::as_i32(t5_5)) as u16;
                                                *base.add(36).cast::<u16>() =
                                                    (_rt::as_i32(t5_6)) as u16;
                                                *base.add(38).cast::<u16>() =
                                                    (_rt::as_i32(t5_7)) as u16;
                                                *base.add(40).cast::<i32>() =
                                                    _rt::as_i32(scope_id4);
                                            }
                                        }
                                    }
                                    None => {
                                        *base.add(8).cast::<u8>() = (0i32) as u8;
                                    }
                                };
                            }
                        }
                        let ptr8 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:sockets/udp@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]outgoing-datagram-stream.send"]
                            fn wit_import(_: i32, _: *mut u8, _: usize, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: *mut u8, _: usize, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, result7, len7, ptr8);
                        let l9 = i32::from(*ptr8.add(0).cast::<u8>());
                        if layout7.size() != 0 {
                            _rt::alloc::dealloc(result7.cast(), layout7);
                        }
                        match l9 {
                            0 => {
                                let e = {
                                    let l10 = *ptr8.add(8).cast::<i64>();
                                    l10 as u64
                                };
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l11 = i32::from(*ptr8.add(8).cast::<u8>());
                                    super::super::super::wasi::sockets::network::ErrorCode::_lift(
                                        l11 as u8,
                                    )
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl OutgoingDatagramStream {
                #[allow(unused_unsafe, clippy::all)]
                pub fn subscribe(&self) -> Pollable {
                    unsafe {
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:sockets/udp@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]outgoing-datagram-stream.subscribe"]
                            fn wit_import(_: i32) -> i32;
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32) -> i32 {
                            unreachable!()
                        }
                        let ret = wit_import((self).handle() as i32);
                        super::super::super::wasi::io::poll::Pollable::from_handle(ret as u32)
                    }
                }
            }
        }
        #[allow(dead_code, clippy::all)]
        pub mod udp_create_socket {
            #[used]
            #[doc(hidden)]
            static __FORCE_SECTION_REF: fn() =
                super::super::super::__link_custom_section_describing_imports;
            use super::super::super::_rt;
            pub type ErrorCode = super::super::super::wasi::sockets::network::ErrorCode;
            pub type IpAddressFamily = super::super::super::wasi::sockets::network::IpAddressFamily;
            pub type UdpSocket = super::super::super::wasi::sockets::udp::UdpSocket;
            #[allow(unused_unsafe, clippy::all)]
            pub fn create_udp_socket(
                address_family: IpAddressFamily,
            ) -> Result<UdpSocket, ErrorCode> {
                unsafe {
                    #[repr(align(4))]
                    struct RetArea([::core::mem::MaybeUninit<u8>; 8]);
                    let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 8]);
                    let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                    #[cfg(target_arch = "wasm32")]
                    #[link(wasm_import_module = "wasi:sockets/udp-create-socket@0.2.0")]
                    extern "C" {
                        #[link_name = "create-udp-socket"]
                        fn wit_import(_: i32, _: *mut u8);
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import(_: i32, _: *mut u8) {
                        unreachable!()
                    }
                    wit_import(address_family.clone() as i32, ptr0);
                    let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                    match l1 {
                        0 => {
                            let e = {
                                let l2 = *ptr0.add(4).cast::<i32>();
                                super::super::super::wasi::sockets::udp::UdpSocket::from_handle(
                                    l2 as u32,
                                )
                            };
                            Ok(e)
                        }
                        1 => {
                            let e = {
                                let l3 = i32::from(*ptr0.add(4).cast::<u8>());
                                super::super::super::wasi::sockets::network::ErrorCode::_lift(
                                    l3 as u8,
                                )
                            };
                            Err(e)
                        }
                        _ => _rt::invalid_enum_discriminant(),
                    }
                }
            }
        }
        #[allow(dead_code, clippy::all)]
        pub mod tcp {
            #[used]
            #[doc(hidden)]
            static __FORCE_SECTION_REF: fn() =
                super::super::super::__link_custom_section_describing_imports;
            use super::super::super::_rt;
            pub type InputStream = super::super::super::wasi::io::streams::InputStream;
            pub type OutputStream = super::super::super::wasi::io::streams::OutputStream;
            pub type Pollable = super::super::super::wasi::io::poll::Pollable;
            pub type Duration = super::super::super::wasi::clocks::monotonic_clock::Duration;
            pub type Network = super::super::super::wasi::sockets::network::Network;
            pub type ErrorCode = super::super::super::wasi::sockets::network::ErrorCode;
            pub type IpSocketAddress = super::super::super::wasi::sockets::network::IpSocketAddress;
            pub type IpAddressFamily = super::super::super::wasi::sockets::network::IpAddressFamily;
            #[repr(u8)]
            #[derive(Clone, Copy, Eq, Ord, PartialEq, PartialOrd)]
            pub enum ShutdownType {
                Receive,
                Send,
                Both,
            }
            impl ::core::fmt::Debug for ShutdownType {
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    match self {
                        ShutdownType::Receive => f.debug_tuple("ShutdownType::Receive").finish(),
                        ShutdownType::Send => f.debug_tuple("ShutdownType::Send").finish(),
                        ShutdownType::Both => f.debug_tuple("ShutdownType::Both").finish(),
                    }
                }
            }
            impl ShutdownType {
                #[doc(hidden)]
                pub unsafe fn _lift(val: u8) -> ShutdownType {
                    if !cfg!(debug_assertions) {
                        return ::core::mem::transmute(val);
                    }
                    match val {
                        0 => ShutdownType::Receive,
                        1 => ShutdownType::Send,
                        2 => ShutdownType::Both,
                        _ => panic!("invalid enum discriminant"),
                    }
                }
            }
            #[derive(Debug)]
            #[repr(transparent)]
            pub struct TcpSocket {
                handle: _rt::Resource<TcpSocket>,
            }
            impl TcpSocket {
                #[doc(hidden)]
                pub unsafe fn from_handle(handle: u32) -> Self {
                    Self { handle: _rt::Resource::from_handle(handle) }
                }
                #[doc(hidden)]
                pub fn take_handle(&self) -> u32 {
                    _rt::Resource::take_handle(&self.handle)
                }
                #[doc(hidden)]
                pub fn handle(&self) -> u32 {
                    _rt::Resource::handle(&self.handle)
                }
            }
            unsafe impl _rt::WasmResource for TcpSocket {
                #[inline]
                unsafe fn drop(_handle: u32) {
                    #[cfg(not(target_arch = "wasm32"))]
                    unreachable!();
                    #[cfg(target_arch = "wasm32")]
                    {
                        #[link(wasm_import_module = "wasi:sockets/tcp@0.2.0")]
                        extern "C" {
                            #[link_name = "[resource-drop]tcp-socket"]
                            fn drop(_: u32);
                        }
                        drop(_handle);
                    }
                }
            }
            impl TcpSocket {
                #[allow(unused_unsafe, clippy::all)]
                pub fn start_bind(
                    &self,
                    network: &Network,
                    local_address: IpSocketAddress,
                ) -> Result<(), ErrorCode> {
                    unsafe {
                        #[repr(align(1))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 2]);
                        let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 2]);
                        use super::super::super::wasi::sockets::network::IpSocketAddress as V4;
                        let (
                            result5_0,
                            result5_1,
                            result5_2,
                            result5_3,
                            result5_4,
                            result5_5,
                            result5_6,
                            result5_7,
                            result5_8,
                            result5_9,
                            result5_10,
                            result5_11,
                        ) = match local_address {
                            V4::Ipv4(e) => {
                                let super::super::super::wasi::sockets::network::Ipv4SocketAddress {
                                    port: port0,
                                    address: address0,
                                } = e;
                                let (t1_0, t1_1, t1_2, t1_3) = address0;
                                (
                                    0i32,
                                    _rt::as_i32(port0),
                                    _rt::as_i32(t1_0),
                                    _rt::as_i32(t1_1),
                                    _rt::as_i32(t1_2),
                                    _rt::as_i32(t1_3),
                                    0i32,
                                    0i32,
                                    0i32,
                                    0i32,
                                    0i32,
                                    0i32,
                                )
                            }
                            V4::Ipv6(e) => {
                                let super::super::super::wasi::sockets::network::Ipv6SocketAddress {
                                    port: port2,
                                    flow_info: flow_info2,
                                    address: address2,
                                    scope_id: scope_id2,
                                } = e;
                                let (t3_0, t3_1, t3_2, t3_3, t3_4, t3_5, t3_6, t3_7) = address2;
                                (
                                    1i32,
                                    _rt::as_i32(port2),
                                    _rt::as_i32(flow_info2),
                                    _rt::as_i32(t3_0),
                                    _rt::as_i32(t3_1),
                                    _rt::as_i32(t3_2),
                                    _rt::as_i32(t3_3),
                                    _rt::as_i32(t3_4),
                                    _rt::as_i32(t3_5),
                                    _rt::as_i32(t3_6),
                                    _rt::as_i32(t3_7),
                                    _rt::as_i32(scope_id2),
                                )
                            }
                        };
                        let ptr6 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:sockets/tcp@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]tcp-socket.start-bind"]
                            fn wit_import(
                                _: i32,
                                _: i32,
                                _: i32,
                                _: i32,
                                _: i32,
                                _: i32,
                                _: i32,
                                _: i32,
                                _: i32,
                                _: i32,
                                _: i32,
                                _: i32,
                                _: i32,
                                _: i32,
                                _: *mut u8,
                            );
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(
                            _: i32,
                            _: i32,
                            _: i32,
                            _: i32,
                            _: i32,
                            _: i32,
                            _: i32,
                            _: i32,
                            _: i32,
                            _: i32,
                            _: i32,
                            _: i32,
                            _: i32,
                            _: i32,
                            _: *mut u8,
                        ) {
                            unreachable!()
                        }
                        wit_import(
                            (self).handle() as i32,
                            (network).handle() as i32,
                            result5_0,
                            result5_1,
                            result5_2,
                            result5_3,
                            result5_4,
                            result5_5,
                            result5_6,
                            result5_7,
                            result5_8,
                            result5_9,
                            result5_10,
                            result5_11,
                            ptr6,
                        );
                        let l7 = i32::from(*ptr6.add(0).cast::<u8>());
                        match l7 {
                            0 => {
                                let e = ();
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l8 = i32::from(*ptr6.add(1).cast::<u8>());
                                    super::super::super::wasi::sockets::network::ErrorCode::_lift(
                                        l8 as u8,
                                    )
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl TcpSocket {
                #[allow(unused_unsafe, clippy::all)]
                pub fn finish_bind(&self) -> Result<(), ErrorCode> {
                    unsafe {
                        #[repr(align(1))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 2]);
                        let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 2]);
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:sockets/tcp@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]tcp-socket.finish-bind"]
                            fn wit_import(_: i32, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, ptr0);
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        match l1 {
                            0 => {
                                let e = ();
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l2 = i32::from(*ptr0.add(1).cast::<u8>());
                                    super::super::super::wasi::sockets::network::ErrorCode::_lift(
                                        l2 as u8,
                                    )
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl TcpSocket {
                #[allow(unused_unsafe, clippy::all)]
                pub fn start_connect(
                    &self,
                    network: &Network,
                    remote_address: IpSocketAddress,
                ) -> Result<(), ErrorCode> {
                    unsafe {
                        #[repr(align(1))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 2]);
                        let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 2]);
                        use super::super::super::wasi::sockets::network::IpSocketAddress as V4;
                        let (
                            result5_0,
                            result5_1,
                            result5_2,
                            result5_3,
                            result5_4,
                            result5_5,
                            result5_6,
                            result5_7,
                            result5_8,
                            result5_9,
                            result5_10,
                            result5_11,
                        ) = match remote_address {
                            V4::Ipv4(e) => {
                                let super::super::super::wasi::sockets::network::Ipv4SocketAddress {
                                    port: port0,
                                    address: address0,
                                } = e;
                                let (t1_0, t1_1, t1_2, t1_3) = address0;
                                (
                                    0i32,
                                    _rt::as_i32(port0),
                                    _rt::as_i32(t1_0),
                                    _rt::as_i32(t1_1),
                                    _rt::as_i32(t1_2),
                                    _rt::as_i32(t1_3),
                                    0i32,
                                    0i32,
                                    0i32,
                                    0i32,
                                    0i32,
                                    0i32,
                                )
                            }
                            V4::Ipv6(e) => {
                                let super::super::super::wasi::sockets::network::Ipv6SocketAddress {
                                    port: port2,
                                    flow_info: flow_info2,
                                    address: address2,
                                    scope_id: scope_id2,
                                } = e;
                                let (t3_0, t3_1, t3_2, t3_3, t3_4, t3_5, t3_6, t3_7) = address2;
                                (
                                    1i32,
                                    _rt::as_i32(port2),
                                    _rt::as_i32(flow_info2),
                                    _rt::as_i32(t3_0),
                                    _rt::as_i32(t3_1),
                                    _rt::as_i32(t3_2),
                                    _rt::as_i32(t3_3),
                                    _rt::as_i32(t3_4),
                                    _rt::as_i32(t3_5),
                                    _rt::as_i32(t3_6),
                                    _rt::as_i32(t3_7),
                                    _rt::as_i32(scope_id2),
                                )
                            }
                        };
                        let ptr6 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:sockets/tcp@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]tcp-socket.start-connect"]
                            fn wit_import(
                                _: i32,
                                _: i32,
                                _: i32,
                                _: i32,
                                _: i32,
                                _: i32,
                                _: i32,
                                _: i32,
                                _: i32,
                                _: i32,
                                _: i32,
                                _: i32,
                                _: i32,
                                _: i32,
                                _: *mut u8,
                            );
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(
                            _: i32,
                            _: i32,
                            _: i32,
                            _: i32,
                            _: i32,
                            _: i32,
                            _: i32,
                            _: i32,
                            _: i32,
                            _: i32,
                            _: i32,
                            _: i32,
                            _: i32,
                            _: i32,
                            _: *mut u8,
                        ) {
                            unreachable!()
                        }
                        wit_import(
                            (self).handle() as i32,
                            (network).handle() as i32,
                            result5_0,
                            result5_1,
                            result5_2,
                            result5_3,
                            result5_4,
                            result5_5,
                            result5_6,
                            result5_7,
                            result5_8,
                            result5_9,
                            result5_10,
                            result5_11,
                            ptr6,
                        );
                        let l7 = i32::from(*ptr6.add(0).cast::<u8>());
                        match l7 {
                            0 => {
                                let e = ();
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l8 = i32::from(*ptr6.add(1).cast::<u8>());
                                    super::super::super::wasi::sockets::network::ErrorCode::_lift(
                                        l8 as u8,
                                    )
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl TcpSocket {
                #[allow(unused_unsafe, clippy::all)]
                pub fn finish_connect(&self) -> Result<(InputStream, OutputStream), ErrorCode> {
                    unsafe {
                        #[repr(align(4))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 12]);
                        let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 12]);
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:sockets/tcp@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]tcp-socket.finish-connect"]
                            fn wit_import(_: i32, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, ptr0);
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        match l1 {
                            0 => {
                                let e = {
                                    let l2 = *ptr0.add(4).cast::<i32>();
                                    let l3 = *ptr0.add(8).cast::<i32>();
                                    (
                                        super::super::super::wasi::io::streams::InputStream::from_handle(
                                            l2 as u32,
                                        ),
                                        super::super::super::wasi::io::streams::OutputStream::from_handle(
                                            l3 as u32,
                                        ),
                                    )
                                };
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l4 = i32::from(*ptr0.add(4).cast::<u8>());
                                    super::super::super::wasi::sockets::network::ErrorCode::_lift(
                                        l4 as u8,
                                    )
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl TcpSocket {
                #[allow(unused_unsafe, clippy::all)]
                pub fn start_listen(&self) -> Result<(), ErrorCode> {
                    unsafe {
                        #[repr(align(1))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 2]);
                        let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 2]);
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:sockets/tcp@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]tcp-socket.start-listen"]
                            fn wit_import(_: i32, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, ptr0);
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        match l1 {
                            0 => {
                                let e = ();
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l2 = i32::from(*ptr0.add(1).cast::<u8>());
                                    super::super::super::wasi::sockets::network::ErrorCode::_lift(
                                        l2 as u8,
                                    )
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl TcpSocket {
                #[allow(unused_unsafe, clippy::all)]
                pub fn finish_listen(&self) -> Result<(), ErrorCode> {
                    unsafe {
                        #[repr(align(1))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 2]);
                        let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 2]);
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:sockets/tcp@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]tcp-socket.finish-listen"]
                            fn wit_import(_: i32, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, ptr0);
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        match l1 {
                            0 => {
                                let e = ();
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l2 = i32::from(*ptr0.add(1).cast::<u8>());
                                    super::super::super::wasi::sockets::network::ErrorCode::_lift(
                                        l2 as u8,
                                    )
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl TcpSocket {
                #[allow(unused_unsafe, clippy::all)]
                pub fn accept(&self) -> Result<(TcpSocket, InputStream, OutputStream), ErrorCode> {
                    unsafe {
                        #[repr(align(4))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 16]);
                        let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 16]);
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:sockets/tcp@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]tcp-socket.accept"]
                            fn wit_import(_: i32, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, ptr0);
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        match l1 {
                            0 => {
                                let e = {
                                    let l2 = *ptr0.add(4).cast::<i32>();
                                    let l3 = *ptr0.add(8).cast::<i32>();
                                    let l4 = *ptr0.add(12).cast::<i32>();
                                    (
                                        TcpSocket::from_handle(l2 as u32),
                                        super::super::super::wasi::io::streams::InputStream::from_handle(
                                            l3 as u32,
                                        ),
                                        super::super::super::wasi::io::streams::OutputStream::from_handle(
                                            l4 as u32,
                                        ),
                                    )
                                };
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l5 = i32::from(*ptr0.add(4).cast::<u8>());
                                    super::super::super::wasi::sockets::network::ErrorCode::_lift(
                                        l5 as u8,
                                    )
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl TcpSocket {
                #[allow(unused_unsafe, clippy::all)]
                pub fn local_address(&self) -> Result<IpSocketAddress, ErrorCode> {
                    unsafe {
                        #[repr(align(4))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 36]);
                        let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 36]);
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:sockets/tcp@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]tcp-socket.local-address"]
                            fn wit_import(_: i32, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, ptr0);
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        match l1 {
                            0 => {
                                let e = {
                                    let l2 = i32::from(*ptr0.add(4).cast::<u8>());
                                    use super::super::super::wasi::sockets::network::IpSocketAddress as V19;
                                    let v19 = match l2 {
                                        0 => {
                                            let e19 = {
                                                let l3 = i32::from(*ptr0.add(8).cast::<u16>());
                                                let l4 = i32::from(*ptr0.add(10).cast::<u8>());
                                                let l5 = i32::from(*ptr0.add(11).cast::<u8>());
                                                let l6 = i32::from(*ptr0.add(12).cast::<u8>());
                                                let l7 = i32::from(*ptr0.add(13).cast::<u8>());
                                                super::super::super::wasi::sockets::network::Ipv4SocketAddress {
                                                    port: l3 as u16,
                                                    address: (l4 as u8, l5 as u8, l6 as u8, l7 as u8),
                                                }
                                            };
                                            V19::Ipv4(e19)
                                        }
                                        n => {
                                            debug_assert_eq!(n, 1, "invalid enum discriminant");
                                            let e19 = {
                                                let l8 = i32::from(*ptr0.add(8).cast::<u16>());
                                                let l9 = *ptr0.add(12).cast::<i32>();
                                                let l10 = i32::from(*ptr0.add(16).cast::<u16>());
                                                let l11 = i32::from(*ptr0.add(18).cast::<u16>());
                                                let l12 = i32::from(*ptr0.add(20).cast::<u16>());
                                                let l13 = i32::from(*ptr0.add(22).cast::<u16>());
                                                let l14 = i32::from(*ptr0.add(24).cast::<u16>());
                                                let l15 = i32::from(*ptr0.add(26).cast::<u16>());
                                                let l16 = i32::from(*ptr0.add(28).cast::<u16>());
                                                let l17 = i32::from(*ptr0.add(30).cast::<u16>());
                                                let l18 = *ptr0.add(32).cast::<i32>();
                                                super::super::super::wasi::sockets::network::Ipv6SocketAddress {
                                                    port: l8 as u16,
                                                    flow_info: l9 as u32,
                                                    address: (
                                                        l10 as u16,
                                                        l11 as u16,
                                                        l12 as u16,
                                                        l13 as u16,
                                                        l14 as u16,
                                                        l15 as u16,
                                                        l16 as u16,
                                                        l17 as u16,
                                                    ),
                                                    scope_id: l18 as u32,
                                                }
                                            };
                                            V19::Ipv6(e19)
                                        }
                                    };
                                    v19
                                };
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l20 = i32::from(*ptr0.add(4).cast::<u8>());
                                    super::super::super::wasi::sockets::network::ErrorCode::_lift(
                                        l20 as u8,
                                    )
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl TcpSocket {
                #[allow(unused_unsafe, clippy::all)]
                pub fn remote_address(&self) -> Result<IpSocketAddress, ErrorCode> {
                    unsafe {
                        #[repr(align(4))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 36]);
                        let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 36]);
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:sockets/tcp@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]tcp-socket.remote-address"]
                            fn wit_import(_: i32, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, ptr0);
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        match l1 {
                            0 => {
                                let e = {
                                    let l2 = i32::from(*ptr0.add(4).cast::<u8>());
                                    use super::super::super::wasi::sockets::network::IpSocketAddress as V19;
                                    let v19 = match l2 {
                                        0 => {
                                            let e19 = {
                                                let l3 = i32::from(*ptr0.add(8).cast::<u16>());
                                                let l4 = i32::from(*ptr0.add(10).cast::<u8>());
                                                let l5 = i32::from(*ptr0.add(11).cast::<u8>());
                                                let l6 = i32::from(*ptr0.add(12).cast::<u8>());
                                                let l7 = i32::from(*ptr0.add(13).cast::<u8>());
                                                super::super::super::wasi::sockets::network::Ipv4SocketAddress {
                                                    port: l3 as u16,
                                                    address: (l4 as u8, l5 as u8, l6 as u8, l7 as u8),
                                                }
                                            };
                                            V19::Ipv4(e19)
                                        }
                                        n => {
                                            debug_assert_eq!(n, 1, "invalid enum discriminant");
                                            let e19 = {
                                                let l8 = i32::from(*ptr0.add(8).cast::<u16>());
                                                let l9 = *ptr0.add(12).cast::<i32>();
                                                let l10 = i32::from(*ptr0.add(16).cast::<u16>());
                                                let l11 = i32::from(*ptr0.add(18).cast::<u16>());
                                                let l12 = i32::from(*ptr0.add(20).cast::<u16>());
                                                let l13 = i32::from(*ptr0.add(22).cast::<u16>());
                                                let l14 = i32::from(*ptr0.add(24).cast::<u16>());
                                                let l15 = i32::from(*ptr0.add(26).cast::<u16>());
                                                let l16 = i32::from(*ptr0.add(28).cast::<u16>());
                                                let l17 = i32::from(*ptr0.add(30).cast::<u16>());
                                                let l18 = *ptr0.add(32).cast::<i32>();
                                                super::super::super::wasi::sockets::network::Ipv6SocketAddress {
                                                    port: l8 as u16,
                                                    flow_info: l9 as u32,
                                                    address: (
                                                        l10 as u16,
                                                        l11 as u16,
                                                        l12 as u16,
                                                        l13 as u16,
                                                        l14 as u16,
                                                        l15 as u16,
                                                        l16 as u16,
                                                        l17 as u16,
                                                    ),
                                                    scope_id: l18 as u32,
                                                }
                                            };
                                            V19::Ipv6(e19)
                                        }
                                    };
                                    v19
                                };
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l20 = i32::from(*ptr0.add(4).cast::<u8>());
                                    super::super::super::wasi::sockets::network::ErrorCode::_lift(
                                        l20 as u8,
                                    )
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl TcpSocket {
                #[allow(unused_unsafe, clippy::all)]
                pub fn is_listening(&self) -> bool {
                    unsafe {
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:sockets/tcp@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]tcp-socket.is-listening"]
                            fn wit_import(_: i32) -> i32;
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32) -> i32 {
                            unreachable!()
                        }
                        let ret = wit_import((self).handle() as i32);
                        _rt::bool_lift(ret as u8)
                    }
                }
            }
            impl TcpSocket {
                #[allow(unused_unsafe, clippy::all)]
                pub fn address_family(&self) -> IpAddressFamily {
                    unsafe {
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:sockets/tcp@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]tcp-socket.address-family"]
                            fn wit_import(_: i32) -> i32;
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32) -> i32 {
                            unreachable!()
                        }
                        let ret = wit_import((self).handle() as i32);
                        super::super::super::wasi::sockets::network::IpAddressFamily::_lift(
                            ret as u8,
                        )
                    }
                }
            }
            impl TcpSocket {
                #[allow(unused_unsafe, clippy::all)]
                pub fn set_listen_backlog_size(&self, value: u64) -> Result<(), ErrorCode> {
                    unsafe {
                        #[repr(align(1))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 2]);
                        let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 2]);
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:sockets/tcp@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]tcp-socket.set-listen-backlog-size"]
                            fn wit_import(_: i32, _: i64, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: i64, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, _rt::as_i64(&value), ptr0);
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        match l1 {
                            0 => {
                                let e = ();
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l2 = i32::from(*ptr0.add(1).cast::<u8>());
                                    super::super::super::wasi::sockets::network::ErrorCode::_lift(
                                        l2 as u8,
                                    )
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl TcpSocket {
                #[allow(unused_unsafe, clippy::all)]
                pub fn keep_alive_enabled(&self) -> Result<bool, ErrorCode> {
                    unsafe {
                        #[repr(align(1))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 2]);
                        let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 2]);
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:sockets/tcp@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]tcp-socket.keep-alive-enabled"]
                            fn wit_import(_: i32, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, ptr0);
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        match l1 {
                            0 => {
                                let e = {
                                    let l2 = i32::from(*ptr0.add(1).cast::<u8>());
                                    _rt::bool_lift(l2 as u8)
                                };
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l3 = i32::from(*ptr0.add(1).cast::<u8>());
                                    super::super::super::wasi::sockets::network::ErrorCode::_lift(
                                        l3 as u8,
                                    )
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl TcpSocket {
                #[allow(unused_unsafe, clippy::all)]
                pub fn set_keep_alive_enabled(&self, value: bool) -> Result<(), ErrorCode> {
                    unsafe {
                        #[repr(align(1))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 2]);
                        let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 2]);
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:sockets/tcp@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]tcp-socket.set-keep-alive-enabled"]
                            fn wit_import(_: i32, _: i32, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: i32, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import(
                            (self).handle() as i32,
                            match &value {
                                true => 1,
                                false => 0,
                            },
                            ptr0,
                        );
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        match l1 {
                            0 => {
                                let e = ();
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l2 = i32::from(*ptr0.add(1).cast::<u8>());
                                    super::super::super::wasi::sockets::network::ErrorCode::_lift(
                                        l2 as u8,
                                    )
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl TcpSocket {
                #[allow(unused_unsafe, clippy::all)]
                pub fn keep_alive_idle_time(&self) -> Result<Duration, ErrorCode> {
                    unsafe {
                        #[repr(align(8))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 16]);
                        let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 16]);
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:sockets/tcp@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]tcp-socket.keep-alive-idle-time"]
                            fn wit_import(_: i32, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, ptr0);
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        match l1 {
                            0 => {
                                let e = {
                                    let l2 = *ptr0.add(8).cast::<i64>();
                                    l2 as u64
                                };
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l3 = i32::from(*ptr0.add(8).cast::<u8>());
                                    super::super::super::wasi::sockets::network::ErrorCode::_lift(
                                        l3 as u8,
                                    )
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl TcpSocket {
                #[allow(unused_unsafe, clippy::all)]
                pub fn set_keep_alive_idle_time(&self, value: Duration) -> Result<(), ErrorCode> {
                    unsafe {
                        #[repr(align(1))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 2]);
                        let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 2]);
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:sockets/tcp@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]tcp-socket.set-keep-alive-idle-time"]
                            fn wit_import(_: i32, _: i64, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: i64, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, _rt::as_i64(value), ptr0);
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        match l1 {
                            0 => {
                                let e = ();
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l2 = i32::from(*ptr0.add(1).cast::<u8>());
                                    super::super::super::wasi::sockets::network::ErrorCode::_lift(
                                        l2 as u8,
                                    )
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl TcpSocket {
                #[allow(unused_unsafe, clippy::all)]
                pub fn keep_alive_interval(&self) -> Result<Duration, ErrorCode> {
                    unsafe {
                        #[repr(align(8))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 16]);
                        let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 16]);
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:sockets/tcp@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]tcp-socket.keep-alive-interval"]
                            fn wit_import(_: i32, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, ptr0);
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        match l1 {
                            0 => {
                                let e = {
                                    let l2 = *ptr0.add(8).cast::<i64>();
                                    l2 as u64
                                };
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l3 = i32::from(*ptr0.add(8).cast::<u8>());
                                    super::super::super::wasi::sockets::network::ErrorCode::_lift(
                                        l3 as u8,
                                    )
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl TcpSocket {
                #[allow(unused_unsafe, clippy::all)]
                pub fn set_keep_alive_interval(&self, value: Duration) -> Result<(), ErrorCode> {
                    unsafe {
                        #[repr(align(1))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 2]);
                        let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 2]);
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:sockets/tcp@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]tcp-socket.set-keep-alive-interval"]
                            fn wit_import(_: i32, _: i64, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: i64, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, _rt::as_i64(value), ptr0);
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        match l1 {
                            0 => {
                                let e = ();
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l2 = i32::from(*ptr0.add(1).cast::<u8>());
                                    super::super::super::wasi::sockets::network::ErrorCode::_lift(
                                        l2 as u8,
                                    )
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl TcpSocket {
                #[allow(unused_unsafe, clippy::all)]
                pub fn keep_alive_count(&self) -> Result<u32, ErrorCode> {
                    unsafe {
                        #[repr(align(4))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 8]);
                        let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 8]);
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:sockets/tcp@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]tcp-socket.keep-alive-count"]
                            fn wit_import(_: i32, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, ptr0);
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        match l1 {
                            0 => {
                                let e = {
                                    let l2 = *ptr0.add(4).cast::<i32>();
                                    l2 as u32
                                };
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l3 = i32::from(*ptr0.add(4).cast::<u8>());
                                    super::super::super::wasi::sockets::network::ErrorCode::_lift(
                                        l3 as u8,
                                    )
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl TcpSocket {
                #[allow(unused_unsafe, clippy::all)]
                pub fn set_keep_alive_count(&self, value: u32) -> Result<(), ErrorCode> {
                    unsafe {
                        #[repr(align(1))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 2]);
                        let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 2]);
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:sockets/tcp@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]tcp-socket.set-keep-alive-count"]
                            fn wit_import(_: i32, _: i32, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: i32, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, _rt::as_i32(&value), ptr0);
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        match l1 {
                            0 => {
                                let e = ();
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l2 = i32::from(*ptr0.add(1).cast::<u8>());
                                    super::super::super::wasi::sockets::network::ErrorCode::_lift(
                                        l2 as u8,
                                    )
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl TcpSocket {
                #[allow(unused_unsafe, clippy::all)]
                pub fn hop_limit(&self) -> Result<u8, ErrorCode> {
                    unsafe {
                        #[repr(align(1))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 2]);
                        let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 2]);
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:sockets/tcp@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]tcp-socket.hop-limit"]
                            fn wit_import(_: i32, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, ptr0);
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        match l1 {
                            0 => {
                                let e = {
                                    let l2 = i32::from(*ptr0.add(1).cast::<u8>());
                                    l2 as u8
                                };
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l3 = i32::from(*ptr0.add(1).cast::<u8>());
                                    super::super::super::wasi::sockets::network::ErrorCode::_lift(
                                        l3 as u8,
                                    )
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl TcpSocket {
                #[allow(unused_unsafe, clippy::all)]
                pub fn set_hop_limit(&self, value: u8) -> Result<(), ErrorCode> {
                    unsafe {
                        #[repr(align(1))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 2]);
                        let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 2]);
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:sockets/tcp@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]tcp-socket.set-hop-limit"]
                            fn wit_import(_: i32, _: i32, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: i32, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, _rt::as_i32(&value), ptr0);
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        match l1 {
                            0 => {
                                let e = ();
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l2 = i32::from(*ptr0.add(1).cast::<u8>());
                                    super::super::super::wasi::sockets::network::ErrorCode::_lift(
                                        l2 as u8,
                                    )
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl TcpSocket {
                #[allow(unused_unsafe, clippy::all)]
                pub fn receive_buffer_size(&self) -> Result<u64, ErrorCode> {
                    unsafe {
                        #[repr(align(8))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 16]);
                        let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 16]);
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:sockets/tcp@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]tcp-socket.receive-buffer-size"]
                            fn wit_import(_: i32, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, ptr0);
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        match l1 {
                            0 => {
                                let e = {
                                    let l2 = *ptr0.add(8).cast::<i64>();
                                    l2 as u64
                                };
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l3 = i32::from(*ptr0.add(8).cast::<u8>());
                                    super::super::super::wasi::sockets::network::ErrorCode::_lift(
                                        l3 as u8,
                                    )
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl TcpSocket {
                #[allow(unused_unsafe, clippy::all)]
                pub fn set_receive_buffer_size(&self, value: u64) -> Result<(), ErrorCode> {
                    unsafe {
                        #[repr(align(1))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 2]);
                        let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 2]);
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:sockets/tcp@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]tcp-socket.set-receive-buffer-size"]
                            fn wit_import(_: i32, _: i64, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: i64, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, _rt::as_i64(&value), ptr0);
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        match l1 {
                            0 => {
                                let e = ();
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l2 = i32::from(*ptr0.add(1).cast::<u8>());
                                    super::super::super::wasi::sockets::network::ErrorCode::_lift(
                                        l2 as u8,
                                    )
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl TcpSocket {
                #[allow(unused_unsafe, clippy::all)]
                pub fn send_buffer_size(&self) -> Result<u64, ErrorCode> {
                    unsafe {
                        #[repr(align(8))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 16]);
                        let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 16]);
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:sockets/tcp@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]tcp-socket.send-buffer-size"]
                            fn wit_import(_: i32, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, ptr0);
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        match l1 {
                            0 => {
                                let e = {
                                    let l2 = *ptr0.add(8).cast::<i64>();
                                    l2 as u64
                                };
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l3 = i32::from(*ptr0.add(8).cast::<u8>());
                                    super::super::super::wasi::sockets::network::ErrorCode::_lift(
                                        l3 as u8,
                                    )
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl TcpSocket {
                #[allow(unused_unsafe, clippy::all)]
                pub fn set_send_buffer_size(&self, value: u64) -> Result<(), ErrorCode> {
                    unsafe {
                        #[repr(align(1))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 2]);
                        let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 2]);
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:sockets/tcp@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]tcp-socket.set-send-buffer-size"]
                            fn wit_import(_: i32, _: i64, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: i64, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, _rt::as_i64(&value), ptr0);
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        match l1 {
                            0 => {
                                let e = ();
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l2 = i32::from(*ptr0.add(1).cast::<u8>());
                                    super::super::super::wasi::sockets::network::ErrorCode::_lift(
                                        l2 as u8,
                                    )
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl TcpSocket {
                #[allow(unused_unsafe, clippy::all)]
                pub fn subscribe(&self) -> Pollable {
                    unsafe {
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:sockets/tcp@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]tcp-socket.subscribe"]
                            fn wit_import(_: i32) -> i32;
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32) -> i32 {
                            unreachable!()
                        }
                        let ret = wit_import((self).handle() as i32);
                        super::super::super::wasi::io::poll::Pollable::from_handle(ret as u32)
                    }
                }
            }
            impl TcpSocket {
                #[allow(unused_unsafe, clippy::all)]
                pub fn shutdown(&self, shutdown_type: ShutdownType) -> Result<(), ErrorCode> {
                    unsafe {
                        #[repr(align(1))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 2]);
                        let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 2]);
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:sockets/tcp@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]tcp-socket.shutdown"]
                            fn wit_import(_: i32, _: i32, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: i32, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, shutdown_type.clone() as i32, ptr0);
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        match l1 {
                            0 => {
                                let e = ();
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l2 = i32::from(*ptr0.add(1).cast::<u8>());
                                    super::super::super::wasi::sockets::network::ErrorCode::_lift(
                                        l2 as u8,
                                    )
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
        }
        #[allow(dead_code, clippy::all)]
        pub mod tcp_create_socket {
            #[used]
            #[doc(hidden)]
            static __FORCE_SECTION_REF: fn() =
                super::super::super::__link_custom_section_describing_imports;
            use super::super::super::_rt;
            pub type ErrorCode = super::super::super::wasi::sockets::network::ErrorCode;
            pub type IpAddressFamily = super::super::super::wasi::sockets::network::IpAddressFamily;
            pub type TcpSocket = super::super::super::wasi::sockets::tcp::TcpSocket;
            #[allow(unused_unsafe, clippy::all)]
            pub fn create_tcp_socket(
                address_family: IpAddressFamily,
            ) -> Result<TcpSocket, ErrorCode> {
                unsafe {
                    #[repr(align(4))]
                    struct RetArea([::core::mem::MaybeUninit<u8>; 8]);
                    let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 8]);
                    let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                    #[cfg(target_arch = "wasm32")]
                    #[link(wasm_import_module = "wasi:sockets/tcp-create-socket@0.2.0")]
                    extern "C" {
                        #[link_name = "create-tcp-socket"]
                        fn wit_import(_: i32, _: *mut u8);
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import(_: i32, _: *mut u8) {
                        unreachable!()
                    }
                    wit_import(address_family.clone() as i32, ptr0);
                    let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                    match l1 {
                        0 => {
                            let e = {
                                let l2 = *ptr0.add(4).cast::<i32>();
                                super::super::super::wasi::sockets::tcp::TcpSocket::from_handle(
                                    l2 as u32,
                                )
                            };
                            Ok(e)
                        }
                        1 => {
                            let e = {
                                let l3 = i32::from(*ptr0.add(4).cast::<u8>());
                                super::super::super::wasi::sockets::network::ErrorCode::_lift(
                                    l3 as u8,
                                )
                            };
                            Err(e)
                        }
                        _ => _rt::invalid_enum_discriminant(),
                    }
                }
            }
        }
        #[allow(dead_code, clippy::all)]
        pub mod ip_name_lookup {
            #[used]
            #[doc(hidden)]
            static __FORCE_SECTION_REF: fn() =
                super::super::super::__link_custom_section_describing_imports;
            use super::super::super::_rt;
            pub type Pollable = super::super::super::wasi::io::poll::Pollable;
            pub type Network = super::super::super::wasi::sockets::network::Network;
            pub type ErrorCode = super::super::super::wasi::sockets::network::ErrorCode;
            pub type IpAddress = super::super::super::wasi::sockets::network::IpAddress;
            #[derive(Debug)]
            #[repr(transparent)]
            pub struct ResolveAddressStream {
                handle: _rt::Resource<ResolveAddressStream>,
            }
            impl ResolveAddressStream {
                #[doc(hidden)]
                pub unsafe fn from_handle(handle: u32) -> Self {
                    Self { handle: _rt::Resource::from_handle(handle) }
                }
                #[doc(hidden)]
                pub fn take_handle(&self) -> u32 {
                    _rt::Resource::take_handle(&self.handle)
                }
                #[doc(hidden)]
                pub fn handle(&self) -> u32 {
                    _rt::Resource::handle(&self.handle)
                }
            }
            unsafe impl _rt::WasmResource for ResolveAddressStream {
                #[inline]
                unsafe fn drop(_handle: u32) {
                    #[cfg(not(target_arch = "wasm32"))]
                    unreachable!();
                    #[cfg(target_arch = "wasm32")]
                    {
                        #[link(wasm_import_module = "wasi:sockets/ip-name-lookup@0.2.0")]
                        extern "C" {
                            #[link_name = "[resource-drop]resolve-address-stream"]
                            fn drop(_: u32);
                        }
                        drop(_handle);
                    }
                }
            }
            impl ResolveAddressStream {
                #[allow(unused_unsafe, clippy::all)]
                pub fn resolve_next_address(&self) -> Result<Option<IpAddress>, ErrorCode> {
                    unsafe {
                        #[repr(align(2))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 22]);
                        let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 22]);
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:sockets/ip-name-lookup@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]resolve-address-stream.resolve-next-address"]
                            fn wit_import(_: i32, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, ptr0);
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        match l1 {
                            0 => {
                                let e = {
                                    let l2 = i32::from(*ptr0.add(2).cast::<u8>());
                                    match l2 {
                                        0 => None,
                                        1 => {
                                            let e = {
                                                let l3 = i32::from(*ptr0.add(4).cast::<u8>());
                                                use super::super::super::wasi::sockets::network::IpAddress as V16;
                                                let v16 = match l3 {
                                                    0 => {
                                                        let e16 = {
                                                            let l4 = i32::from(
                                                                *ptr0.add(6).cast::<u8>(),
                                                            );
                                                            let l5 = i32::from(
                                                                *ptr0.add(7).cast::<u8>(),
                                                            );
                                                            let l6 = i32::from(
                                                                *ptr0.add(8).cast::<u8>(),
                                                            );
                                                            let l7 = i32::from(
                                                                *ptr0.add(9).cast::<u8>(),
                                                            );
                                                            (l4 as u8, l5 as u8, l6 as u8, l7 as u8)
                                                        };
                                                        V16::Ipv4(e16)
                                                    }
                                                    n => {
                                                        debug_assert_eq!(
                                                            n, 1,
                                                            "invalid enum discriminant"
                                                        );
                                                        let e16 = {
                                                            let l8 = i32::from(
                                                                *ptr0.add(6).cast::<u16>(),
                                                            );
                                                            let l9 = i32::from(
                                                                *ptr0.add(8).cast::<u16>(),
                                                            );
                                                            let l10 = i32::from(
                                                                *ptr0.add(10).cast::<u16>(),
                                                            );
                                                            let l11 = i32::from(
                                                                *ptr0.add(12).cast::<u16>(),
                                                            );
                                                            let l12 = i32::from(
                                                                *ptr0.add(14).cast::<u16>(),
                                                            );
                                                            let l13 = i32::from(
                                                                *ptr0.add(16).cast::<u16>(),
                                                            );
                                                            let l14 = i32::from(
                                                                *ptr0.add(18).cast::<u16>(),
                                                            );
                                                            let l15 = i32::from(
                                                                *ptr0.add(20).cast::<u16>(),
                                                            );
                                                            (
                                                                l8 as u16, l9 as u16, l10 as u16,
                                                                l11 as u16, l12 as u16, l13 as u16,
                                                                l14 as u16, l15 as u16,
                                                            )
                                                        };
                                                        V16::Ipv6(e16)
                                                    }
                                                };
                                                v16
                                            };
                                            Some(e)
                                        }
                                        _ => _rt::invalid_enum_discriminant(),
                                    }
                                };
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l17 = i32::from(*ptr0.add(2).cast::<u8>());
                                    super::super::super::wasi::sockets::network::ErrorCode::_lift(
                                        l17 as u8,
                                    )
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl ResolveAddressStream {
                #[allow(unused_unsafe, clippy::all)]
                pub fn subscribe(&self) -> Pollable {
                    unsafe {
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:sockets/ip-name-lookup@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]resolve-address-stream.subscribe"]
                            fn wit_import(_: i32) -> i32;
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32) -> i32 {
                            unreachable!()
                        }
                        let ret = wit_import((self).handle() as i32);
                        super::super::super::wasi::io::poll::Pollable::from_handle(ret as u32)
                    }
                }
            }
            #[allow(unused_unsafe, clippy::all)]
            pub fn resolve_addresses(
                network: &Network,
                name: &str,
            ) -> Result<ResolveAddressStream, ErrorCode> {
                unsafe {
                    #[repr(align(4))]
                    struct RetArea([::core::mem::MaybeUninit<u8>; 8]);
                    let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 8]);
                    let vec0 = name;
                    let ptr0 = vec0.as_ptr().cast::<u8>();
                    let len0 = vec0.len();
                    let ptr1 = ret_area.0.as_mut_ptr().cast::<u8>();
                    #[cfg(target_arch = "wasm32")]
                    #[link(wasm_import_module = "wasi:sockets/ip-name-lookup@0.2.0")]
                    extern "C" {
                        #[link_name = "resolve-addresses"]
                        fn wit_import(_: i32, _: *mut u8, _: usize, _: *mut u8);
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import(_: i32, _: *mut u8, _: usize, _: *mut u8) {
                        unreachable!()
                    }
                    wit_import((network).handle() as i32, ptr0.cast_mut(), len0, ptr1);
                    let l2 = i32::from(*ptr1.add(0).cast::<u8>());
                    match l2 {
                        0 => {
                            let e = {
                                let l3 = *ptr1.add(4).cast::<i32>();
                                ResolveAddressStream::from_handle(l3 as u32)
                            };
                            Ok(e)
                        }
                        1 => {
                            let e = {
                                let l4 = i32::from(*ptr1.add(4).cast::<u8>());
                                super::super::super::wasi::sockets::network::ErrorCode::_lift(
                                    l4 as u8,
                                )
                            };
                            Err(e)
                        }
                        _ => _rt::invalid_enum_discriminant(),
                    }
                }
            }
        }
    }
}
#[allow(dead_code)]
pub mod wavs {
    #[allow(dead_code)]
    pub mod worker {
        #[allow(dead_code, clippy::all)]
        pub mod layer_types {
            #[used]
            #[doc(hidden)]
            static __FORCE_SECTION_REF: fn() =
                super::super::super::__link_custom_section_describing_imports;
            use super::super::super::_rt;
            #[repr(C)]
            #[derive(Clone, Copy)]
            pub struct Timestamp {
                pub nanos: u64,
            }
            impl ::core::fmt::Debug for Timestamp {
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    f.debug_struct("Timestamp").field("nanos", &self.nanos).finish()
                }
            }
            #[derive(Clone)]
            pub struct CosmosAddress {
                pub bech32_addr: _rt::String,
                /// prefix is the first part of the bech32 address
                pub prefix_len: u32,
            }
            impl ::core::fmt::Debug for CosmosAddress {
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    f.debug_struct("CosmosAddress")
                        .field("bech32-addr", &self.bech32_addr)
                        .field("prefix-len", &self.prefix_len)
                        .finish()
                }
            }
            #[derive(Clone)]
            pub struct CosmosEvent {
                pub ty: _rt::String,
                pub attributes: _rt::Vec<(_rt::String, _rt::String)>,
            }
            impl ::core::fmt::Debug for CosmosEvent {
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    f.debug_struct("CosmosEvent")
                        .field("ty", &self.ty)
                        .field("attributes", &self.attributes)
                        .finish()
                }
            }
            #[derive(Clone)]
            pub struct CosmosChainConfig {
                pub chain_id: _rt::String,
                pub rpc_endpoint: Option<_rt::String>,
                pub grpc_endpoint: Option<_rt::String>,
                pub grpc_web_endpoint: Option<_rt::String>,
                pub gas_price: f32,
                pub gas_denom: _rt::String,
                pub bech32_prefix: _rt::String,
            }
            impl ::core::fmt::Debug for CosmosChainConfig {
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    f.debug_struct("CosmosChainConfig")
                        .field("chain-id", &self.chain_id)
                        .field("rpc-endpoint", &self.rpc_endpoint)
                        .field("grpc-endpoint", &self.grpc_endpoint)
                        .field("grpc-web-endpoint", &self.grpc_web_endpoint)
                        .field("gas-price", &self.gas_price)
                        .field("gas-denom", &self.gas_denom)
                        .field("bech32-prefix", &self.bech32_prefix)
                        .finish()
                }
            }
            #[derive(Clone)]
            pub struct EvmAddress {
                pub raw_bytes: _rt::Vec<u8>,
            }
            impl ::core::fmt::Debug for EvmAddress {
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    f.debug_struct("EvmAddress").field("raw-bytes", &self.raw_bytes).finish()
                }
            }
            #[derive(Clone)]
            pub struct EvmEventLogData {
                /// the raw log topics that can be decoded into an event
                pub topics: _rt::Vec<_rt::Vec<u8>>,
                /// the raw log data that can be decoded into an event
                pub data: _rt::Vec<u8>,
            }
            impl ::core::fmt::Debug for EvmEventLogData {
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    f.debug_struct("EvmEventLogData")
                        .field("topics", &self.topics)
                        .field("data", &self.data)
                        .finish()
                }
            }
            #[derive(Clone)]
            pub struct EvmChainConfig {
                pub chain_id: _rt::String,
                pub ws_endpoint: Option<_rt::String>,
                pub http_endpoint: Option<_rt::String>,
            }
            impl ::core::fmt::Debug for EvmChainConfig {
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    f.debug_struct("EvmChainConfig")
                        .field("chain-id", &self.chain_id)
                        .field("ws-endpoint", &self.ws_endpoint)
                        .field("http-endpoint", &self.http_endpoint)
                        .finish()
                }
            }
            #[derive(Clone)]
            pub struct TriggerSourceEvmContractEvent {
                pub address: EvmAddress,
                pub chain_name: _rt::String,
                pub event_hash: _rt::Vec<u8>,
            }
            impl ::core::fmt::Debug for TriggerSourceEvmContractEvent {
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    f.debug_struct("TriggerSourceEvmContractEvent")
                        .field("address", &self.address)
                        .field("chain-name", &self.chain_name)
                        .field("event-hash", &self.event_hash)
                        .finish()
                }
            }
            #[derive(Clone)]
            pub struct TriggerSourceCosmosContractEvent {
                pub address: CosmosAddress,
                pub chain_name: _rt::String,
                pub event_type: _rt::String,
            }
            impl ::core::fmt::Debug for TriggerSourceCosmosContractEvent {
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    f.debug_struct("TriggerSourceCosmosContractEvent")
                        .field("address", &self.address)
                        .field("chain-name", &self.chain_name)
                        .field("event-type", &self.event_type)
                        .finish()
                }
            }
            #[derive(Clone)]
            pub struct BlockIntervalSource {
                pub chain_name: _rt::String,
                pub n_blocks: u32,
                pub start_block: Option<u64>,
                pub end_block: Option<u64>,
            }
            impl ::core::fmt::Debug for BlockIntervalSource {
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    f.debug_struct("BlockIntervalSource")
                        .field("chain-name", &self.chain_name)
                        .field("n-blocks", &self.n_blocks)
                        .field("start-block", &self.start_block)
                        .field("end-block", &self.end_block)
                        .finish()
                }
            }
            #[derive(Clone)]
            pub struct TriggerSourceCron {
                pub schedule: _rt::String,
                pub start_time: Option<Timestamp>,
                pub end_time: Option<Timestamp>,
            }
            impl ::core::fmt::Debug for TriggerSourceCron {
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    f.debug_struct("TriggerSourceCron")
                        .field("schedule", &self.schedule)
                        .field("start-time", &self.start_time)
                        .field("end-time", &self.end_time)
                        .finish()
                }
            }
            #[derive(Clone)]
            pub enum TriggerSource {
                EvmContractEvent(TriggerSourceEvmContractEvent),
                CosmosContractEvent(TriggerSourceCosmosContractEvent),
                BlockInterval(BlockIntervalSource),
                Cron(TriggerSourceCron),
                Manual,
            }
            impl ::core::fmt::Debug for TriggerSource {
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    match self {
                        TriggerSource::EvmContractEvent(e) => {
                            f.debug_tuple("TriggerSource::EvmContractEvent").field(e).finish()
                        }
                        TriggerSource::CosmosContractEvent(e) => {
                            f.debug_tuple("TriggerSource::CosmosContractEvent").field(e).finish()
                        }
                        TriggerSource::BlockInterval(e) => {
                            f.debug_tuple("TriggerSource::BlockInterval").field(e).finish()
                        }
                        TriggerSource::Cron(e) => {
                            f.debug_tuple("TriggerSource::Cron").field(e).finish()
                        }
                        TriggerSource::Manual => f.debug_tuple("TriggerSource::Manual").finish(),
                    }
                }
            }
            #[derive(Clone)]
            pub struct TriggerConfig {
                pub service_id: _rt::String,
                pub workflow_id: _rt::String,
                pub trigger_source: TriggerSource,
            }
            impl ::core::fmt::Debug for TriggerConfig {
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    f.debug_struct("TriggerConfig")
                        .field("service-id", &self.service_id)
                        .field("workflow-id", &self.workflow_id)
                        .field("trigger-source", &self.trigger_source)
                        .finish()
                }
            }
            #[derive(Clone)]
            pub struct TriggerDataEvmContractEvent {
                pub contract_address: EvmAddress,
                pub chain_name: _rt::String,
                pub log: EvmEventLogData,
                pub block_height: u64,
            }
            impl ::core::fmt::Debug for TriggerDataEvmContractEvent {
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    f.debug_struct("TriggerDataEvmContractEvent")
                        .field("contract-address", &self.contract_address)
                        .field("chain-name", &self.chain_name)
                        .field("log", &self.log)
                        .field("block-height", &self.block_height)
                        .finish()
                }
            }
            #[derive(Clone)]
            pub struct TriggerDataCosmosContractEvent {
                pub contract_address: CosmosAddress,
                pub chain_name: _rt::String,
                pub event: CosmosEvent,
                pub block_height: u64,
            }
            impl ::core::fmt::Debug for TriggerDataCosmosContractEvent {
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    f.debug_struct("TriggerDataCosmosContractEvent")
                        .field("contract-address", &self.contract_address)
                        .field("chain-name", &self.chain_name)
                        .field("event", &self.event)
                        .field("block-height", &self.block_height)
                        .finish()
                }
            }
            #[derive(Clone)]
            pub struct BlockIntervalData {
                pub chain_name: _rt::String,
                pub block_height: u64,
            }
            impl ::core::fmt::Debug for BlockIntervalData {
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    f.debug_struct("BlockIntervalData")
                        .field("chain-name", &self.chain_name)
                        .field("block-height", &self.block_height)
                        .finish()
                }
            }
            #[repr(C)]
            #[derive(Clone, Copy)]
            pub struct TriggerDataCron {
                pub trigger_time: Timestamp,
            }
            impl ::core::fmt::Debug for TriggerDataCron {
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    f.debug_struct("TriggerDataCron")
                        .field("trigger-time", &self.trigger_time)
                        .finish()
                }
            }
            #[derive(Clone)]
            pub enum TriggerData {
                EvmContractEvent(TriggerDataEvmContractEvent),
                CosmosContractEvent(TriggerDataCosmosContractEvent),
                BlockInterval(BlockIntervalData),
                Cron(TriggerDataCron),
                Raw(_rt::Vec<u8>),
            }
            impl ::core::fmt::Debug for TriggerData {
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    match self {
                        TriggerData::EvmContractEvent(e) => {
                            f.debug_tuple("TriggerData::EvmContractEvent").field(e).finish()
                        }
                        TriggerData::CosmosContractEvent(e) => {
                            f.debug_tuple("TriggerData::CosmosContractEvent").field(e).finish()
                        }
                        TriggerData::BlockInterval(e) => {
                            f.debug_tuple("TriggerData::BlockInterval").field(e).finish()
                        }
                        TriggerData::Cron(e) => {
                            f.debug_tuple("TriggerData::Cron").field(e).finish()
                        }
                        TriggerData::Raw(e) => f.debug_tuple("TriggerData::Raw").field(e).finish(),
                    }
                }
            }
            #[derive(Clone)]
            pub struct TriggerAction {
                pub config: TriggerConfig,
                pub data: TriggerData,
            }
            impl ::core::fmt::Debug for TriggerAction {
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    f.debug_struct("TriggerAction")
                        .field("config", &self.config)
                        .field("data", &self.data)
                        .finish()
                }
            }
            #[derive(Clone)]
            pub struct WasmResponse {
                pub payload: _rt::Vec<u8>,
                pub ordering: Option<u64>,
            }
            impl ::core::fmt::Debug for WasmResponse {
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    f.debug_struct("WasmResponse")
                        .field("payload", &self.payload)
                        .field("ordering", &self.ordering)
                        .finish()
                }
            }
            #[derive(Clone, Copy)]
            pub enum LogLevel {
                Error,
                Warn,
                Info,
                Debug,
                Trace,
            }
            impl ::core::fmt::Debug for LogLevel {
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    match self {
                        LogLevel::Error => f.debug_tuple("LogLevel::Error").finish(),
                        LogLevel::Warn => f.debug_tuple("LogLevel::Warn").finish(),
                        LogLevel::Info => f.debug_tuple("LogLevel::Info").finish(),
                        LogLevel::Debug => f.debug_tuple("LogLevel::Debug").finish(),
                        LogLevel::Trace => f.debug_tuple("LogLevel::Trace").finish(),
                    }
                }
            }
        }
    }
}
#[allow(dead_code, clippy::all)]
pub mod host {
    #[used]
    #[doc(hidden)]
    static __FORCE_SECTION_REF: fn() = super::__link_custom_section_describing_imports;
    use super::_rt;
    pub type EvmChainConfig = super::wavs::worker::layer_types::EvmChainConfig;
    pub type CosmosChainConfig = super::wavs::worker::layer_types::CosmosChainConfig;
    pub type LogLevel = super::wavs::worker::layer_types::LogLevel;
    #[allow(unused_unsafe, clippy::all)]
    pub fn get_evm_chain_config(chain_name: &str) -> Option<EvmChainConfig> {
        unsafe {
            #[repr(align(4))]
            struct RetArea([::core::mem::MaybeUninit<u8>; 36]);
            let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 36]);
            let vec0 = chain_name;
            let ptr0 = vec0.as_ptr().cast::<u8>();
            let len0 = vec0.len();
            let ptr1 = ret_area.0.as_mut_ptr().cast::<u8>();
            #[cfg(target_arch = "wasm32")]
            #[link(wasm_import_module = "host")]
            extern "C" {
                #[link_name = "get-evm-chain-config"]
                fn wit_import(_: *mut u8, _: usize, _: *mut u8);
            }
            #[cfg(not(target_arch = "wasm32"))]
            fn wit_import(_: *mut u8, _: usize, _: *mut u8) {
                unreachable!()
            }
            wit_import(ptr0.cast_mut(), len0, ptr1);
            let l2 = i32::from(*ptr1.add(0).cast::<u8>());
            match l2 {
                0 => None,
                1 => {
                    let e = {
                        let l3 = *ptr1.add(4).cast::<*mut u8>();
                        let l4 = *ptr1.add(8).cast::<usize>();
                        let len5 = l4;
                        let bytes5 = _rt::Vec::from_raw_parts(l3.cast(), len5, len5);
                        let l6 = i32::from(*ptr1.add(12).cast::<u8>());
                        let l10 = i32::from(*ptr1.add(24).cast::<u8>());
                        super::wavs::worker::layer_types::EvmChainConfig {
                            chain_id: _rt::string_lift(bytes5),
                            ws_endpoint: match l6 {
                                0 => None,
                                1 => {
                                    let e = {
                                        let l7 = *ptr1.add(16).cast::<*mut u8>();
                                        let l8 = *ptr1.add(20).cast::<usize>();
                                        let len9 = l8;
                                        let bytes9 =
                                            _rt::Vec::from_raw_parts(l7.cast(), len9, len9);
                                        _rt::string_lift(bytes9)
                                    };
                                    Some(e)
                                }
                                _ => _rt::invalid_enum_discriminant(),
                            },
                            http_endpoint: match l10 {
                                0 => None,
                                1 => {
                                    let e = {
                                        let l11 = *ptr1.add(28).cast::<*mut u8>();
                                        let l12 = *ptr1.add(32).cast::<usize>();
                                        let len13 = l12;
                                        let bytes13 =
                                            _rt::Vec::from_raw_parts(l11.cast(), len13, len13);
                                        _rt::string_lift(bytes13)
                                    };
                                    Some(e)
                                }
                                _ => _rt::invalid_enum_discriminant(),
                            },
                        }
                    };
                    Some(e)
                }
                _ => _rt::invalid_enum_discriminant(),
            }
        }
    }
    #[allow(unused_unsafe, clippy::all)]
    pub fn get_cosmos_chain_config(chain_name: &str) -> Option<CosmosChainConfig> {
        unsafe {
            #[repr(align(4))]
            struct RetArea([::core::mem::MaybeUninit<u8>; 68]);
            let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 68]);
            let vec0 = chain_name;
            let ptr0 = vec0.as_ptr().cast::<u8>();
            let len0 = vec0.len();
            let ptr1 = ret_area.0.as_mut_ptr().cast::<u8>();
            #[cfg(target_arch = "wasm32")]
            #[link(wasm_import_module = "host")]
            extern "C" {
                #[link_name = "get-cosmos-chain-config"]
                fn wit_import(_: *mut u8, _: usize, _: *mut u8);
            }
            #[cfg(not(target_arch = "wasm32"))]
            fn wit_import(_: *mut u8, _: usize, _: *mut u8) {
                unreachable!()
            }
            wit_import(ptr0.cast_mut(), len0, ptr1);
            let l2 = i32::from(*ptr1.add(0).cast::<u8>());
            match l2 {
                0 => None,
                1 => {
                    let e = {
                        let l3 = *ptr1.add(4).cast::<*mut u8>();
                        let l4 = *ptr1.add(8).cast::<usize>();
                        let len5 = l4;
                        let bytes5 = _rt::Vec::from_raw_parts(l3.cast(), len5, len5);
                        let l6 = i32::from(*ptr1.add(12).cast::<u8>());
                        let l10 = i32::from(*ptr1.add(24).cast::<u8>());
                        let l14 = i32::from(*ptr1.add(36).cast::<u8>());
                        let l18 = *ptr1.add(48).cast::<f32>();
                        let l19 = *ptr1.add(52).cast::<*mut u8>();
                        let l20 = *ptr1.add(56).cast::<usize>();
                        let len21 = l20;
                        let bytes21 = _rt::Vec::from_raw_parts(l19.cast(), len21, len21);
                        let l22 = *ptr1.add(60).cast::<*mut u8>();
                        let l23 = *ptr1.add(64).cast::<usize>();
                        let len24 = l23;
                        let bytes24 = _rt::Vec::from_raw_parts(l22.cast(), len24, len24);
                        super::wavs::worker::layer_types::CosmosChainConfig {
                            chain_id: _rt::string_lift(bytes5),
                            rpc_endpoint: match l6 {
                                0 => None,
                                1 => {
                                    let e = {
                                        let l7 = *ptr1.add(16).cast::<*mut u8>();
                                        let l8 = *ptr1.add(20).cast::<usize>();
                                        let len9 = l8;
                                        let bytes9 =
                                            _rt::Vec::from_raw_parts(l7.cast(), len9, len9);
                                        _rt::string_lift(bytes9)
                                    };
                                    Some(e)
                                }
                                _ => _rt::invalid_enum_discriminant(),
                            },
                            grpc_endpoint: match l10 {
                                0 => None,
                                1 => {
                                    let e = {
                                        let l11 = *ptr1.add(28).cast::<*mut u8>();
                                        let l12 = *ptr1.add(32).cast::<usize>();
                                        let len13 = l12;
                                        let bytes13 =
                                            _rt::Vec::from_raw_parts(l11.cast(), len13, len13);
                                        _rt::string_lift(bytes13)
                                    };
                                    Some(e)
                                }
                                _ => _rt::invalid_enum_discriminant(),
                            },
                            grpc_web_endpoint: match l14 {
                                0 => None,
                                1 => {
                                    let e = {
                                        let l15 = *ptr1.add(40).cast::<*mut u8>();
                                        let l16 = *ptr1.add(44).cast::<usize>();
                                        let len17 = l16;
                                        let bytes17 =
                                            _rt::Vec::from_raw_parts(l15.cast(), len17, len17);
                                        _rt::string_lift(bytes17)
                                    };
                                    Some(e)
                                }
                                _ => _rt::invalid_enum_discriminant(),
                            },
                            gas_price: l18,
                            gas_denom: _rt::string_lift(bytes21),
                            bech32_prefix: _rt::string_lift(bytes24),
                        }
                    };
                    Some(e)
                }
                _ => _rt::invalid_enum_discriminant(),
            }
        }
    }
    #[allow(unused_unsafe, clippy::all)]
    pub fn config_var(key: &str) -> Option<_rt::String> {
        unsafe {
            #[repr(align(4))]
            struct RetArea([::core::mem::MaybeUninit<u8>; 12]);
            let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 12]);
            let vec0 = key;
            let ptr0 = vec0.as_ptr().cast::<u8>();
            let len0 = vec0.len();
            let ptr1 = ret_area.0.as_mut_ptr().cast::<u8>();
            #[cfg(target_arch = "wasm32")]
            #[link(wasm_import_module = "host")]
            extern "C" {
                #[link_name = "config-var"]
                fn wit_import(_: *mut u8, _: usize, _: *mut u8);
            }
            #[cfg(not(target_arch = "wasm32"))]
            fn wit_import(_: *mut u8, _: usize, _: *mut u8) {
                unreachable!()
            }
            wit_import(ptr0.cast_mut(), len0, ptr1);
            let l2 = i32::from(*ptr1.add(0).cast::<u8>());
            match l2 {
                0 => None,
                1 => {
                    let e = {
                        let l3 = *ptr1.add(4).cast::<*mut u8>();
                        let l4 = *ptr1.add(8).cast::<usize>();
                        let len5 = l4;
                        let bytes5 = _rt::Vec::from_raw_parts(l3.cast(), len5, len5);
                        _rt::string_lift(bytes5)
                    };
                    Some(e)
                }
                _ => _rt::invalid_enum_discriminant(),
            }
        }
    }
    #[allow(unused_unsafe, clippy::all)]
    pub fn log(level: LogLevel, message: &str) {
        unsafe {
            use super::wavs::worker::layer_types::LogLevel as V0;
            let result1 = match level {
                V0::Error => 0i32,
                V0::Warn => 1i32,
                V0::Info => 2i32,
                V0::Debug => 3i32,
                V0::Trace => 4i32,
            };
            let vec2 = message;
            let ptr2 = vec2.as_ptr().cast::<u8>();
            let len2 = vec2.len();
            #[cfg(target_arch = "wasm32")]
            #[link(wasm_import_module = "host")]
            extern "C" {
                #[link_name = "log"]
                fn wit_import(_: i32, _: *mut u8, _: usize);
            }
            #[cfg(not(target_arch = "wasm32"))]
            fn wit_import(_: i32, _: *mut u8, _: usize) {
                unreachable!()
            }
            wit_import(result1, ptr2.cast_mut(), len2);
        }
    }
}
mod _rt {
    pub use alloc_crate::string::String;
    pub use alloc_crate::vec::Vec;
    use core::fmt;
    use core::marker;
    use core::sync::atomic::{AtomicU32, Ordering::Relaxed};
    /// A type which represents a component model resource, either imported or
    /// exported into this component.
    ///
    /// This is a low-level wrapper which handles the lifetime of the resource
    /// (namely this has a destructor). The `T` provided defines the component model
    /// intrinsics that this wrapper uses.
    ///
    /// One of the chief purposes of this type is to provide `Deref` implementations
    /// to access the underlying data when it is owned.
    ///
    /// This type is primarily used in generated code for exported and imported
    /// resources.
    #[repr(transparent)]
    pub struct Resource<T: WasmResource> {
        handle: AtomicU32,
        _marker: marker::PhantomData<T>,
    }
    /// A trait which all wasm resources implement, namely providing the ability to
    /// drop a resource.
    ///
    /// This generally is implemented by generated code, not user-facing code.
    #[allow(clippy::missing_safety_doc)]
    pub unsafe trait WasmResource {
        /// Invokes the `[resource-drop]...` intrinsic.
        unsafe fn drop(handle: u32);
    }
    impl<T: WasmResource> Resource<T> {
        #[doc(hidden)]
        pub unsafe fn from_handle(handle: u32) -> Self {
            debug_assert!(handle != u32::MAX);
            Self { handle: AtomicU32::new(handle), _marker: marker::PhantomData }
        }
        /// Takes ownership of the handle owned by `resource`.
        ///
        /// Note that this ideally would be `into_handle` taking `Resource<T>` by
        /// ownership. The code generator does not enable that in all situations,
        /// unfortunately, so this is provided instead.
        ///
        /// Also note that `take_handle` is in theory only ever called on values
        /// owned by a generated function. For example a generated function might
        /// take `Resource<T>` as an argument but then call `take_handle` on a
        /// reference to that argument. In that sense the dynamic nature of
        /// `take_handle` should only be exposed internally to generated code, not
        /// to user code.
        #[doc(hidden)]
        pub fn take_handle(resource: &Resource<T>) -> u32 {
            resource.handle.swap(u32::MAX, Relaxed)
        }
        #[doc(hidden)]
        pub fn handle(resource: &Resource<T>) -> u32 {
            resource.handle.load(Relaxed)
        }
    }
    impl<T: WasmResource> fmt::Debug for Resource<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("Resource").field("handle", &self.handle).finish()
        }
    }
    impl<T: WasmResource> Drop for Resource<T> {
        fn drop(&mut self) {
            unsafe {
                match self.handle.load(Relaxed) {
                    u32::MAX => {}
                    other => T::drop(other),
                }
            }
        }
    }
    pub unsafe fn bool_lift(val: u8) -> bool {
        if cfg!(debug_assertions) {
            match val {
                0 => false,
                1 => true,
                _ => panic!("invalid bool discriminant"),
            }
        } else {
            val != 0
        }
    }
    pub use alloc_crate::alloc;
    pub fn as_i64<T: AsI64>(t: T) -> i64 {
        t.as_i64()
    }
    pub trait AsI64 {
        fn as_i64(self) -> i64;
    }
    impl<'a, T: Copy + AsI64> AsI64 for &'a T {
        fn as_i64(self) -> i64 {
            (*self).as_i64()
        }
    }
    impl AsI64 for i64 {
        #[inline]
        fn as_i64(self) -> i64 {
            self as i64
        }
    }
    impl AsI64 for u64 {
        #[inline]
        fn as_i64(self) -> i64 {
            self as i64
        }
    }
    pub unsafe fn string_lift(bytes: Vec<u8>) -> String {
        if cfg!(debug_assertions) {
            String::from_utf8(bytes).unwrap()
        } else {
            String::from_utf8_unchecked(bytes)
        }
    }
    pub unsafe fn invalid_enum_discriminant<T>() -> T {
        if cfg!(debug_assertions) {
            panic!("invalid enum discriminant")
        } else {
            core::hint::unreachable_unchecked()
        }
    }
    pub unsafe fn cabi_dealloc(ptr: *mut u8, size: usize, align: usize) {
        if size == 0 {
            return;
        }
        let layout = alloc::Layout::from_size_align_unchecked(size, align);
        alloc::dealloc(ptr, layout);
    }
    pub fn as_i32<T: AsI32>(t: T) -> i32 {
        t.as_i32()
    }
    pub trait AsI32 {
        fn as_i32(self) -> i32;
    }
    impl<'a, T: Copy + AsI32> AsI32 for &'a T {
        fn as_i32(self) -> i32 {
            (*self).as_i32()
        }
    }
    impl AsI32 for i32 {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    impl AsI32 for u32 {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    impl AsI32 for i16 {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    impl AsI32 for u16 {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    impl AsI32 for i8 {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    impl AsI32 for u8 {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    impl AsI32 for char {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    impl AsI32 for usize {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    #[cfg(target_arch = "wasm32")]
    pub fn run_ctors_once() {
        wit_bindgen_rt::run_ctors_once();
    }
    extern crate alloc as alloc_crate;
}
/// Generates `#[no_mangle]` functions to export the specified type as the
/// root implementation of all generated traits.
///
/// For more information see the documentation of `wit_bindgen::generate!`.
///
/// ```rust
/// # macro_rules! export{ ($($t:tt)*) => (); }
/// # trait Guest {}
/// struct MyType;
///
/// impl Guest for MyType {
///     // ...
/// }
///
/// export!(MyType);
/// ```
#[allow(unused_macros)]
#[doc(hidden)]
macro_rules! __export_layer_trigger_world_impl {
    ($ty:ident) => {
        self::export!($ty with_types_in self);
    };
    ($ty:ident with_types_in $($path_to_types_root:tt)*) => {
        $($path_to_types_root)*:: __export_world_layer_trigger_world_cabi!($ty
        with_types_in $($path_to_types_root)*);
    };
}
#[doc(inline)]
pub(crate) use __export_layer_trigger_world_impl as export;
#[cfg(target_arch = "wasm32")]
#[link_section = "component-type:wit-bindgen:0.35.0:wavs:worker@0.4.0:layer-trigger-world:encoded world"]
#[doc(hidden)]
pub static __WIT_BINDGEN_COMPONENT_TYPE: [u8; 17448] = *b"\
\0asm\x0d\0\x01\0\0\x19\x16wit-component-encoding\x04\0\x07\x9d\x87\x01\x01A\x02\
\x01A^\x01B1\x01r\x01\x05nanosw\x04\0\x09timestamp\x03\0\0\x01r\x02\x0bbech32-ad\
drs\x0aprefix-leny\x04\0\x0ecosmos-address\x03\0\x02\x01o\x02ss\x01p\x04\x01r\x02\
\x02tys\x0aattributes\x05\x04\0\x0ccosmos-event\x03\0\x06\x01ks\x01r\x07\x08chai\
n-ids\x0crpc-endpoint\x08\x0dgrpc-endpoint\x08\x11grpc-web-endpoint\x08\x09gas-p\
ricev\x09gas-denoms\x0dbech32-prefixs\x04\0\x13cosmos-chain-config\x03\0\x09\x01\
p}\x01r\x01\x09raw-bytes\x0b\x04\0\x0bevm-address\x03\0\x0c\x01p\x0b\x01r\x02\x06\
topics\x0e\x04data\x0b\x04\0\x12evm-event-log-data\x03\0\x0f\x01r\x03\x08chain-i\
ds\x0bws-endpoint\x08\x0dhttp-endpoint\x08\x04\0\x10evm-chain-config\x03\0\x11\x01\
r\x03\x07address\x0d\x0achain-names\x0aevent-hash\x0b\x04\0!trigger-source-evm-c\
ontract-event\x03\0\x13\x01r\x03\x07address\x03\x0achain-names\x0aevent-types\x04\
\0$trigger-source-cosmos-contract-event\x03\0\x15\x01kw\x01r\x04\x0achain-names\x08\
n-blocksy\x0bstart-block\x17\x09end-block\x17\x04\0\x15block-interval-source\x03\
\0\x18\x01k\x01\x01r\x03\x08schedules\x0astart-time\x1a\x08end-time\x1a\x04\0\x13\
trigger-source-cron\x03\0\x1b\x01q\x05\x12evm-contract-event\x01\x14\0\x15cosmos\
-contract-event\x01\x16\0\x0eblock-interval\x01\x19\0\x04cron\x01\x1c\0\x06manua\
l\0\0\x04\0\x0etrigger-source\x03\0\x1d\x01r\x03\x0aservice-ids\x0bworkflow-ids\x0e\
trigger-source\x1e\x04\0\x0etrigger-config\x03\0\x1f\x01r\x04\x10contract-addres\
s\x0d\x0achain-names\x03log\x10\x0cblock-heightw\x04\0\x1ftrigger-data-evm-contr\
act-event\x03\0!\x01r\x04\x10contract-address\x03\x0achain-names\x05event\x07\x0c\
block-heightw\x04\0\"trigger-data-cosmos-contract-event\x03\0#\x01r\x02\x0achain\
-names\x0cblock-heightw\x04\0\x13block-interval-data\x03\0%\x01r\x01\x0ctrigger-\
time\x01\x04\0\x11trigger-data-cron\x03\0'\x01q\x05\x12evm-contract-event\x01\"\0\
\x15cosmos-contract-event\x01$\0\x0eblock-interval\x01&\0\x04cron\x01(\0\x03raw\x01\
\x0b\0\x04\0\x0ctrigger-data\x03\0)\x01r\x02\x06config\x20\x04data*\x04\0\x0etri\
gger-action\x03\0+\x01r\x02\x07payload\x0b\x08ordering\x17\x04\0\x0dwasm-respons\
e\x03\0-\x01q\x05\x05error\0\0\x04warn\0\0\x04info\0\0\x05debug\0\0\x05trace\0\0\
\x04\0\x09log-level\x03\0/\x03\0\x1dwavs:worker/layer-types@0.4.0\x05\0\x02\x03\0\
\0\x0etrigger-action\x03\0\x0etrigger-action\x03\0\x01\x02\x03\0\0\x0dwasm-respo\
nse\x03\0\x0dwasm-response\x03\0\x03\x01B\x0a\x04\0\x08pollable\x03\x01\x01h\0\x01\
@\x01\x04self\x01\0\x7f\x04\0\x16[method]pollable.ready\x01\x02\x01@\x01\x04self\
\x01\x01\0\x04\0\x16[method]pollable.block\x01\x03\x01p\x01\x01py\x01@\x01\x02in\
\x04\0\x05\x04\0\x04poll\x01\x06\x03\0\x12wasi:io/poll@0.2.0\x05\x05\x02\x03\0\x01\
\x08pollable\x01B\x0f\x02\x03\x02\x01\x06\x04\0\x08pollable\x03\0\0\x01w\x04\0\x07\
instant\x03\0\x02\x01w\x04\0\x08duration\x03\0\x04\x01@\0\0\x03\x04\0\x03now\x01\
\x06\x01@\0\0\x05\x04\0\x0aresolution\x01\x07\x01i\x01\x01@\x01\x04when\x03\0\x08\
\x04\0\x11subscribe-instant\x01\x09\x01@\x01\x04when\x05\0\x08\x04\0\x12subscrib\
e-duration\x01\x0a\x03\0!wasi:clocks/monotonic-clock@0.2.0\x05\x07\x01B\x04\x04\0\
\x05error\x03\x01\x01h\0\x01@\x01\x04self\x01\0s\x04\0\x1d[method]error.to-debug\
-string\x01\x02\x03\0\x13wasi:io/error@0.2.0\x05\x08\x02\x03\0\x03\x05error\x01B\
(\x02\x03\x02\x01\x09\x04\0\x05error\x03\0\0\x02\x03\x02\x01\x06\x04\0\x08pollab\
le\x03\0\x02\x01i\x01\x01q\x02\x15last-operation-failed\x01\x04\0\x06closed\0\0\x04\
\0\x0cstream-error\x03\0\x05\x04\0\x0cinput-stream\x03\x01\x04\0\x0doutput-strea\
m\x03\x01\x01h\x07\x01p}\x01j\x01\x0a\x01\x06\x01@\x02\x04self\x09\x03lenw\0\x0b\
\x04\0\x19[method]input-stream.read\x01\x0c\x04\0\"[method]input-stream.blocking\
-read\x01\x0c\x01j\x01w\x01\x06\x01@\x02\x04self\x09\x03lenw\0\x0d\x04\0\x19[met\
hod]input-stream.skip\x01\x0e\x04\0\"[method]input-stream.blocking-skip\x01\x0e\x01\
i\x03\x01@\x01\x04self\x09\0\x0f\x04\0\x1e[method]input-stream.subscribe\x01\x10\
\x01h\x08\x01@\x01\x04self\x11\0\x0d\x04\0![method]output-stream.check-write\x01\
\x12\x01j\0\x01\x06\x01@\x02\x04self\x11\x08contents\x0a\0\x13\x04\0\x1b[method]\
output-stream.write\x01\x14\x04\0.[method]output-stream.blocking-write-and-flush\
\x01\x14\x01@\x01\x04self\x11\0\x13\x04\0\x1b[method]output-stream.flush\x01\x15\
\x04\0$[method]output-stream.blocking-flush\x01\x15\x01@\x01\x04self\x11\0\x0f\x04\
\0\x1f[method]output-stream.subscribe\x01\x16\x01@\x02\x04self\x11\x03lenw\0\x13\
\x04\0\"[method]output-stream.write-zeroes\x01\x17\x04\05[method]output-stream.b\
locking-write-zeroes-and-flush\x01\x17\x01@\x03\x04self\x11\x03src\x09\x03lenw\0\
\x0d\x04\0\x1c[method]output-stream.splice\x01\x18\x04\0%[method]output-stream.b\
locking-splice\x01\x18\x03\0\x15wasi:io/streams@0.2.0\x05\x0a\x02\x03\0\x02\x08d\
uration\x02\x03\0\x04\x0cinput-stream\x02\x03\0\x04\x0doutput-stream\x01B\xc0\x01\
\x02\x03\x02\x01\x0b\x04\0\x08duration\x03\0\0\x02\x03\x02\x01\x0c\x04\0\x0cinpu\
t-stream\x03\0\x02\x02\x03\x02\x01\x0d\x04\0\x0doutput-stream\x03\0\x04\x02\x03\x02\
\x01\x09\x04\0\x08io-error\x03\0\x06\x02\x03\x02\x01\x06\x04\0\x08pollable\x03\0\
\x08\x01q\x0a\x03get\0\0\x04head\0\0\x04post\0\0\x03put\0\0\x06delete\0\0\x07con\
nect\0\0\x07options\0\0\x05trace\0\0\x05patch\0\0\x05other\x01s\0\x04\0\x06metho\
d\x03\0\x0a\x01q\x03\x04HTTP\0\0\x05HTTPS\0\0\x05other\x01s\0\x04\0\x06scheme\x03\
\0\x0c\x01ks\x01k{\x01r\x02\x05rcode\x0e\x09info-code\x0f\x04\0\x11DNS-error-pay\
load\x03\0\x10\x01k}\x01r\x02\x08alert-id\x12\x0dalert-message\x0e\x04\0\x1aTLS-\
alert-received-payload\x03\0\x13\x01ky\x01r\x02\x0afield-name\x0e\x0afield-size\x15\
\x04\0\x12field-size-payload\x03\0\x16\x01kw\x01k\x17\x01q'\x0bDNS-timeout\0\0\x09\
DNS-error\x01\x11\0\x15destination-not-found\0\0\x17destination-unavailable\0\0\x19\
destination-IP-prohibited\0\0\x19destination-IP-unroutable\0\0\x12connection-ref\
used\0\0\x15connection-terminated\0\0\x12connection-timeout\0\0\x17connection-re\
ad-timeout\0\0\x18connection-write-timeout\0\0\x18connection-limit-reached\0\0\x12\
TLS-protocol-error\0\0\x15TLS-certificate-error\0\0\x12TLS-alert-received\x01\x14\
\0\x13HTTP-request-denied\0\0\x1cHTTP-request-length-required\0\0\x16HTTP-reques\
t-body-size\x01\x18\0\x1bHTTP-request-method-invalid\0\0\x18HTTP-request-URI-inv\
alid\0\0\x19HTTP-request-URI-too-long\0\0\x20HTTP-request-header-section-size\x01\
\x15\0\x18HTTP-request-header-size\x01\x19\0!HTTP-request-trailer-section-size\x01\
\x15\0\x19HTTP-request-trailer-size\x01\x17\0\x18HTTP-response-incomplete\0\0!HT\
TP-response-header-section-size\x01\x15\0\x19HTTP-response-header-size\x01\x17\0\
\x17HTTP-response-body-size\x01\x18\0\"HTTP-response-trailer-section-size\x01\x15\
\0\x1aHTTP-response-trailer-size\x01\x17\0\x1dHTTP-response-transfer-coding\x01\x0e\
\0\x1cHTTP-response-content-coding\x01\x0e\0\x15HTTP-response-timeout\0\0\x13HTT\
P-upgrade-failed\0\0\x13HTTP-protocol-error\0\0\x0dloop-detected\0\0\x13configur\
ation-error\0\0\x0einternal-error\x01\x0e\0\x04\0\x0aerror-code\x03\0\x1a\x01q\x03\
\x0einvalid-syntax\0\0\x09forbidden\0\0\x09immutable\0\0\x04\0\x0cheader-error\x03\
\0\x1c\x01s\x04\0\x09field-key\x03\0\x1e\x01p}\x04\0\x0bfield-value\x03\0\x20\x04\
\0\x06fields\x03\x01\x04\0\x07headers\x03\0\"\x04\0\x08trailers\x03\0\"\x04\0\x10\
incoming-request\x03\x01\x04\0\x10outgoing-request\x03\x01\x04\0\x0frequest-opti\
ons\x03\x01\x04\0\x11response-outparam\x03\x01\x01{\x04\0\x0bstatus-code\x03\0)\x04\
\0\x11incoming-response\x03\x01\x04\0\x0dincoming-body\x03\x01\x04\0\x0ffuture-t\
railers\x03\x01\x04\0\x11outgoing-response\x03\x01\x04\0\x0doutgoing-body\x03\x01\
\x04\0\x18future-incoming-response\x03\x01\x01i\"\x01@\0\01\x04\0\x13[constructo\
r]fields\x012\x01o\x02\x1f!\x01p3\x01j\x011\x01\x1d\x01@\x01\x07entries4\05\x04\0\
\x18[static]fields.from-list\x016\x01h\"\x01p!\x01@\x02\x04self7\x04name\x1f\08\x04\
\0\x12[method]fields.get\x019\x01@\x02\x04self7\x04name\x1f\0\x7f\x04\0\x12[meth\
od]fields.has\x01:\x01j\0\x01\x1d\x01@\x03\x04self7\x04name\x1f\x05value8\0;\x04\
\0\x12[method]fields.set\x01<\x01@\x02\x04self7\x04name\x1f\0;\x04\0\x15[method]\
fields.delete\x01=\x01@\x03\x04self7\x04name\x1f\x05value!\0;\x04\0\x15[method]f\
ields.append\x01>\x01@\x01\x04self7\04\x04\0\x16[method]fields.entries\x01?\x01@\
\x01\x04self7\01\x04\0\x14[method]fields.clone\x01@\x01h%\x01@\x01\x04self\xc1\0\
\0\x0b\x04\0\x1f[method]incoming-request.method\x01B\x01@\x01\x04self\xc1\0\0\x0e\
\x04\0([method]incoming-request.path-with-query\x01C\x01k\x0d\x01@\x01\x04self\xc1\
\0\0\xc4\0\x04\0\x1f[method]incoming-request.scheme\x01E\x04\0\"[method]incoming\
-request.authority\x01C\x01i#\x01@\x01\x04self\xc1\0\0\xc6\0\x04\0\x20[method]in\
coming-request.headers\x01G\x01i,\x01j\x01\xc8\0\0\x01@\x01\x04self\xc1\0\0\xc9\0\
\x04\0\x20[method]incoming-request.consume\x01J\x01i&\x01@\x01\x07headers\xc6\0\0\
\xcb\0\x04\0\x1d[constructor]outgoing-request\x01L\x01h&\x01i/\x01j\x01\xce\0\0\x01\
@\x01\x04self\xcd\0\0\xcf\0\x04\0\x1d[method]outgoing-request.body\x01P\x01@\x01\
\x04self\xcd\0\0\x0b\x04\0\x1f[method]outgoing-request.method\x01Q\x01j\0\0\x01@\
\x02\x04self\xcd\0\x06method\x0b\0\xd2\0\x04\0#[method]outgoing-request.set-meth\
od\x01S\x01@\x01\x04self\xcd\0\0\x0e\x04\0([method]outgoing-request.path-with-qu\
ery\x01T\x01@\x02\x04self\xcd\0\x0fpath-with-query\x0e\0\xd2\0\x04\0,[method]out\
going-request.set-path-with-query\x01U\x01@\x01\x04self\xcd\0\0\xc4\0\x04\0\x1f[\
method]outgoing-request.scheme\x01V\x01@\x02\x04self\xcd\0\x06scheme\xc4\0\0\xd2\
\0\x04\0#[method]outgoing-request.set-scheme\x01W\x04\0\"[method]outgoing-reques\
t.authority\x01T\x01@\x02\x04self\xcd\0\x09authority\x0e\0\xd2\0\x04\0&[method]o\
utgoing-request.set-authority\x01X\x01@\x01\x04self\xcd\0\0\xc6\0\x04\0\x20[meth\
od]outgoing-request.headers\x01Y\x01i'\x01@\0\0\xda\0\x04\0\x1c[constructor]requ\
est-options\x01[\x01h'\x01k\x01\x01@\x01\x04self\xdc\0\0\xdd\0\x04\0'[method]req\
uest-options.connect-timeout\x01^\x01@\x02\x04self\xdc\0\x08duration\xdd\0\0\xd2\
\0\x04\0+[method]request-options.set-connect-timeout\x01_\x04\0*[method]request-\
options.first-byte-timeout\x01^\x04\0.[method]request-options.set-first-byte-tim\
eout\x01_\x04\0-[method]request-options.between-bytes-timeout\x01^\x04\01[method\
]request-options.set-between-bytes-timeout\x01_\x01i(\x01i.\x01j\x01\xe1\0\x01\x1b\
\x01@\x02\x05param\xe0\0\x08response\xe2\0\x01\0\x04\0\x1d[static]response-outpa\
ram.set\x01c\x01h+\x01@\x01\x04self\xe4\0\0*\x04\0\x20[method]incoming-response.\
status\x01e\x01@\x01\x04self\xe4\0\0\xc6\0\x04\0![method]incoming-response.heade\
rs\x01f\x01@\x01\x04self\xe4\0\0\xc9\0\x04\0![method]incoming-response.consume\x01\
g\x01h,\x01i\x03\x01j\x01\xe9\0\0\x01@\x01\x04self\xe8\0\0\xea\0\x04\0\x1c[metho\
d]incoming-body.stream\x01k\x01i-\x01@\x01\x04this\xc8\0\0\xec\0\x04\0\x1c[stati\
c]incoming-body.finish\x01m\x01h-\x01i\x09\x01@\x01\x04self\xee\0\0\xef\0\x04\0!\
[method]future-trailers.subscribe\x01p\x01i$\x01k\xf1\0\x01j\x01\xf2\0\x01\x1b\x01\
j\x01\xf3\0\0\x01k\xf4\0\x01@\x01\x04self\xee\0\0\xf5\0\x04\0\x1b[method]future-\
trailers.get\x01v\x01@\x01\x07headers\xc6\0\0\xe1\0\x04\0\x1e[constructor]outgoi\
ng-response\x01w\x01h.\x01@\x01\x04self\xf8\0\0*\x04\0%[method]outgoing-response\
.status-code\x01y\x01@\x02\x04self\xf8\0\x0bstatus-code*\0\xd2\0\x04\0)[method]o\
utgoing-response.set-status-code\x01z\x01@\x01\x04self\xf8\0\0\xc6\0\x04\0![meth\
od]outgoing-response.headers\x01{\x01@\x01\x04self\xf8\0\0\xcf\0\x04\0\x1e[metho\
d]outgoing-response.body\x01|\x01h/\x01i\x05\x01j\x01\xfe\0\0\x01@\x01\x04self\xfd\
\0\0\xff\0\x04\0\x1b[method]outgoing-body.write\x01\x80\x01\x01j\0\x01\x1b\x01@\x02\
\x04this\xce\0\x08trailers\xf2\0\0\x81\x01\x04\0\x1c[static]outgoing-body.finish\
\x01\x82\x01\x01h0\x01@\x01\x04self\x83\x01\0\xef\0\x04\0*[method]future-incomin\
g-response.subscribe\x01\x84\x01\x01i+\x01j\x01\x85\x01\x01\x1b\x01j\x01\x86\x01\
\0\x01k\x87\x01\x01@\x01\x04self\x83\x01\0\x88\x01\x04\0$[method]future-incoming\
-response.get\x01\x89\x01\x01h\x07\x01k\x1b\x01@\x01\x03err\x8a\x01\0\x8b\x01\x04\
\0\x0fhttp-error-code\x01\x8c\x01\x03\0\x15wasi:http/types@0.2.0\x05\x0e\x02\x03\
\0\x05\x10outgoing-request\x02\x03\0\x05\x0frequest-options\x02\x03\0\x05\x18fut\
ure-incoming-response\x02\x03\0\x05\x0aerror-code\x01B\x0f\x02\x03\x02\x01\x0f\x04\
\0\x10outgoing-request\x03\0\0\x02\x03\x02\x01\x10\x04\0\x0frequest-options\x03\0\
\x02\x02\x03\x02\x01\x11\x04\0\x18future-incoming-response\x03\0\x04\x02\x03\x02\
\x01\x12\x04\0\x0aerror-code\x03\0\x06\x01i\x01\x01i\x03\x01k\x09\x01i\x05\x01j\x01\
\x0b\x01\x07\x01@\x02\x07request\x08\x07options\x0a\0\x0c\x04\0\x06handle\x01\x0d\
\x03\0\x20wasi:http/outgoing-handler@0.2.0\x05\x13\x02\x03\0\0\x10evm-chain-conf\
ig\x02\x03\0\0\x13cosmos-chain-config\x02\x03\0\0\x09log-level\x01B\x11\x02\x03\x02\
\x01\x14\x04\0\x10evm-chain-config\x03\0\0\x02\x03\x02\x01\x15\x04\0\x13cosmos-c\
hain-config\x03\0\x02\x02\x03\x02\x01\x16\x04\0\x09log-level\x03\0\x04\x01k\x01\x01\
@\x01\x0achain-names\0\x06\x04\0\x14get-evm-chain-config\x01\x07\x01k\x03\x01@\x01\
\x0achain-names\0\x08\x04\0\x17get-cosmos-chain-config\x01\x09\x01ks\x01@\x01\x03\
keys\0\x0a\x04\0\x0aconfig-var\x01\x0b\x01@\x02\x05level\x05\x07messages\x01\0\x04\
\0\x03log\x01\x0c\x03\0\x04host\x05\x17\x01B\x0a\x01o\x02ss\x01p\0\x01@\0\0\x01\x04\
\0\x0fget-environment\x01\x02\x01ps\x01@\0\0\x03\x04\0\x0dget-arguments\x01\x04\x01\
ks\x01@\0\0\x05\x04\0\x0binitial-cwd\x01\x06\x03\0\x1awasi:cli/environment@0.2.0\
\x05\x18\x01B\x03\x01j\0\0\x01@\x01\x06status\0\x01\0\x04\0\x04exit\x01\x01\x03\0\
\x13wasi:cli/exit@0.2.0\x05\x19\x01B\x05\x02\x03\x02\x01\x0c\x04\0\x0cinput-stre\
am\x03\0\0\x01i\x01\x01@\0\0\x02\x04\0\x09get-stdin\x01\x03\x03\0\x14wasi:cli/st\
din@0.2.0\x05\x1a\x01B\x05\x02\x03\x02\x01\x0d\x04\0\x0doutput-stream\x03\0\0\x01\
i\x01\x01@\0\0\x02\x04\0\x0aget-stdout\x01\x03\x03\0\x15wasi:cli/stdout@0.2.0\x05\
\x1b\x01B\x05\x02\x03\x02\x01\x0d\x04\0\x0doutput-stream\x03\0\0\x01i\x01\x01@\0\
\0\x02\x04\0\x0aget-stderr\x01\x03\x03\0\x15wasi:cli/stderr@0.2.0\x05\x1c\x01B\x01\
\x04\0\x0eterminal-input\x03\x01\x03\0\x1dwasi:cli/terminal-input@0.2.0\x05\x1d\x01\
B\x01\x04\0\x0fterminal-output\x03\x01\x03\0\x1ewasi:cli/terminal-output@0.2.0\x05\
\x1e\x02\x03\0\x0d\x0eterminal-input\x01B\x06\x02\x03\x02\x01\x1f\x04\0\x0etermi\
nal-input\x03\0\0\x01i\x01\x01k\x02\x01@\0\0\x03\x04\0\x12get-terminal-stdin\x01\
\x04\x03\0\x1dwasi:cli/terminal-stdin@0.2.0\x05\x20\x02\x03\0\x0e\x0fterminal-ou\
tput\x01B\x06\x02\x03\x02\x01!\x04\0\x0fterminal-output\x03\0\0\x01i\x01\x01k\x02\
\x01@\0\0\x03\x04\0\x13get-terminal-stdout\x01\x04\x03\0\x1ewasi:cli/terminal-st\
dout@0.2.0\x05\"\x01B\x06\x02\x03\x02\x01!\x04\0\x0fterminal-output\x03\0\0\x01i\
\x01\x01k\x02\x01@\0\0\x03\x04\0\x13get-terminal-stderr\x01\x04\x03\0\x1ewasi:cl\
i/terminal-stderr@0.2.0\x05#\x01B\x05\x01r\x02\x07secondsw\x0bnanosecondsy\x04\0\
\x08datetime\x03\0\0\x01@\0\0\x01\x04\0\x03now\x01\x02\x04\0\x0aresolution\x01\x02\
\x03\0\x1cwasi:clocks/wall-clock@0.2.0\x05$\x02\x03\0\x04\x05error\x02\x03\0\x12\
\x08datetime\x01Br\x02\x03\x02\x01\x0c\x04\0\x0cinput-stream\x03\0\0\x02\x03\x02\
\x01\x0d\x04\0\x0doutput-stream\x03\0\x02\x02\x03\x02\x01%\x04\0\x05error\x03\0\x04\
\x02\x03\x02\x01&\x04\0\x08datetime\x03\0\x06\x01w\x04\0\x08filesize\x03\0\x08\x01\
m\x08\x07unknown\x0cblock-device\x10character-device\x09directory\x04fifo\x0dsym\
bolic-link\x0cregular-file\x06socket\x04\0\x0fdescriptor-type\x03\0\x0a\x01n\x06\
\x04read\x05write\x13file-integrity-sync\x13data-integrity-sync\x14requested-wri\
te-sync\x10mutate-directory\x04\0\x10descriptor-flags\x03\0\x0c\x01n\x01\x0esyml\
ink-follow\x04\0\x0apath-flags\x03\0\x0e\x01n\x04\x06create\x09directory\x09excl\
usive\x08truncate\x04\0\x0aopen-flags\x03\0\x10\x01w\x04\0\x0alink-count\x03\0\x12\
\x01k\x07\x01r\x06\x04type\x0b\x0alink-count\x13\x04size\x09\x15data-access-time\
stamp\x14\x1bdata-modification-timestamp\x14\x17status-change-timestamp\x14\x04\0\
\x0fdescriptor-stat\x03\0\x15\x01q\x03\x09no-change\0\0\x03now\0\0\x09timestamp\x01\
\x07\0\x04\0\x0dnew-timestamp\x03\0\x17\x01r\x02\x04type\x0b\x04names\x04\0\x0fd\
irectory-entry\x03\0\x19\x01m%\x06access\x0bwould-block\x07already\x0ebad-descri\
ptor\x04busy\x08deadlock\x05quota\x05exist\x0efile-too-large\x15illegal-byte-seq\
uence\x0bin-progress\x0binterrupted\x07invalid\x02io\x0cis-directory\x04loop\x0e\
too-many-links\x0cmessage-size\x0dname-too-long\x09no-device\x08no-entry\x07no-l\
ock\x13insufficient-memory\x12insufficient-space\x0dnot-directory\x09not-empty\x0f\
not-recoverable\x0bunsupported\x06no-tty\x0eno-such-device\x08overflow\x0dnot-pe\
rmitted\x04pipe\x09read-only\x0cinvalid-seek\x0etext-file-busy\x0ccross-device\x04\
\0\x0aerror-code\x03\0\x1b\x01m\x06\x06normal\x0asequential\x06random\x09will-ne\
ed\x09dont-need\x08no-reuse\x04\0\x06advice\x03\0\x1d\x01r\x02\x05lowerw\x05uppe\
rw\x04\0\x13metadata-hash-value\x03\0\x1f\x04\0\x0adescriptor\x03\x01\x04\0\x16d\
irectory-entry-stream\x03\x01\x01h!\x01i\x01\x01j\x01$\x01\x1c\x01@\x02\x04self#\
\x06offset\x09\0%\x04\0\"[method]descriptor.read-via-stream\x01&\x01i\x03\x01j\x01\
'\x01\x1c\x01@\x02\x04self#\x06offset\x09\0(\x04\0#[method]descriptor.write-via-\
stream\x01)\x01@\x01\x04self#\0(\x04\0$[method]descriptor.append-via-stream\x01*\
\x01j\0\x01\x1c\x01@\x04\x04self#\x06offset\x09\x06length\x09\x06advice\x1e\0+\x04\
\0\x19[method]descriptor.advise\x01,\x01@\x01\x04self#\0+\x04\0\x1c[method]descr\
iptor.sync-data\x01-\x01j\x01\x0d\x01\x1c\x01@\x01\x04self#\0.\x04\0\x1c[method]\
descriptor.get-flags\x01/\x01j\x01\x0b\x01\x1c\x01@\x01\x04self#\00\x04\0\x1b[me\
thod]descriptor.get-type\x011\x01@\x02\x04self#\x04size\x09\0+\x04\0\x1b[method]\
descriptor.set-size\x012\x01@\x03\x04self#\x15data-access-timestamp\x18\x1bdata-\
modification-timestamp\x18\0+\x04\0\x1c[method]descriptor.set-times\x013\x01p}\x01\
o\x024\x7f\x01j\x015\x01\x1c\x01@\x03\x04self#\x06length\x09\x06offset\x09\06\x04\
\0\x17[method]descriptor.read\x017\x01j\x01\x09\x01\x1c\x01@\x03\x04self#\x06buf\
fer4\x06offset\x09\08\x04\0\x18[method]descriptor.write\x019\x01i\"\x01j\x01:\x01\
\x1c\x01@\x01\x04self#\0;\x04\0![method]descriptor.read-directory\x01<\x04\0\x17\
[method]descriptor.sync\x01-\x01@\x02\x04self#\x04paths\0+\x04\0&[method]descrip\
tor.create-directory-at\x01=\x01j\x01\x16\x01\x1c\x01@\x01\x04self#\0>\x04\0\x17\
[method]descriptor.stat\x01?\x01@\x03\x04self#\x0apath-flags\x0f\x04paths\0>\x04\
\0\x1a[method]descriptor.stat-at\x01@\x01@\x05\x04self#\x0apath-flags\x0f\x04pat\
hs\x15data-access-timestamp\x18\x1bdata-modification-timestamp\x18\0+\x04\0\x1f[\
method]descriptor.set-times-at\x01A\x01@\x05\x04self#\x0eold-path-flags\x0f\x08o\
ld-paths\x0enew-descriptor#\x08new-paths\0+\x04\0\x1a[method]descriptor.link-at\x01\
B\x01i!\x01j\x01\xc3\0\x01\x1c\x01@\x05\x04self#\x0apath-flags\x0f\x04paths\x0ao\
pen-flags\x11\x05flags\x0d\0\xc4\0\x04\0\x1a[method]descriptor.open-at\x01E\x01j\
\x01s\x01\x1c\x01@\x02\x04self#\x04paths\0\xc6\0\x04\0\x1e[method]descriptor.rea\
dlink-at\x01G\x04\0&[method]descriptor.remove-directory-at\x01=\x01@\x04\x04self\
#\x08old-paths\x0enew-descriptor#\x08new-paths\0+\x04\0\x1c[method]descriptor.re\
name-at\x01H\x01@\x03\x04self#\x08old-paths\x08new-paths\0+\x04\0\x1d[method]des\
criptor.symlink-at\x01I\x04\0![method]descriptor.unlink-file-at\x01=\x01@\x02\x04\
self#\x05other#\0\x7f\x04\0![method]descriptor.is-same-object\x01J\x01j\x01\x20\x01\
\x1c\x01@\x01\x04self#\0\xcb\0\x04\0\x20[method]descriptor.metadata-hash\x01L\x01\
@\x03\x04self#\x0apath-flags\x0f\x04paths\0\xcb\0\x04\0#[method]descriptor.metad\
ata-hash-at\x01M\x01h\"\x01k\x1a\x01j\x01\xcf\0\x01\x1c\x01@\x01\x04self\xce\0\0\
\xd0\0\x04\03[method]directory-entry-stream.read-directory-entry\x01Q\x01h\x05\x01\
k\x1c\x01@\x01\x03err\xd2\0\0\xd3\0\x04\0\x15filesystem-error-code\x01T\x03\0\x1b\
wasi:filesystem/types@0.2.0\x05'\x02\x03\0\x13\x0adescriptor\x01B\x07\x02\x03\x02\
\x01(\x04\0\x0adescriptor\x03\0\0\x01i\x01\x01o\x02\x02s\x01p\x03\x01@\0\0\x04\x04\
\0\x0fget-directories\x01\x05\x03\0\x1ewasi:filesystem/preopens@0.2.0\x05)\x01B\x11\
\x04\0\x07network\x03\x01\x01m\x15\x07unknown\x0daccess-denied\x0dnot-supported\x10\
invalid-argument\x0dout-of-memory\x07timeout\x14concurrency-conflict\x0fnot-in-p\
rogress\x0bwould-block\x0dinvalid-state\x10new-socket-limit\x14address-not-binda\
ble\x0eaddress-in-use\x12remote-unreachable\x12connection-refused\x10connection-\
reset\x12connection-aborted\x12datagram-too-large\x11name-unresolvable\x1atempor\
ary-resolver-failure\x1apermanent-resolver-failure\x04\0\x0aerror-code\x03\0\x01\
\x01m\x02\x04ipv4\x04ipv6\x04\0\x11ip-address-family\x03\0\x03\x01o\x04}}}}\x04\0\
\x0cipv4-address\x03\0\x05\x01o\x08{{{{{{{{\x04\0\x0cipv6-address\x03\0\x07\x01q\
\x02\x04ipv4\x01\x06\0\x04ipv6\x01\x08\0\x04\0\x0aip-address\x03\0\x09\x01r\x02\x04\
port{\x07address\x06\x04\0\x13ipv4-socket-address\x03\0\x0b\x01r\x04\x04port{\x09\
flow-infoy\x07address\x08\x08scope-idy\x04\0\x13ipv6-socket-address\x03\0\x0d\x01\
q\x02\x04ipv4\x01\x0c\0\x04ipv6\x01\x0e\0\x04\0\x11ip-socket-address\x03\0\x0f\x03\
\0\x1awasi:sockets/network@0.2.0\x05*\x02\x03\0\x15\x07network\x01B\x05\x02\x03\x02\
\x01+\x04\0\x07network\x03\0\0\x01i\x01\x01@\0\0\x02\x04\0\x10instance-network\x01\
\x03\x03\0#wasi:sockets/instance-network@0.2.0\x05,\x02\x03\0\x15\x0aerror-code\x02\
\x03\0\x15\x11ip-socket-address\x02\x03\0\x15\x11ip-address-family\x01BD\x02\x03\
\x02\x01\x06\x04\0\x08pollable\x03\0\0\x02\x03\x02\x01+\x04\0\x07network\x03\0\x02\
\x02\x03\x02\x01-\x04\0\x0aerror-code\x03\0\x04\x02\x03\x02\x01.\x04\0\x11ip-soc\
ket-address\x03\0\x06\x02\x03\x02\x01/\x04\0\x11ip-address-family\x03\0\x08\x01p\
}\x01r\x02\x04data\x0a\x0eremote-address\x07\x04\0\x11incoming-datagram\x03\0\x0b\
\x01k\x07\x01r\x02\x04data\x0a\x0eremote-address\x0d\x04\0\x11outgoing-datagram\x03\
\0\x0e\x04\0\x0audp-socket\x03\x01\x04\0\x18incoming-datagram-stream\x03\x01\x04\
\0\x18outgoing-datagram-stream\x03\x01\x01h\x10\x01h\x03\x01j\0\x01\x05\x01@\x03\
\x04self\x13\x07network\x14\x0dlocal-address\x07\0\x15\x04\0\x1d[method]udp-sock\
et.start-bind\x01\x16\x01@\x01\x04self\x13\0\x15\x04\0\x1e[method]udp-socket.fin\
ish-bind\x01\x17\x01i\x11\x01i\x12\x01o\x02\x18\x19\x01j\x01\x1a\x01\x05\x01@\x02\
\x04self\x13\x0eremote-address\x0d\0\x1b\x04\0\x19[method]udp-socket.stream\x01\x1c\
\x01j\x01\x07\x01\x05\x01@\x01\x04self\x13\0\x1d\x04\0\x20[method]udp-socket.loc\
al-address\x01\x1e\x04\0![method]udp-socket.remote-address\x01\x1e\x01@\x01\x04s\
elf\x13\0\x09\x04\0![method]udp-socket.address-family\x01\x1f\x01j\x01}\x01\x05\x01\
@\x01\x04self\x13\0\x20\x04\0$[method]udp-socket.unicast-hop-limit\x01!\x01@\x02\
\x04self\x13\x05value}\0\x15\x04\0([method]udp-socket.set-unicast-hop-limit\x01\"\
\x01j\x01w\x01\x05\x01@\x01\x04self\x13\0#\x04\0&[method]udp-socket.receive-buff\
er-size\x01$\x01@\x02\x04self\x13\x05valuew\0\x15\x04\0*[method]udp-socket.set-r\
eceive-buffer-size\x01%\x04\0#[method]udp-socket.send-buffer-size\x01$\x04\0'[me\
thod]udp-socket.set-send-buffer-size\x01%\x01i\x01\x01@\x01\x04self\x13\0&\x04\0\
\x1c[method]udp-socket.subscribe\x01'\x01h\x11\x01p\x0c\x01j\x01)\x01\x05\x01@\x02\
\x04self(\x0bmax-resultsw\0*\x04\0([method]incoming-datagram-stream.receive\x01+\
\x01@\x01\x04self(\0&\x04\0*[method]incoming-datagram-stream.subscribe\x01,\x01h\
\x12\x01@\x01\x04self-\0#\x04\0+[method]outgoing-datagram-stream.check-send\x01.\
\x01p\x0f\x01@\x02\x04self-\x09datagrams/\0#\x04\0%[method]outgoing-datagram-str\
eam.send\x010\x01@\x01\x04self-\0&\x04\0*[method]outgoing-datagram-stream.subscr\
ibe\x011\x03\0\x16wasi:sockets/udp@0.2.0\x050\x02\x03\0\x17\x0audp-socket\x01B\x0c\
\x02\x03\x02\x01+\x04\0\x07network\x03\0\0\x02\x03\x02\x01-\x04\0\x0aerror-code\x03\
\0\x02\x02\x03\x02\x01/\x04\0\x11ip-address-family\x03\0\x04\x02\x03\x02\x011\x04\
\0\x0audp-socket\x03\0\x06\x01i\x07\x01j\x01\x08\x01\x03\x01@\x01\x0eaddress-fam\
ily\x05\0\x09\x04\0\x11create-udp-socket\x01\x0a\x03\0$wasi:sockets/udp-create-s\
ocket@0.2.0\x052\x01BT\x02\x03\x02\x01\x0c\x04\0\x0cinput-stream\x03\0\0\x02\x03\
\x02\x01\x0d\x04\0\x0doutput-stream\x03\0\x02\x02\x03\x02\x01\x06\x04\0\x08polla\
ble\x03\0\x04\x02\x03\x02\x01\x0b\x04\0\x08duration\x03\0\x06\x02\x03\x02\x01+\x04\
\0\x07network\x03\0\x08\x02\x03\x02\x01-\x04\0\x0aerror-code\x03\0\x0a\x02\x03\x02\
\x01.\x04\0\x11ip-socket-address\x03\0\x0c\x02\x03\x02\x01/\x04\0\x11ip-address-\
family\x03\0\x0e\x01m\x03\x07receive\x04send\x04both\x04\0\x0dshutdown-type\x03\0\
\x10\x04\0\x0atcp-socket\x03\x01\x01h\x12\x01h\x09\x01j\0\x01\x0b\x01@\x03\x04se\
lf\x13\x07network\x14\x0dlocal-address\x0d\0\x15\x04\0\x1d[method]tcp-socket.sta\
rt-bind\x01\x16\x01@\x01\x04self\x13\0\x15\x04\0\x1e[method]tcp-socket.finish-bi\
nd\x01\x17\x01@\x03\x04self\x13\x07network\x14\x0eremote-address\x0d\0\x15\x04\0\
\x20[method]tcp-socket.start-connect\x01\x18\x01i\x01\x01i\x03\x01o\x02\x19\x1a\x01\
j\x01\x1b\x01\x0b\x01@\x01\x04self\x13\0\x1c\x04\0![method]tcp-socket.finish-con\
nect\x01\x1d\x04\0\x1f[method]tcp-socket.start-listen\x01\x17\x04\0\x20[method]t\
cp-socket.finish-listen\x01\x17\x01i\x12\x01o\x03\x1e\x19\x1a\x01j\x01\x1f\x01\x0b\
\x01@\x01\x04self\x13\0\x20\x04\0\x19[method]tcp-socket.accept\x01!\x01j\x01\x0d\
\x01\x0b\x01@\x01\x04self\x13\0\"\x04\0\x20[method]tcp-socket.local-address\x01#\
\x04\0![method]tcp-socket.remote-address\x01#\x01@\x01\x04self\x13\0\x7f\x04\0\x1f\
[method]tcp-socket.is-listening\x01$\x01@\x01\x04self\x13\0\x0f\x04\0![method]tc\
p-socket.address-family\x01%\x01@\x02\x04self\x13\x05valuew\0\x15\x04\0*[method]\
tcp-socket.set-listen-backlog-size\x01&\x01j\x01\x7f\x01\x0b\x01@\x01\x04self\x13\
\0'\x04\0%[method]tcp-socket.keep-alive-enabled\x01(\x01@\x02\x04self\x13\x05val\
ue\x7f\0\x15\x04\0)[method]tcp-socket.set-keep-alive-enabled\x01)\x01j\x01\x07\x01\
\x0b\x01@\x01\x04self\x13\0*\x04\0'[method]tcp-socket.keep-alive-idle-time\x01+\x01\
@\x02\x04self\x13\x05value\x07\0\x15\x04\0+[method]tcp-socket.set-keep-alive-idl\
e-time\x01,\x04\0&[method]tcp-socket.keep-alive-interval\x01+\x04\0*[method]tcp-\
socket.set-keep-alive-interval\x01,\x01j\x01y\x01\x0b\x01@\x01\x04self\x13\0-\x04\
\0#[method]tcp-socket.keep-alive-count\x01.\x01@\x02\x04self\x13\x05valuey\0\x15\
\x04\0'[method]tcp-socket.set-keep-alive-count\x01/\x01j\x01}\x01\x0b\x01@\x01\x04\
self\x13\00\x04\0\x1c[method]tcp-socket.hop-limit\x011\x01@\x02\x04self\x13\x05v\
alue}\0\x15\x04\0\x20[method]tcp-socket.set-hop-limit\x012\x01j\x01w\x01\x0b\x01\
@\x01\x04self\x13\03\x04\0&[method]tcp-socket.receive-buffer-size\x014\x04\0*[me\
thod]tcp-socket.set-receive-buffer-size\x01&\x04\0#[method]tcp-socket.send-buffe\
r-size\x014\x04\0'[method]tcp-socket.set-send-buffer-size\x01&\x01i\x05\x01@\x01\
\x04self\x13\05\x04\0\x1c[method]tcp-socket.subscribe\x016\x01@\x02\x04self\x13\x0d\
shutdown-type\x11\0\x15\x04\0\x1b[method]tcp-socket.shutdown\x017\x03\0\x16wasi:\
sockets/tcp@0.2.0\x053\x02\x03\0\x19\x0atcp-socket\x01B\x0c\x02\x03\x02\x01+\x04\
\0\x07network\x03\0\0\x02\x03\x02\x01-\x04\0\x0aerror-code\x03\0\x02\x02\x03\x02\
\x01/\x04\0\x11ip-address-family\x03\0\x04\x02\x03\x02\x014\x04\0\x0atcp-socket\x03\
\0\x06\x01i\x07\x01j\x01\x08\x01\x03\x01@\x01\x0eaddress-family\x05\0\x09\x04\0\x11\
create-tcp-socket\x01\x0a\x03\0$wasi:sockets/tcp-create-socket@0.2.0\x055\x02\x03\
\0\x15\x0aip-address\x01B\x16\x02\x03\x02\x01\x06\x04\0\x08pollable\x03\0\0\x02\x03\
\x02\x01+\x04\0\x07network\x03\0\x02\x02\x03\x02\x01-\x04\0\x0aerror-code\x03\0\x04\
\x02\x03\x02\x016\x04\0\x0aip-address\x03\0\x06\x04\0\x16resolve-address-stream\x03\
\x01\x01h\x08\x01k\x07\x01j\x01\x0a\x01\x05\x01@\x01\x04self\x09\0\x0b\x04\03[me\
thod]resolve-address-stream.resolve-next-address\x01\x0c\x01i\x01\x01@\x01\x04se\
lf\x09\0\x0d\x04\0([method]resolve-address-stream.subscribe\x01\x0e\x01h\x03\x01\
i\x08\x01j\x01\x10\x01\x05\x01@\x02\x07network\x0f\x04names\0\x11\x04\0\x11resol\
ve-addresses\x01\x12\x03\0!wasi:sockets/ip-name-lookup@0.2.0\x057\x01B\x05\x01p}\
\x01@\x01\x03lenw\0\0\x04\0\x10get-random-bytes\x01\x01\x01@\0\0w\x04\0\x0eget-r\
andom-u64\x01\x02\x03\0\x18wasi:random/random@0.2.0\x058\x01B\x05\x01p}\x01@\x01\
\x03lenw\0\0\x04\0\x19get-insecure-random-bytes\x01\x01\x01@\0\0w\x04\0\x17get-i\
nsecure-random-u64\x01\x02\x03\0\x1awasi:random/insecure@0.2.0\x059\x01B\x03\x01\
o\x02ww\x01@\0\0\0\x04\0\x0dinsecure-seed\x01\x01\x03\0\x1fwasi:random/insecure-\
seed@0.2.0\x05:\x01k\x04\x01j\x01;\x01s\x01@\x01\x0etrigger-action\x02\0<\x04\0\x03\
run\x01=\x04\0%wavs:worker/layer-trigger-world@0.4.0\x04\0\x0b\x19\x01\0\x13laye\
r-trigger-world\x03\0\0\0G\x09producers\x01\x0cprocessed-by\x02\x0dwit-component\
\x070.220.0\x10wit-bindgen-rust\x060.35.0";
#[inline(never)]
#[doc(hidden)]
pub fn __link_custom_section_describing_imports() {
    wit_bindgen_rt::maybe_link_cabi_realloc();
}
