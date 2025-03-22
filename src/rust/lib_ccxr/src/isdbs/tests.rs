extern crate libc;
use crate::isdbs::isdbd::*;
use crate::isdbs::isdbs::*;

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CString;
    use std::ptr;

    #[test]
    fn test_layout_get_height_horizontal_960x540() {
        assert_eq!(layout_get_height(WritingFormat::Horizontal960x540), 540);
    }

    #[test]
    fn test_layout_get_height_vertical_960x540() {
        assert_eq!(layout_get_height(WritingFormat::Vertical960x540), 540);
    }

    #[test]
    fn test_layout_get_height_horizontal_1920x1080() {
        assert_eq!(layout_get_height(WritingFormat::Horizontal1920x1080), 480);
    }

    #[test]
    fn test_layout_get_height_vertical_1920x1080() {
        assert_eq!(layout_get_height(WritingFormat::Vertical1920x1080), 480);
    }

    #[test]
    fn test_layout_get_height_default() {
        assert_eq!(layout_get_height(WritingFormat::None), 480);
    }

    #[test]
    fn test_is_horizontal_layout_horizontal_std_density() {
        assert!(is_horizontal_layout(WritingFormat::HorizontalStdDensity));
    }

    #[test]
    fn test_is_horizontal_layout_horizontal_high_density() {
        assert!(is_horizontal_layout(WritingFormat::HorizontalHighDensity));
    }

    #[test]
    fn test_is_horizontal_layout_horizontal_western_lang() {
        assert!(is_horizontal_layout(WritingFormat::HorizontalWesternLang));
    }

    #[test]
    fn test_is_horizontal_layout_horizontal_1920x1080() {
        assert!(is_horizontal_layout(WritingFormat::Horizontal1920x1080));
    }

    #[test]
    fn test_is_horizontal_layout_vertical_1920x1080() {
        assert!(!is_horizontal_layout(WritingFormat::Vertical1920x1080));
    }

    #[test]
    fn test_layout_get_width_horizontal_960x540() {
        assert_eq!(layout_get_width(WritingFormat::Horizontal960x540), 960);
    }

    #[test]
    fn test_layout_get_width_vertical_960x540() {
        assert_eq!(layout_get_width(WritingFormat::Vertical960x540), 960);
    }

    #[test]
    fn test_layout_get_width_horizontal_1920x1080() {
        assert_eq!(layout_get_width(WritingFormat::Horizontal1920x1080), 720);
    }

    #[test]
    fn test_layout_get_width_vertical_1920x1080() {
        assert_eq!(layout_get_width(WritingFormat::Vertical1920x1080), 720);
    }

    #[test]
    fn test_layout_get_width_default() {
        assert_eq!(layout_get_width(WritingFormat::None), 720);
    }

    #[test]
    fn test_isdb_set_global_time() {
        let mut ctx = ISDBSubContext {
            nb_char: 0,
            nb_line: 0,
            timestamp: 0,
            prev_timestamp: 0,
            text_list_head: ListHead {
                next: ptr::null_mut(),
                prev: ptr::null_mut(),
            },
            buffered_text: ListHead {
                next: ptr::null_mut(),
                prev: ptr::null_mut(),
            },
            current_state: ISDBSubState {
                auto_display: 0,
                rollup_mode: 0,
                need_init: 0,
                clut_high_idx: 0,
                fg_color: 0,
                bg_color: 0,
                hfg_color: 0,
                hbg_color: 0,
                mat_color: 0,
                raster_color: 0,
                layout_state: ISDBSubLayout {
                    format: WritingFormat::None,
                    display_area: DisplayArea { x: 0, y: 0 },
                    font_size: 0,
                    font_scale: FontScale { fscx: 0, fscy: 0 },
                    cell_spacing: 0,
                    cursor_pos: DisplayArea { x: 0, y: 0 },
                    ccc: 0,
                    acps: 0,
                },
            },
            tmd: IsdbTmd::Free,
            nb_lang: 0,
            offset_time: OffsetTime {
                hour: 0,
                min: 0,
                sec: 0,
                milli: 0,
            },
            dmf: 0,
            dc: 0,
            cfg_no_rollup: 0,
        };
        isdb_set_global_time(&mut ctx, 123456789);
        assert_eq!(ctx.timestamp, 123456789);
    }

    #[test]
    fn test_init_layout() {
        let mut layout = ISDBSubLayout {
            font_size: 0,
            display_area: DisplayArea { x: 0, y: 0 },
            font_scale: FontScale { fscx: 0, fscy: 0 },
            format: WritingFormat::None,
            cell_spacing: 0,
            cursor_pos: DisplayArea { x: 0, y: 0 },
            ccc: 0,
            acps: 0,
        };
        init_layout(&mut layout);
        unsafe {
            assert_eq!(layout.font_size, 36);
            assert_eq!(layout.display_area.x, 0);
            assert_eq!(layout.display_area.y, 0);
            assert_eq!(layout.font_scale.fscx, 100);
            assert_eq!(layout.font_scale.fscy, 100);
        }
    }

    #[test]
    fn test_init_list_head() {
        let mut list_head = ListHead {
            next: ptr::null_mut(),
            prev: ptr::null_mut(),
        };
        init_list_head(&mut list_head);
        unsafe {
            assert_eq!(list_head.next, &mut list_head as *mut _);
            assert_eq!(list_head.prev, &mut list_head as *mut _);
        }
    }

    #[test]
    fn test_isdb_set_global_time_zero() {
        let mut ctx = ISDBSubContext {
            timestamp: 123456789,
            ..Default::default()
        };
        isdb_set_global_time(&mut ctx, 0);
        assert_eq!(ctx.timestamp, 0);
    }

    #[test]
    fn test_init_layout_custom_values() {
        let mut layout = ISDBSubLayout {
            font_size: 50,
            display_area: DisplayArea { x: 10, y: 20 },
            font_scale: FontScale {
                fscx: 150,
                fscy: 150,
            },
            format: WritingFormat::None,
            cell_spacing: 0,
            cursor_pos: DisplayArea { x: 0, y: 0 },
            ccc: 0,
            acps: 0,
        };
        init_layout(&mut layout);
        unsafe {
            assert_eq!(layout.font_size, 36);
            assert_eq!(layout.display_area.x, 0);
            assert_eq!(layout.display_area.y, 0);
            assert_eq!(layout.font_scale.fscx, 100);
            assert_eq!(layout.font_scale.fscy, 100);
        }
    }

    #[test]
    fn test_init_list_head_null() {
        let mut list_head = ListHead {
            next: ptr::null_mut(),
            prev: ptr::null_mut(),
        };
        init_list_head(&mut list_head);
        unsafe {
            assert_eq!(list_head.next, &mut list_head as *mut _);
            assert_eq!(list_head.prev, &mut list_head as *mut _);
        }
    }

    #[test]
    fn test_free() {
        unsafe {
            let layout = Layout::new::<u8>();
            let ptr = alloc(layout) as *mut c_void;
            assert!(!ptr.is_null());
            free(ptr);
        }
    }

    #[test]
    fn test_free_null() {
        unsafe {
            let ptr: *mut c_void = ptr::null_mut();
            free(ptr);
        }
    }

    #[test]
    fn test_freep() {
        unsafe {
            let layout = Layout::new::<u8>();
            let mut ptr = alloc(layout) as *mut c_void;
            assert!(!ptr.is_null());
            freep(&mut ptr);
            assert!(ptr.is_null());
        }
    }

    #[test]
    fn test_freep_null() {
        unsafe {
            let mut ptr: *mut c_void = ptr::null_mut();
            freep(&mut ptr);
            assert!(ptr.is_null());
        }
    }

    use super::*;
    use std::ptr;

    #[test]
    fn test___list_del() {
        let mut head = ListHead {
            next: ptr::null_mut(),
            prev: ptr::null_mut(),
        };
        let mut node1 = ListHead {
            next: ptr::null_mut(),
            prev: ptr::null_mut(),
        };
        let mut node2 = ListHead {
            next: ptr::null_mut(),
            prev: ptr::null_mut(),
        };

        head.next = &mut node1;
        node1.prev = &mut head;
        node1.next = &mut node2;
        node2.prev = &mut node1;

        unsafe {
            __list_del(&mut node1, &mut node2);
        }

        unsafe {
            assert_eq!(node2.prev, &mut head);
            assert_eq!(head.next, &mut node2);
        }
    }

    #[test]
    fn test___list_del_single_node() {
        let mut head = ListHead {
            next: ptr::null_mut(),
            prev: ptr::null_mut(),
        };
        let mut node = ListHead {
            next: ptr::null_mut(),
            prev: ptr::null_mut(),
        };

        head.next = &mut node;
        node.prev = &mut head;
        node.next = &mut head;
        head.prev = &mut node;

        unsafe {
            __list_del(&mut head, &mut node);
        }

        unsafe {
            assert_eq!(node.prev, &mut head);
            assert_eq!(head.next, &mut head);
        }
    }

    #[test]
    fn test_list_del() {
        let mut head = ListHead {
            next: ptr::null_mut(),
            prev: ptr::null_mut(),
        };
        let mut node = ListHead {
            next: ptr::null_mut(),
            prev: ptr::null_mut(),
        };

        head.next = &mut node;
        node.prev = &mut head;
        node.next = &mut head;
        head.prev = &mut node;

        unsafe {
            list_del(&mut node);
        }

        unsafe {
            assert_eq!(node.next, LIST_POISON1);
            assert_eq!(node.prev, LIST_POISON2);
            assert_eq!(head.next, &mut head);
            assert_eq!(head.prev, &mut head);
        }
    }

    #[test]
    fn test_list_del_multiple_nodes() {
        let mut head = ListHead {
            next: ptr::null_mut(),
            prev: ptr::null_mut(),
        };
        let mut node1 = ListHead {
            next: ptr::null_mut(),
            prev: ptr::null_mut(),
        };
        let mut node2 = ListHead {
            next: ptr::null_mut(),
            prev: ptr::null_mut(),
        };

        head.next = &mut node1;
        node1.prev = &mut head;
        node1.next = &mut node2;
        node2.prev = &mut node1;
        node2.next = &mut head;
        head.prev = &mut node2;

        unsafe {
            list_del(&mut node1);
        }

        unsafe {
            assert_eq!(node1.next, LIST_POISON1);
            assert_eq!(node1.prev, LIST_POISON2);
            assert_eq!(node2.prev, &mut head);
            assert_eq!(head.next, &mut node2);
        }
    }

    use super::*;
    use std::ptr;

    #[test]
    fn test_list_for_each_entry_safe() {
        let mut head = ListHead {
            next: ptr::null_mut(),
            prev: ptr::null_mut(),
        };
        let mut node1 = ISDBText {
            list: ListHead {
                next: ptr::null_mut(),
                prev: ptr::null_mut(),
            },
            buf: ptr::null_mut(),
            len: 0,
            used: 0,
            pos: 0,
            txt_tail: ptr::null_mut(),
            timestamp: 0,
            refcount: 0,
        };
        let mut node2 = ISDBText {
            list: ListHead {
                next: ptr::null_mut(),
                prev: ptr::null_mut(),
            },
            buf: ptr::null_mut(),
            len: 0,
            used: 0,
            pos: 0,
            txt_tail: ptr::null_mut(),
            timestamp: 0,
            refcount: 0,
        };

        head.next = &mut node1.list;
        node1.list.prev = &mut head;
        node1.list.next = &mut node2.list;
        node2.list.prev = &mut node1.list;
        node2.list.next = &mut head;
        head.prev = &mut node2.list;

        unsafe {
            list_for_each_entry_safe(&mut node1, &mut node2, &mut head, b"list\0".as_ptr());
        }
    }

    #[test]
    fn test_list_entry() {
        let mut node = ISDBText {
            list: ListHead {
                next: ptr::null_mut(),
                prev: ptr::null_mut(),
            },
            buf: ptr::null_mut(),
            len: 0,
            used: 0,
            pos: 0,
            txt_tail: ptr::null_mut(),
            timestamp: 0,
            refcount: 0,
        };
        let list_ptr = &mut node.list as *mut ListHead;
        let text_ptr = unsafe { list_entry(list_ptr, b"list\0".as_ptr()) };
        unsafe {
            assert_eq!((*text_ptr).data, 42);
        }
    }

    #[test]
    fn test_container_of() {
        let mut node = ISDBText {
            list: ListHead {
                next: ptr::null_mut(),
                prev: ptr::null_mut(),
            },
            buf: ptr::null_mut(),
            len: 0,
            used: 0,
            pos: 0,
            txt_tail: ptr::null_mut(),
            timestamp: 0,
            refcount: 0,
        };
        let list_ptr = &mut node.list as *mut ListHead;
        let text_ptr = unsafe { container_of(list_ptr, b"list\0".as_ptr()) };
        unsafe {
            assert_eq!((*text_ptr).data, 42);
        }
    }

    #[test]
    fn test_ccx_offsetof() {
        let offset = unsafe { ccx_offsetof(b"list\0".as_ptr()) };
        assert_eq!(offset, std::mem::offset_of!(ISDBText, list));
    }

    #[test]
    fn test_delete_isdb_decoder_empty() {
        let mut ctx = ISDBSubContext {
            nb_char: 0,
            nb_line: 0,
            timestamp: 0,
            prev_timestamp: 0,
            text_list_head: ListHead {
                next: ptr::null_mut(),
                prev: ptr::null_mut(),
            },
            buffered_text: ListHead {
                next: ptr::null_mut(),
                prev: ptr::null_mut(),
            },
            current_state: ISDBSubState {
                auto_display: 0,
                rollup_mode: 0,
                need_init: 0,
                clut_high_idx: 0,
                fg_color: 0,
                bg_color: 0,
                hfg_color: 0,
                hbg_color: 0,
                mat_color: 0,
                raster_color: 0,
                layout_state: ISDBSubLayout {
                    format: WritingFormat::HorizontalStdDensity,
                    display_area: DispArea {
                        x: 0,
                        y: 0,
                        w: 0,
                        h: 0,
                    },
                    font_size: 36,
                    font_scale: FScale {
                        fscx: 100,
                        fscy: 100,
                    },
                    cell_spacing: Spacing { col: 0, row: 0 },
                    cursor_pos: ISDBPos { x: 0, y: 0 },
                    ccc: IsdbCCComposition::None,
                    acps: [0; 2],
                },
            },
            tmd: IsdbTmd::Free,
            nb_lang: 0,
            offset_time: OffsetTime {
                hour: 0,
                min: 0,
                sec: 0,
                milli: 0,
            },
            dmf: 0,
            dc: 0,
            cfg_no_rollup: 0,
        };

        unsafe {
            init_list_head(&mut ctx.text_list_head);
            init_list_head(&mut ctx.buffered_text);
            delete_isdb_decoder(&mut ctx);
        }

        unsafe {
            assert_eq!(ctx.text_list_head.next, &mut ctx.text_list_head as *mut _);
            assert_eq!(ctx.text_list_head.prev, &mut ctx.text_list_head as *mut _);
            assert_eq!(ctx.buffered_text.next, &mut ctx.buffered_text as *mut _);
            assert_eq!(ctx.buffered_text.prev, &mut ctx.buffered_text as *mut _);
        }
    }

    #[test]
    unsafe fn test_delete_isdb_decoder_with_entries() {
        let mut ctx = ISDBSubContext {
            text_list_head: ListHead {
                next: ptr::null_mut(),
                prev: ptr::null_mut(),
            },
            buffered_text: ListHead {
                next: ptr::null_mut(),
                prev: ptr::null_mut(),
            },
            timestamp: 0,
            nb_char: 0,
            nb_line: 0,
            prev_timestamp: 0,
            current_state: ISDBSubState::default(),
            tmd: IsdbTmd::Free,
            nb_lang: 0,
            offset_time: OffsetTime {
                hour: 0,
                min: 0,
                sec: 0,
                milli: 0,
            },
            dmf: 0,
            dc: 0,
            cfg_no_rollup: 0,
        };

        let mut text1 = ISDBText {
            list: ListHead {
                next: ptr::null_mut(),
                prev: ptr::null_mut(),
            },
            buf: alloc(Layout::from_size_align(128, 1).unwrap()) as *mut i8,
            used: 0,
            len: 128,
            pos: ISDBPos { x: 0, y: 0 },
            txt_tail: ptr::null_mut(),
            timestamp: 0,
            refcount: 0,
        };

        let mut text2 = ISDBText {
            list: ListHead {
                next: ptr::null_mut(),
                prev: ptr::null_mut(),
            },
            buf: alloc(Layout::from_size_align(128, 1).unwrap()) as *mut i8,
            used: 0,
            len: 128,
            pos: ISDBPos { x: 0, y: 0 },
            txt_tail: 0,
            timestamp: 0,
            refcount: 0,
        };

        unsafe {
            init_list_head(&mut ctx.text_list_head);
            init_list_head(&mut ctx.buffered_text);
            list_add_tail(&mut text1.list, &mut ctx.text_list_head);
            list_add_tail(&mut text2.list, &mut ctx.buffered_text);
            delete_isdb_decoder(&mut ctx);
        }

        unsafe {
            assert_eq!(ctx.text_list_head.next, &mut ctx.text_list_head as *mut _);
            assert_eq!(ctx.text_list_head.prev, &mut ctx.text_list_head as *mut _);
            assert_eq!(ctx.buffered_text.next, &mut ctx.buffered_text as *mut _);
            assert_eq!(ctx.buffered_text.prev, &mut ctx.buffered_text as *mut _);
        }
    }

    use super::*;
    use std::fmt::Write;

    #[test]
    fn test_isdb_log() {
        let mut output = String::new();
        write!(&mut output, "Test log message").unwrap();
        isdb_log(format_args!("{}", output));
        // Since isdb_log does nothing, there's nothing to assert here.
        // This test ensures that the function can be called without errors.
    }

    use super::*;
    use std::alloc::{alloc, dealloc, Layout};

    #[test]
    fn test_allocate_text_node() {
        let layout = ISDBSubLayout {
            font_size: 36,
            display_area: DisplayArea { x: 0, y: 0 },
            font_scale: FontScale {
                fscx: 100,
                fscy: 100,
            },
            format: WritingFormat::None,
            cell_spacing: 0,
            cursor_pos: DisplayArea { x: 0, y: 0 },
            ccc: 0,
            acps: 0,
        };

        let text_node = allocate_text_node(&layout);
        assert!(!text_node.is_null());

        unsafe {
            assert_eq!((*text_node).used, 0);
            assert_eq!((*text_node).len, 128);
            assert!(!(*text_node).buf.is_null());
            assert_eq!(*(*text_node).buf, 0);

            let text_layout = Layout::new::<ISDBText>();
            dealloc(text_node as *mut u8, text_layout);
        }
    }

    #[test]
    fn test_reserve_buf() {
        let mut text = ISDBText {
            list: ListHead {
                next: ptr::null_mut(),
                prev: ptr::null_mut(),
            },
            buf: unsafe { alloc(Layout::from_size_align(128, 1).unwrap()) } as *mut i8,
            used: 0,
            len: 128,
            pos: Position { x: 0, y: 0 },
            txt_tail: ptr::null_mut(),
            timestamp: 0,
            refcount: 0,
        };

        unsafe {
            assert_eq!(reserve_buf(&mut text, 100), 0);
            assert_eq!(text.len, 128);

            assert_eq!(reserve_buf(&mut text, 200), 0);
            assert!(text.len >= 328);

            let layout = Layout::from_size_align(text.len, 1).unwrap();
            dealloc(text.buf as *mut u8, layout);
        }
    }

    use super::*;
    use std::ptr;

    #[test]
    fn test_list_add() {
        let mut head = ListHead {
            next: ptr::null_mut(),
            prev: ptr::null_mut(),
        };
        let mut elem = ListHead {
            next: ptr::null_mut(),
            prev: ptr::null_mut(),
        };

        unsafe {
            init_list_head(&mut head);
            list_add(&mut elem, &mut head);
        }

        assert_eq!(head.next, &mut elem as *mut _);
        assert_eq!(elem.prev, &mut head as *mut _);
    }

    #[test]
    fn test_list_add_tail() {
        let mut head = ListHead {
            next: ptr::null_mut(),
            prev: ptr::null_mut(),
        };
        let mut elem = ListHead {
            next: ptr::null_mut(),
            prev: ptr::null_mut(),
        };

        unsafe {
            init_list_head(&mut head);
            list_add_tail(&mut elem, &mut head);
        }

        assert_eq!(head.prev, &mut elem as *mut _);
        assert_eq!(elem.next, &mut head as *mut _);
    }

    #[test]
    fn test_list_for_each_entry() {
        let mut head = ListHead {
            next: ptr::null_mut(),
            prev: ptr::null_mut(),
        };
        let mut elem1 = ISDBText {
            buf: ptr::null_mut(),
            len: 0,
            used: 0,
            pos: ISDBPos { x: 0, y: 0 },
            txt_tail: 0,
            timestamp: 0,
            refcount: 0,
            list: ListHead {
                next: ptr::null_mut(),
                prev: ptr::null_mut(),
            },
        };
        let mut elem2 = ISDBText {
            buf: ptr::null_mut(),
            len: 0,
            used: 0,
            pos: ISDBPos { x: 0, y: 0 },
            txt_tail: 0,
            timestamp: 0,
            refcount: 0,
            list: ListHead {
                next: ptr::null_mut(),
                prev: ptr::null_mut(),
            },
        };

        unsafe {
            init_list_head(&mut head);
            list_add(&mut elem1.list, &mut head);
            list_add(&mut elem2.list, &mut head);

            let mut count = 0;
            list_for_each_entry(&mut head, std::mem::offset_of!(ISDBText, list), |text| {
                count += 1;
            });

            assert_eq!(count, 2);
        }
    }

    #[test]
    fn test_append_char() {
        let mut ctx = ISDBSubContext {
            nb_char: 0,
            nb_line: 0,
            timestamp: 0,
            prev_timestamp: 0,
            text_list_head: ListHead {
                next: ptr::null_mut(),
                prev: ptr::null_mut(),
            },
            buffered_text: ListHead {
                next: ptr::null_mut(),
                prev: ptr::null_mut(),
            },
            current_state: ISDBSubState {
                auto_display: 0,
                rollup_mode: 0,
                need_init: 0,
                clut_high_idx: 0,
                fg_color: 0,
                bg_color: 0,
                hfg_color: 0,
                hbg_color: 0,
                mat_color: 0,
                raster_color: 0,
                layout_state: ISDBSubLayout {
                    format: WritingFormat::HorizontalStdDensity,
                    display_area: DispArea {
                        x: 0,
                        y: 0,
                        w: 0,
                        h: 0,
                    },
                    font_size: 36,
                    font_scale: FScale {
                        fscx: 100,
                        fscy: 100,
                    },
                    cell_spacing: Spacing { col: 0, row: 0 },
                    cursor_pos: ISDBPos { x: 0, y: 0 },
                    ccc: IsdbCCComposition::None,
                    acps: [0; 2],
                },
            },
            tmd: IsdbTmd::Free,
            nb_lang: 0,
            offset_time: OffsetTime {
                hour: 0,
                min: 0,
                sec: 0,
                milli: 0,
            },
            dmf: 0,
            dc: 0,
            cfg_no_rollup: 0,
        };

        unsafe {
            init_list_head(&mut ctx.text_list_head);
            let result = append_char(&mut ctx, 'A');
            assert_eq!(result, 1);
        }
    }

    #[test]
    fn test_ccx_strstr_ignorespace() {
        let str1 = CString::new("hello world").unwrap();
        let str2 = CString::new("hello  world").unwrap();
        let result = ccx_strstr_ignorespace(str1.as_ptr(), str2.as_ptr());
        assert_eq!(result, 1);
    }

    #[test]
    fn test_list_empty() {
        let mut head = ListHead {
            next: ptr::null_mut(),
            prev: ptr::null_mut(),
        };

        unsafe {
            init_list_head(&mut head);
        }

        assert!(list_empty(&head));
    }

    #[test]
    fn test_append_char() {
        let mut ctx = ISDBSubContext {
            nb_char: 0,
            nb_line: 0,
            timestamp: 0,
            prev_timestamp: 0,
            text_list_head: ListHead {
                next: ptr::null_mut(),
                prev: ptr::null_mut(),
            },
            buffered_text: ListHead {
                next: ptr::null_mut(),
                prev: ptr::null_mut(),
            },
            current_state: ISDBSubState {
                auto_display: 0,
                rollup_mode: 0,
                need_init: 0,
                clut_high_idx: 0,
                fg_color: 0,
                bg_color: 0,
                hfg_color: 0,
                hbg_color: 0,
                mat_color: 0,
                raster_color: 0,
                layout_state: ISDBSubLayout {
                    format: WritingFormat::HorizontalStdDensity,
                    display_area: DispArea {
                        x: 0,
                        y: 0,
                        w: 0,
                        h: 0,
                    },
                    font_size: 36,
                    font_scale: FScale {
                        fscx: 100,
                        fscy: 100,
                    },
                    cell_spacing: Spacing { col: 0, row: 0 },
                    cursor_pos: ISDBPos { x: 0, y: 0 },
                    ccc: IsdbCCComposition::None,
                    acps: [0; 2],
                },
            },
            tmd: IsdbTmd::Free,
            nb_lang: 0,
            offset_time: OffsetTime {
                hour: 0,
                min: 0,
                sec: 0,
                milli: 0,
            },
            dmf: 0,
            dc: 0,
            cfg_no_rollup: 0,
        };

        unsafe {
            init_list_head(&mut ctx.text_list_head);
            let result = append_char(&mut ctx, 'A');
            assert_eq!(result, 1);
        }
    }

    #[test]
    fn test_ccx_strstr_ignorespace() {
        let str1 = CString::new("hello world").unwrap();
        let str2 = CString::new("hello  world").unwrap();
        let result = ccx_strstr_ignorespace(str1.as_ptr(), str2.as_ptr());
        assert_eq!(result, 1);
    }

    #[test]
    fn test_list_empty() {
        let mut head = ListHead {
            next: ptr::null_mut(),
            prev: ptr::null_mut(),
        };

        unsafe {
            init_list_head(&mut head);
        }

        assert!(list_empty(&head));
    }

    #[test]
    fn test_get_text() {
        let mut ctx = ISDBSubContext {
            nb_char: 0,
            nb_line: 0,
            timestamp: 0,
            prev_timestamp: 0,
            text_list_head: ListHead {
                next: ptr::null_mut(),
                prev: ptr::null_mut(),
            },
            buffered_text: ListHead {
                next: ptr::null_mut(),
                prev: ptr::null_mut(),
            },
            current_state: ISDBSubState {
                auto_display: 0,
                rollup_mode: 0,
                need_init: 0,
                clut_high_idx: 0,
                fg_color: 0,
                bg_color: 0,
                hfg_color: 0,
                hbg_color: 0,
                mat_color: 0,
                raster_color: 0,
                layout_state: ISDBSubLayout {
                    format: WritingFormat::HorizontalStdDensity,
                    display_area: DispArea {
                        x: 0,
                        y: 0,
                        w: 0,
                        h: 0,
                    },
                    font_size: 36,
                    font_scale: FScale {
                        fscx: 100,
                        fscy: 100,
                    },
                    cell_spacing: Spacing { col: 0, row: 0 },
                    cursor_pos: ISDBPos { x: 0, y: 0 },
                    ccc: IsdbCCComposition::None,
                    acps: [0; 2],
                },
            },
            tmd: IsdbTmd::Free,
            nb_lang: 0,
            offset_time: OffsetTime {
                hour: 0,
                min: 0,
                sec: 0,
                milli: 0,
            },
            dmf: 0,
            dc: 0,
            cfg_no_rollup: 0,
        };

        unsafe {
            init_list_head(&mut ctx.text_list_head);
            let mut buffer = [0u8; 1024];
            let result = get_text(&mut ctx, &mut buffer, 1024);
            assert_eq!(result, 0);
        }
    }

    #[test]
    fn test_set_writing_format() {
        let mut ctx = ISDBSubContext {
            nb_char: 0,
            nb_line: 0,
            timestamp: 0,
            prev_timestamp: 0,
            text_list_head: ListHead {
                next: ptr::null_mut(),
                prev: ptr::null_mut(),
            },
            buffered_text: ListHead {
                next: ptr::null_mut(),
                prev: ptr::null_mut(),
            },
            current_state: ISDBSubState {
                auto_display: 0,
                rollup_mode: 0,
                need_init: 0,
                clut_high_idx: 0,
                fg_color: 0,
                bg_color: 0,
                hfg_color: 0,
                hbg_color: 0,
                mat_color: 0,
                raster_color: 0,
                layout_state: ISDBSubLayout {
                    format: WritingFormat::HorizontalStdDensity,
                    display_area: DispArea {
                        x: 0,
                        y: 0,
                        w: 0,
                        h: 0,
                    },
                    font_size: 36,
                    font_scale: FScale {
                        fscx: 100,
                        fscy: 100,
                    },
                    cell_spacing: Spacing { col: 0, row: 0 },
                    cursor_pos: ISDBPos { x: 0, y: 0 },
                    ccc: IsdbCCComposition::None,
                    acps: [0; 2],
                },
            },
            tmd: IsdbTmd::Free,
            nb_lang: 0,
            offset_time: OffsetTime {
                hour: 0,
                min: 0,
                sec: 0,
                milli: 0,
            },
            dmf: 0,
            dc: 0,
            cfg_no_rollup: 0,
        };

        let arg = [0x00, 0x20];
        set_writing_format(&mut ctx, &arg);
        assert_eq!(
            ctx.current_state.layout_state.format,
            WritingFormat::HorizontalStdDensity
        );
    }

    #[test]
    fn test_move_penpos() {
        let mut ctx = ISDBSubContext {
            nb_char: 0,
            nb_line: 0,
            timestamp: 0,
            prev_timestamp: 0,
            text_list_head: ListHead {
                next: ptr::null_mut(),
                prev: ptr::null_mut(),
            },
            buffered_text: ListHead {
                next: ptr::null_mut(),
                prev: ptr::null_mut(),
            },
            current_state: ISDBSubState {
                auto_display: 0,
                rollup_mode: 0,
                need_init: 0,
                clut_high_idx: 0,
                fg_color: 0,
                bg_color: 0,
                hfg_color: 0,
                hbg_color: 0,
                mat_color: 0,
                raster_color: 0,
                layout_state: ISDBSubLayout {
                    format: WritingFormat::HorizontalStdDensity,
                    display_area: DispArea {
                        x: 0,
                        y: 0,
                        w: 0,
                        h: 0,
                    },
                    font_size: 36,
                    font_scale: FScale {
                        fscx: 100,
                        fscy: 100,
                    },
                    cell_spacing: Spacing { col: 0, row: 0 },
                    cursor_pos: ISDBPos { x: 0, y: 0 },
                    ccc: IsdbCCComposition::None,
                    acps: [0; 2],
                },
            },
            tmd: IsdbTmd::Free,
            nb_lang: 0,
            offset_time: OffsetTime {
                hour: 0,
                min: 0,
                sec: 0,
                milli: 0,
            },
            dmf: 0,
            dc: 0,
            cfg_no_rollup: 0,
        };

        move_penpos(&mut ctx, 10, 20);
        assert_eq!(ctx.current_state.layout_state.cursor_pos.x, 20);
        assert_eq!(ctx.current_state.layout_state.cursor_pos.y, 10);
    }

    #[test]
    fn test_set_position() {
        let mut ctx = ISDBSubContext {
            nb_char: 0,
            nb_line: 0,
            timestamp: 0,
            prev_timestamp: 0,
            text_list_head: ListHead {
                next: ptr::null_mut(),
                prev: ptr::null_mut(),
            },
            buffered_text: ListHead {
                next: ptr::null_mut(),
                prev: ptr::null_mut(),
            },
            current_state: ISDBSubState {
                auto_display: 0,
                rollup_mode: 0,
                need_init: 0,
                clut_high_idx: 0,
                fg_color: 0,
                bg_color: 0,
                hfg_color: 0,
                hbg_color: 0,
                mat_color: 0,
                raster_color: 0,
                layout_state: ISDBSubLayout {
                    format: WritingFormat::HorizontalStdDensity,
                    display_area: DispArea {
                        x: 0,
                        y: 0,
                        w: 0,
                        h: 0,
                    },
                    font_size: 36,
                    font_scale: FScale {
                        fscx: 100,
                        fscy: 100,
                    },
                    cell_spacing: Spacing { col: 0, row: 0 },
                    cursor_pos: ISDBPos { x: 0, y: 0 },
                    ccc: IsdbCCComposition::None,
                    acps: [0; 2],
                },
            },
            tmd: IsdbTmd::Free,
            nb_lang: 0,
            offset_time: OffsetTime {
                hour: 0,
                min: 0,
                sec: 0,
                milli: 0,
            },
            dmf: 0,
            dc: 0,
            cfg_no_rollup: 0,
        };

        set_position(&mut ctx, 5, 10);
        assert_eq!(ctx.current_state.layout_state.cursor_pos.x, 720);
        assert_eq!(ctx.current_state.layout_state.cursor_pos.y, 216);
    }

    #[test]
    fn test_get_csi_params() {
        let q = b"12;34 ";
        let mut p1 = None;
        let mut p2 = None;
        let result = get_csi_params(q, &mut p1, &mut p2).unwrap();
        assert_eq!(result, 6);
        assert_eq!(p1, Some(12));
        assert_eq!(p2, Some(34));
    }

    #[test]
    fn test_isdb_command_log() {
        // This function is a no-op, so we just call it to ensure it doesn't panic
        isdb_command_log("Test", format_args!(""));
    }

    #[test]
    fn test_parse_csi() {
        let mut ctx = ISDBSubContext {
            nb_char: 0,
            nb_line: 0,
            timestamp: 0,
            prev_timestamp: 0,
            text_list_head: ListHead {
                next: ptr::null_mut(),
                prev: ptr::null_mut(),
            },
            buffered_text: ListHead {
                next: ptr::null_mut(),
                prev: ptr::null_mut(),
            },
            current_state: ISDBSubState {
                auto_display: 0,
                rollup_mode: 0,
                need_init: 0,
                clut_high_idx: 0,
                fg_color: 0,
                bg_color: 0,
                hfg_color: 0,
                hbg_color: 0,
                mat_color: 0,
                raster_color: 0,
                layout_state: ISDBSubLayout {
                    format: WritingFormat::HorizontalStdDensity,
                    display_area: DispArea {
                        x: 0,
                        y: 0,
                        w: 0,
                        h: 0,
                    },
                    font_size: 36,
                    font_scale: FScale {
                        fscx: 100,
                        fscy: 100,
                    },
                    cell_spacing: Spacing { col: 0, row: 0 },
                    cursor_pos: ISDBPos { x: 0, y: 0 },
                    ccc: IsdbCCComposition::None,
                    acps: [0; 2],
                },
            },
            tmd: IsdbTmd::Free,
            nb_lang: 0,
            offset_time: OffsetTime {
                hour: 0,
                min: 0,
                sec: 0,
                milli: 0,
            },
            dmf: 0,
            dc: 0,
            cfg_no_rollup: 0,
        };

        let buf = b"53 20";
        let result = parse_csi(&mut ctx, buf);
        assert_eq!(result, 4);
        assert_eq!(
            ctx.current_state.layout_state.format,
            WritingFormat::HorizontalStdDensity
        );
    }

    #[test]
    fn test_parse_command() {
        let mut ctx = ISDBSubContext {
            nb_char: 0,
            nb_line: 0,
            timestamp: 0,
            prev_timestamp: 0,
            text_list_head: ListHead {
                next: ptr::null_mut(),
                prev: ptr::null_mut(),
            },
            buffered_text: ListHead {
                next: ptr::null_mut(),
                prev: ptr::null_mut(),
            },
            current_state: ISDBSubState {
                auto_display: 0,
                rollup_mode: 0,
                need_init: 0,
                clut_high_idx: 0,
                fg_color: 0,
                bg_color: 0,
                hfg_color: 0,
                hbg_color: 0,
                mat_color: 0,
                raster_color: 0,
                layout_state: ISDBSubLayout {
                    format: WritingFormat::HorizontalStdDensity,
                    display_area: DispArea {
                        x: 0,
                        y: 0,
                        w: 0,
                        h: 0,
                    },
                    font_size: 36,
                    font_scale: FScale {
                        fscx: 100,
                        fscy: 100,
                    },
                    cell_spacing: Spacing { col: 0, row: 0 },
                    cursor_pos: ISDBPos { x: 0, y: 0 },
                    ccc: IsdbCCComposition::None,
                    acps: [0; 2],
                },
            },
            tmd: IsdbTmd::Free,
            nb_lang: 0,
            offset_time: OffsetTime {
                hour: 0,
                min: 0,
                sec: 0,
                milli: 0,
            },
            dmf: 0,
            dc: 0,
            cfg_no_rollup: 0,
        };

        let buf = b"\x91\x20";
        let result = parse_command(&mut ctx, buf);
        assert_eq!(result, 2);
        assert_eq!(ctx.current_state.fg_color, DEFAULT_CLUT[1]);
    }

    #[test]
    fn test_parse_caption_management_data() {
        let mut ctx = ISDBSubContext {
            nb_char: 0,
            nb_line: 0,
            timestamp: 0,
            prev_timestamp: 0,
            text_list_head: ListHead {
                next: ptr::null_mut(),
                prev: ptr::null_mut(),
            },
            buffered_text: ListHead {
                next: ptr::null_mut(),
                prev: ptr::null_mut(),
            },
            current_state: ISDBSubState {
                auto_display: 0,
                rollup_mode: 0,
                need_init: 0,
                clut_high_idx: 0,
                fg_color: 0,
                bg_color: 0,
                hfg_color: 0,
                hbg_color: 0,
                mat_color: 0,
                raster_color: 0,
                layout_state: ISDBSubLayout {
                    format: WritingFormat::HorizontalStdDensity,
                    display_area: DispArea {
                        x: 0,
                        y: 0,
                        w: 0,
                        h: 0,
                    },
                    font_size: 36,
                    font_scale: FScale {
                        fscx: 100,
                        fscy: 100,
                    },
                    cell_spacing: Spacing { col: 0, row: 0 },
                    cursor_pos: ISDBPos { x: 0, y: 0 },
                    ccc: IsdbCCComposition::None,
                    acps: [0; 2],
                },
            },
            tmd: IsdbTmd::Free,
            nb_lang: 0,
            offset_time: OffsetTime {
                hour: 0,
                min: 0,
                sec: 0,
                milli: 0,
            },
            dmf: 0,
            dc: 0,
            cfg_no_rollup: 0,
        };

        let buf = b"\x80\x12\x34\x56\x78\x01\x41\x42\x43\x10";
        let result = parse_caption_management_data(&mut ctx, buf);
        assert_eq!(result, 10);
        assert_eq!(ctx.tmd, IsdbTmd::OffsetTime);
        assert_eq!(ctx.offset_time.hour, 1);
        assert_eq!(ctx.offset_time.min, 2);
        assert_eq!(ctx.offset_time.sec, 3);
        assert_eq!(ctx.offset_time.milli, 456);
        assert_eq!(ctx.nb_lang, 1);
        assert_eq!(ctx.dmf, 0x10);
    }

    #[test]
    fn test_add_cc_sub_text() {
        let mut sub = CcSubtitle {
            data: ptr::null_mut(),
            datatype: Subdatatype::Generic,
            nb_data: 0,
            type_: Subtype::Text,
            enc_type: CcxEncodingType::Utf8,
            start_time: 0,
            end_time: 0,
            flags: 0,
            lang_index: 0,
            got_output: 0,
            mode: [0; 5],
            info: [0; 4],
            time_out: 0,
            next: ptr::null_mut(),
            prev: ptr::null_mut(),
        };

        let result = add_cc_sub_text(
            &mut sub,
            "Hello, world!",
            1000,
            2000,
            Some("info"),
            Some("mode"),
            CcxEncodingType::Utf8,
        );
        assert_eq!(result, 0);
        assert_eq!(sub.nb_data, 13);
        assert_eq!(sub.start_time, 1000);
        assert_eq!(sub.end_time, 2000);
        assert_eq!(
            unsafe { CStr::from_ptr(sub.data as *const c_char).to_str().unwrap() },
            "Hello, world!"
        );
        assert_eq!(sub.info, [105, 110, 102, 111]); // "info" in ASCII
        assert_eq!(sub.mode, [109, 111, 100, 101, 0]); // "mode" in ASCII
        assert_eq!(sub.got_output, 1);
    }

    #[test]
    fn test_parse_statement() {
        let mut ctx = ISDBSubContext {
            nb_char: 0,
            nb_line: 0,
            timestamp: 0,
            prev_timestamp: 0,
            text_list_head: ListHead {
                next: ptr::null_mut(),
                prev: ptr::null_mut(),
            },
            buffered_text: ListHead {
                next: ptr::null_mut(),
                prev: ptr::null_mut(),
            },
            current_state: ISDBSubState {
                auto_display: 0,
                rollup_mode: 0,
                need_init: 0,
                clut_high_idx: 0,
                fg_color: 0,
                bg_color: 0,
                hfg_color: 0,
                hbg_color: 0,
                mat_color: 0,
                raster_color: 0,
                layout_state: ISDBSubLayout {
                    format: WritingFormat::HorizontalStdDensity,
                    display_area: DispArea {
                        x: 0,
                        y: 0,
                        w: 0,
                        h: 0,
                    },
                    font_size: 36,
                    font_scale: FScale {
                        fscx: 100,
                        fscy: 100,
                    },
                    cell_spacing: Spacing { col: 0, row: 0 },
                    cursor_pos: ISDBPos { x: 0, y: 0 },
                    ccc: IsdbCCComposition::None,
                    acps: [0; 2],
                },
            },
            tmd: IsdbTmd::Free,
            nb_lang: 0,
            offset_time: OffsetTime {
                hour: 0,
                min: 0,
                sec: 0,
                milli: 0,
            },
            dmf: 0,
            dc: 0,
            cfg_no_rollup: 0,
        };

        let buf = b"\x20\x41\x42\x43";
        let result = parse_statement(&mut ctx, buf, buf.len() as i32);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_parse_data_unit() {
        let mut ctx = ISDBSubContext {
            nb_char: 0,
            nb_line: 0,
            timestamp: 0,
            prev_timestamp: 0,
            text_list_head: ListHead {
                next: ptr::null_mut(),
                prev: ptr::null_mut(),
            },
            buffered_text: ListHead {
                next: ptr::null_mut(),
                prev: ptr::null_mut(),
            },
            current_state: ISDBSubState {
                auto_display: 0,
                rollup_mode: 0,
                need_init: 0,
                clut_high_idx: 0,
                fg_color: 0,
                bg_color: 0,
                hfg_color: 0,
                hbg_color: 0,
                mat_color: 0,
                raster_color: 0,
                layout_state: ISDBSubLayout {
                    format: WritingFormat::HorizontalStdDensity,
                    display_area: DispArea {
                        x: 0,
                        y: 0,
                        w: 0,
                        h: 0,
                    },
                    font_size: 36,
                    font_scale: FScale {
                        fscx: 100,
                        fscy: 100,
                    },
                    cell_spacing: Spacing { col: 0, row: 0 },
                    cursor_pos: ISDBPos { x: 0, y: 0 },
                    ccc: IsdbCCComposition::None,
                    acps: [0; 2],
                },
            },
            tmd: IsdbTmd::Free,
            nb_lang: 0,
            offset_time: OffsetTime {
                hour: 0,
                min: 0,
                sec: 0,
                milli: 0,
            },
            dmf: 0,
            dc: 0,
            cfg_no_rollup: 0,
        };

        let buf = b"\x20\x00\x00\x03\x41\x42\x43";
        let result = parse_data_unit(&mut ctx, buf, buf.len() as i32);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_parse_caption_statement_data() {
        let mut ctx = ISDBSubContext {
            nb_char: 0,
            nb_line: 0,
            timestamp: 0,
            prev_timestamp: 0,
            text_list_head: ListHead {
                next: ptr::null_mut(),
                prev: ptr::null_mut(),
            },
            buffered_text: ListHead {
                next: ptr::null_mut(),
                prev: ptr::null_mut(),
            },
            current_state: ISDBSubState {
                auto_display: 0,
                rollup_mode: 0,
                need_init: 0,
                clut_high_idx: 0,
                fg_color: 0,
                bg_color: 0,
                hfg_color: 0,
                hbg_color: 0,
                mat_color: 0,
                raster_color: 0,
                layout_state: ISDBSubLayout {
                    format: WritingFormat::HorizontalStdDensity,
                    display_area: DispArea {
                        x: 0,
                        y: 0,
                        w: 0,
                        h: 0,
                    },
                    font_size: 36,
                    font_scale: FScale {
                        fscx: 100,
                        fscy: 100,
                    },
                    cell_spacing: Spacing { col: 0, row: 0 },
                    cursor_pos: ISDBPos { x: 0, y: 0 },
                    ccc: IsdbCCComposition::None,
                    acps: [0; 2],
                },
            },
            tmd: IsdbTmd::Free,
            nb_lang: 0,
            offset_time: OffsetTime {
                hour: 0,
                min: 0,
                sec: 0,
                milli: 0,
            },
            dmf: 0,
            dc: 0,
            cfg_no_rollup: 0,
        };

        let mut sub = CcSubtitle {
            data: ptr::null_mut(),
            datatype: Subdatatype::Generic,
            nb_data: 0,
            type_: Subtype::Text,
            enc_type: CcxEncodingType::Utf8,
            start_time: 0,
            end_time: 0,
            flags: 0,
            lang_index: 0,
            got_output: 0,
            mode: [0; 5],
            info: [0; 4],
            time_out: 0,
            next: ptr::null_mut(),
            prev: ptr::null_mut(),
        };

        let buf = b"\x80\x00\x00\x03\x41\x42\x43";
        let result = parse_caption_statement_data(&mut ctx, 0, buf, buf.len() as i32, &mut sub);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_isdb_parse_data_group() {
        let mut ctx = ISDBSubContext {
            nb_char: 0,
            nb_line: 0,
            timestamp: 0,
            prev_timestamp: 0,
            text_list_head: ListHead {
                next: ptr::null_mut(),
                prev: ptr::null_mut(),
            },
            buffered_text: ListHead {
                next: ptr::null_mut(),
                prev: ptr::null_mut(),
            },
            current_state: ISDBSubState {
                auto_display: 0,
                rollup_mode: 0,
                need_init: 0,
                clut_high_idx: 0,
                fg_color: 0,
                bg_color: 0,
                hfg_color: 0,
                hbg_color: 0,
                mat_color: 0,
                raster_color: 0,
                layout_state: ISDBSubLayout {
                    format: WritingFormat::HorizontalStdDensity,
                    display_area: DispArea {
                        x: 0,
                        y: 0,
                        w: 0,
                        h: 0,
                    },
                    font_size: 36,
                    font_scale: FScale {
                        fscx: 100,
                        fscy: 100,
                    },
                    cell_spacing: Spacing { col: 0, row: 0 },
                    cursor_pos: ISDBPos { x: 0, y: 0 },
                    ccc: IsdbCCComposition::None,
                    acps: [0; 2],
                },
            },
            tmd: IsdbTmd::Free,
            nb_lang: 0,
            offset_time: OffsetTime {
                hour: 0,
                min: 0,
                sec: 0,
                milli: 0,
            },
            dmf: 0,
            dc: 0,
            cfg_no_rollup: 0,
        };

        let mut sub = CcSubtitle {
            data: ptr::null_mut(),
            datatype: Subdatatype::Generic,
            nb_data: 0,
            type_: Subtype::Text,
            enc_type: CcxEncodingType::Utf8,
            start_time: 0,
            end_time: 0,
            flags: 0,
            lang_index: 0,
            got_output: 0,
            mode: [0; 5],
            info: [0; 4],
            time_out: 0,
            next: ptr::null_mut(),
            prev: ptr::null_mut(),
        };

        let buf = b"\x00\x00\x00\x00\x00\x00\x00\x00";
        let result = isdb_parse_data_group(&mut ctx as *mut _ as *mut c_void, buf, &mut sub);
        assert_eq!(result, 8);
    }

    #[test]
    fn test_isdbsub_decode() {
        let mut ctx = ISDBSubContext {
            nb_char: 0,
            nb_line: 0,
            timestamp: 0,
            prev_timestamp: 0,
            text_list_head: ListHead {
                next: ptr::null_mut(),
                prev: ptr::null_mut(),
            },
            buffered_text: ListHead {
                next: ptr::null_mut(),
                prev: ptr::null_mut(),
            },
            current_state: ISDBSubState {
                auto_display: 0,
                rollup_mode: 0,
                need_init: 0,
                clut_high_idx: 0,
                fg_color: 0,
                bg_color: 0,
                hfg_color: 0,
                hbg_color: 0,
                mat_color: 0,
                raster_color: 0,
                layout_state: ISDBSubLayout {
                    format: WritingFormat::HorizontalStdDensity,
                    display_area: DispArea {
                        x: 0,
                        y: 0,
                        w: 0,
                        h: 0,
                    },
                    font_size: 36,
                    font_scale: FScale {
                        fscx: 100,
                        fscy: 100,
                    },
                    cell_spacing: Spacing { col: 0, row: 0 },
                    cursor_pos: ISDBPos { x: 0, y: 0 },
                    ccc: IsdbCCComposition::None,
                    acps: [0; 2],
                },
            },
            tmd: IsdbTmd::Free,
            nb_lang: 0,
            offset_time: OffsetTime {
                hour: 0,
                min: 0,
                sec: 0,
                milli: 0,
            },
            dmf: 0,
            dc: 0,
            cfg_no_rollup: 0,
        };

        let mut dec_ctx = LibCcDecode {
            private_data: &mut ctx as *mut _ as *mut c_void,
            no_rollup: 0,
            cc_stats: ptr::null_mut(),
            saw_caption_block: 0,
            processed_enough: 0,
            context_cc608_field_1: ptr::null_mut(),
            context_cc608_field_2: ptr::null_mut(),
            noscte20: 0,
            fix_padding: 0,
            write_format: Default::default(),
            extraction_start: 0,
            extraction_end: 0,
            subs_delay: 0,
            extract: 0,
            fullbin: 0,
            dec_sub: ptr::null_mut(),
            in_bufferdatatype: 0,
            hauppauge_mode: 0,
            frames_since_last_gop: 0,
            saw_gop_header: 0,
            max_gop_length: 0,
            last_gop_length: 0,
            total_pulldownfields: 0,
            total_pulldownframes: 0,
            program_number: 0,
            list: ptr::null_mut(),
            timing: ptr::null_mut(),
            codec: ptr::null_mut(),
            has_ccdata_buffered: 0,
            is_alloc: 0,
            avc_ctx: ptr::null_mut(),
            current_hor_size: 0,
            current_vert_size: 0,
            current_aspect_ratio: 0,
            current_frame_rate: 0,
            no_bitstream_error: 0,
            saw_seqgoppic: 0,
            in_pic_data: 0,
            current_progressive_sequence: 0,
            current_pulldownfields: 0,
            temporal_reference: 0,
            picture_coding_type: 0,
            num_key_frames: 0,
            picture_structure: 0,
            repeat_first_field: 0,
            progressive_frame: 0,
            pulldownfields: 0,
            top_field_first: 0,
            stat_numuserheaders: 0,
            stat_dvdccheaders: 0,
            stat_scte20ccheaders: 0,
            stat_replay5000headers: 0,
            stat_replay4000headers: 0,
            stat_dishheaders: 0,
            stat_hdtv: 0,
            stat_divicom: 0,
            false_pict_header: 0,
            dtvcc: ptr::null_mut(),
            current_field: 0,
            maxtref: 0,
            cc_data_count: 0,
            cc_fts: ptr::null_mut(),
            cc_data_pkts: ptr::null_mut(),
            anchor_seq_number: 0,
            xds_ctx: ptr::null_mut(),
            vbi_decoder: ptr::null_mut(),
            writedata: ptr::null_mut(),
            ocr_quantmode: ptr::null_mut(),
            prev: ptr::null_mut(),
        };

        let mut sub = CcSubtitle {
            data: ptr::null_mut(),
            datatype: Subdatatype::Generic,
            nb_data: 0,
            type_: Subtype::Text,
            enc_type: CcxEncodingType::Utf8,
            start_time: 0,
            end_time: 0,
            flags: 0,
            lang_index: 0,
            got_output: 0,
            mode: [0; 5],
            info: [0; 4],
            time_out: 0,
            next: ptr::null_mut(),
            prev: ptr::null_mut(),
        };

        let buf = b"\x80\xFF\x00\x00\x00\x00\x00\x00";
        let result = isdbsub_decode(&mut dec_ctx, buf, &mut sub);
        assert_eq!(result, 1);
    }
}
