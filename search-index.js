var searchIndex = {};
searchIndex["gensokyo"] = {"doc":"","items":[[5,"efi_start","gensokyo","",null,null],[5,"abort","","",null,{"inputs":[],"output":null}]],"paths":[]};
searchIndex["efi"] = {"doc":"This crate provides a high-level interface to UEFI.","items":[[17,"PAGE_SIZE","efi","",null,null],[3,"Guid","","",null,null],[12,"0","","",0,null],[12,"1","","",0,null],[12,"2","","",0,null],[12,"3","","",0,null],[3,"BltPixel","","",null,null],[12,"blue","","",1,null],[12,"green","","",1,null],[12,"red","","",1,null],[12,"reserved","","",1,null],[3,"ModeInformation","","",null,null],[12,"version","","",2,null],[12,"horizontal_resolution","","",2,null],[12,"vertical_resolution","","",2,null],[12,"pixel_format","","",2,null],[12,"pixel_information","","",2,null],[12,"pixels_per_scan_line","","",2,null],[3,"PixelBitmask","","",null,null],[12,"red_mask","","",3,null],[12,"green_mask","","",3,null],[12,"blue_mask","","",3,null],[12,"reserved_mask","","",3,null],[4,"PixelFormat","","",null,null],[13,"RedGreenBlueReserved8BitPerColor","","",4,null],[13,"BlueGreenRedReserved8BitPerColor","","",4,null],[13,"BitMask","","",4,null],[13,"BltOnly","","",4,null],[3,"MemoryDescriptor","","Represents a UEFI memory descriptor.",null,null],[12,"type_","","",5,null],[12,"physical_start","","",5,null],[12,"virtual_start","","",5,null],[12,"number_of_pages","","",5,null],[12,"attribute","","",5,null],[4,"MemoryType","","",null,null],[13,"ReservedMemoryType","","",6,null],[13,"LoaderCode","","",6,null],[13,"LoaderData","","",6,null],[13,"BootServicesCode","","",6,null],[13,"BootServicesData","","",6,null],[13,"RuntimeServicesCode","","",6,null],[13,"RuntimeServicesData","","",6,null],[13,"ConventionalMemory","","",6,null],[13,"UnusableMemory","","",6,null],[13,"ACPIReclaimMemory","","",6,null],[13,"ACPIMemoryNVS","","",6,null],[13,"MemoryMappedIO","","",6,null],[13,"MemoryMappedIOPortSpace","","",6,null],[13,"PalCode","","",6,null],[13,"PersistentMemory","","",6,null],[13,"MaxMemoryType","","",6,null],[6,"PhysicalAddress","","",null,null],[6,"VirtualAddress","","",null,null],[3,"MemoryAttribute","","",null,null],[3,"BootServices","","UEFI boot services.",null,null],[3,"GraphicsOutput","","",null,null],[3,"SimpleTextOutput","","Provides a simple interface for displaying text.",null,null],[3,"EfiBox","","An object allocated on the UEFI heap.",null,null],[3,"MemoryMap","","Represents a UEFI memory map.",null,null],[3,"MapKey","","A memory map key returned by `BootServices::memory_map()`.",null,null],[3,"MemoryMapIter","","Iterator for `MemoryMap`.",null,null],[3,"MemoryMapMutIter","","Iterator for `MemoryMap`.",null,null],[3,"RuntimeServices","","UEFI runtime services.",null,null],[4,"Status","","",null,null],[13,"Known","","",7,null],[13,"Unknown","","",7,null],[5,"check_status","","Converts a low-level `EFI_STATUS` to a high-level `EfiResult`.",null,{"inputs":[{"name":"status"}],"output":{"name":"efiresult"}}],[5,"init","","Initializes the UEFI wrapper.",null,null],[11,"linear_frame_buffer","","Returns the physical address and size of the linear frame buffer.",8,null],[11,"current_mode","","Returns the index of the current mode.",8,null],[11,"current_mode_info","","Returns information pertaining to the current mode.",8,null],[11,"max_mode","","Returns the number of modes supported by this device. All mode numbers\nare in the range `[0, max_mode)`.",8,null],[11,"query_mode","","Queries information on the specified mode.",8,null],[11,"set_mode","","Sets the mode of this device.",8,null],[11,"fill","","Fills the rectangle with a single color.",8,null],[11,"copy_buffer_to_video","","Copies a pixel buffer to the screen.",8,null],[11,"write_str","","Write a string to the handle.",9,null],[11,"write_fmt","","Write a formatting object to the handle.",9,null],[6,"EfiResult","","",null,null],[6,"ModeNumber","","",null,null],[8,"Protocol","","",null,null],[18,"GUID","","",10,null],[11,"partial_cmp","","",7,null],[11,"lt","","",7,null],[11,"le","","",7,null],[11,"gt","","",7,null],[11,"ge","","",7,null],[11,"eq","","",7,null],[11,"ne","","",7,null],[11,"cmp","","",7,null],[11,"fmt","","",7,null],[11,"clone","","",7,null],[11,"from","","",7,{"inputs":[{"name":"usize"}],"output":{"name":"self"}}],[11,"into","","",7,null],[11,"stdout","","Returns a handle to the console output.",11,null],[11,"stderr","","Returns a handle to the console standard error.",11,null],[11,"boxed","","Places an object on the UEFI heap.",11,null],[11,"allocate","","Allocates a block of memory using the UEFI allocator. The memory is of\ntype `EfiLoaderData`.",11,null],[11,"deallocate","","Deallocates a block of memory provided by `allocate()`.",11,null],[11,"locate_protocol","","Returns the first protocol instance that matches the given protocol.",11,null],[11,"memory_map","","Retrieves a copy of the UEFI memory map.",11,null],[11,"exit_boot_services","","Terminate boot services.",11,null],[11,"get_instance","","Retrieves a copy of the boot services table, if present.",11,{"inputs":[],"output":{"name":"option"}}],[11,"from_raw","","Constructs an `EfiBox` from a raw pointer.",12,null],[11,"into_raw","","Extracts the raw pointer from an `EfiBox`.",12,null],[11,"from_raw_slice","","Constructs a boxed slice from a pointer and length.",12,null],[11,"deref","","",12,null],[11,"deref_mut","","",12,null],[11,"drop","","",12,null],[11,"fmt","","",12,null],[11,"fmt","","",12,null],[11,"fmt","","",13,null],[11,"from_raw","","Constructs a memory map.",13,null],[11,"len","","Returns the number of entries in the memory map.",13,null],[11,"iter","","Returns an iterator over the memory map.",13,null],[11,"iter_mut","","Returns a mutable iterator over the memory map.",13,null],[11,"drop","","",13,null],[11,"eq","","",14,null],[11,"ne","","",14,null],[11,"fmt","","",14,null],[11,"clone","","",14,null],[11,"fmt","","",15,null],[11,"next","","",15,null],[11,"fmt","","",16,null],[11,"next","","",16,null],[11,"partial_cmp","","",3,null],[11,"lt","","",3,null],[11,"le","","",3,null],[11,"gt","","",3,null],[11,"ge","","",3,null],[11,"eq","","",3,null],[11,"ne","","",3,null],[11,"cmp","","",3,null],[11,"fmt","","",3,null],[11,"clone","","",3,null],[11,"partial_cmp","","",4,null],[11,"eq","","",4,null],[11,"cmp","","",4,null],[11,"fmt","","",4,null],[11,"clone","","",4,null],[11,"partial_cmp","","",2,null],[11,"lt","","",2,null],[11,"le","","",2,null],[11,"gt","","",2,null],[11,"ge","","",2,null],[11,"eq","","",2,null],[11,"ne","","",2,null],[11,"cmp","","",2,null],[11,"fmt","","",2,null],[11,"clone","","",2,null],[11,"partial_cmp","","",1,null],[11,"lt","","",1,null],[11,"le","","",1,null],[11,"gt","","",1,null],[11,"ge","","",1,null],[11,"eq","","",1,null],[11,"ne","","",1,null],[11,"cmp","","",1,null],[11,"fmt","","",1,null],[11,"clone","","",1,null],[11,"partial_cmp","","",0,null],[11,"lt","","",0,null],[11,"le","","",0,null],[11,"gt","","",0,null],[11,"ge","","",0,null],[11,"eq","","",0,null],[11,"ne","","",0,null],[11,"cmp","","",0,null],[11,"fmt","","",0,null],[11,"clone","","",0,null],[11,"partial_cmp","","",6,null],[11,"eq","","",6,null],[11,"cmp","","",6,null],[11,"fmt","","",6,null],[11,"clone","","",6,null],[11,"partial_cmp","","",5,null],[11,"lt","","",5,null],[11,"le","","",5,null],[11,"gt","","",5,null],[11,"ge","","",5,null],[11,"eq","","",5,null],[11,"ne","","",5,null],[11,"cmp","","",5,null],[11,"clone","","",5,null],[11,"fmt","","",5,null],[11,"physical_end","","",5,null],[11,"hash","","",17,null],[11,"cmp","","",17,null],[11,"partial_cmp","","",17,null],[11,"lt","","",17,null],[11,"le","","",17,null],[11,"gt","","",17,null],[11,"ge","","",17,null],[11,"clone","","",17,null],[11,"eq","","",17,null],[11,"ne","","",17,null],[11,"fmt","","",17,null],[11,"empty","","Returns an empty set of flags.",17,{"inputs":[],"output":{"name":"memoryattribute"}}],[11,"all","","Returns the set containing all flags.",17,{"inputs":[],"output":{"name":"memoryattribute"}}],[11,"bits","","Returns the raw value of the flags currently stored.",17,null],[11,"from_bits","","Convert from underlying bit representation, unless that\nrepresentation contains bits that do not correspond to a flag.",17,{"inputs":[{"name":"u64"}],"output":{"name":"option"}}],[11,"from_bits_truncate","","Convert from underlying bit representation, dropping any bits\nthat do not correspond to flags.",17,{"inputs":[{"name":"u64"}],"output":{"name":"memoryattribute"}}],[11,"is_empty","","Returns `true` if no flags are currently stored.",17,null],[11,"is_all","","Returns `true` if all flags are currently set.",17,null],[11,"intersects","","Returns `true` if there are flags common to both `self` and `other`.",17,null],[11,"contains","","Returns `true` all of the flags in `other` are contained within `self`.",17,null],[11,"insert","","Inserts the specified flags in-place.",17,null],[11,"remove","","Removes the specified flags in-place.",17,null],[11,"toggle","","Toggles the specified flags in-place.",17,null],[11,"bitor","","Returns the union of the two sets of flags.",17,null],[11,"bitor_assign","","Adds the set of flags.",17,null],[11,"bitxor","","Returns the left flags, but with all the right flags toggled.",17,null],[11,"bitxor_assign","","Toggles the set of flags.",17,null],[11,"bitand","","Returns the intersection between the two sets of flags.",17,null],[11,"bitand_assign","","Disables all flags disabled in the set.",17,null],[11,"sub","","Returns the set difference of the two sets of flags.",17,null],[11,"sub_assign","","Disables all flags enabled in the set.",17,null],[11,"not","","Returns the complement of this set of flags.",17,null],[11,"extend","","",17,null],[11,"from_iter","","",17,{"inputs":[{"name":"t"}],"output":{"name":"memoryattribute"}}]],"paths":[[3,"Guid"],[3,"BltPixel"],[3,"ModeInformation"],[3,"PixelBitmask"],[4,"PixelFormat"],[3,"MemoryDescriptor"],[4,"MemoryType"],[4,"Status"],[3,"GraphicsOutput"],[3,"SimpleTextOutput"],[8,"Protocol"],[3,"BootServices"],[3,"EfiBox"],[3,"MemoryMap"],[3,"MapKey"],[3,"MemoryMapIter"],[3,"MemoryMapMutIter"],[3,"MemoryAttribute"]]};
searchIndex["efi_sys"] = {"doc":"Low-level UEFI definitions.","items":[[3,"GraphicsOutputProtocol","efi_sys","",null,null],[12,"query_mode","","",0,null],[12,"set_mode","","",0,null],[12,"blt","","",0,null],[12,"mode","","",0,null],[3,"PixelBitmask","","",null,null],[12,"red_mask","","",1,null],[12,"green_mask","","",1,null],[12,"blue_mask","","",1,null],[12,"reserved_mask","","",1,null],[3,"ModeInformation","","",null,null],[12,"version","","",2,null],[12,"horizontal_resolution","","",2,null],[12,"vertical_resolution","","",2,null],[12,"pixel_format","","",2,null],[12,"pixel_information","","",2,null],[12,"pixels_per_scan_line","","",2,null],[3,"Mode","","",null,null],[12,"max_mode","","",3,null],[12,"mode","","",3,null],[12,"info","","",3,null],[12,"frame_buffer_base","","",3,null],[12,"frame_buffer_size","","",3,null],[3,"BltPixel","","",null,null],[12,"blue","","",4,null],[12,"green","","",4,null],[12,"red","","",4,null],[12,"reserved","","",4,null],[3,"SimpleTextInputProtocol","","",null,null],[3,"SimpleTextOutputProtocol","","",null,null],[12,"reset","","",5,null],[12,"output_string","","",5,null],[3,"Guid","","",null,null],[12,"0","","",6,null],[12,"1","","",6,null],[12,"2","","",6,null],[12,"3","","",6,null],[3,"TableHeader","","",null,null],[12,"signature","","",7,null],[12,"revision","","",7,null],[12,"header_size","","",7,null],[12,"crc32","","",7,null],[12,"reserved","","",7,null],[3,"SystemTable","","",null,null],[12,"hdr","","",8,null],[12,"firmware_vendor","","",8,null],[12,"firmware_revision","","",8,null],[12,"console_in_handle","","",8,null],[12,"con_in","","",8,null],[12,"console_out_handle","","",8,null],[12,"con_out","","",8,null],[12,"standard_error_handle","","",8,null],[12,"std_err","","",8,null],[12,"runtime_services","","",8,null],[12,"boot_services","","",8,null],[12,"number_of_table_entries","","",8,null],[12,"configuration_table","","",8,null],[3,"BootServices","","",null,null],[12,"hdr","","",9,null],[12,"raise_tpl","","",9,null],[12,"restore_tpl","","",9,null],[12,"allocate_pages","","",9,null],[12,"free_pages","","",9,null],[12,"get_memory_map","","",9,null],[12,"allocate_pool","","",9,null],[12,"free_pool","","",9,null],[12,"create_event","","",9,null],[12,"set_timer","","",9,null],[12,"wait_for_event","","",9,null],[12,"signal_event","","",9,null],[12,"close_event","","",9,null],[12,"check_event","","",9,null],[12,"install_protocol_interface","","",9,null],[12,"reinstall_protocol_interface","","",9,null],[12,"uninstall_protocol_interface","","",9,null],[12,"handle_protocol","","",9,null],[12,"reserved","","",9,null],[12,"register_protocol_notify","","",9,null],[12,"locate_handle","","",9,null],[12,"locate_device_path","","",9,null],[12,"install_configuration_table","","",9,null],[12,"load_image","","",9,null],[12,"start_image","","",9,null],[12,"exit","","",9,null],[12,"unload_image","","",9,null],[12,"exit_boot_services","","",9,null],[12,"get_next_monotonic_count","","",9,null],[12,"stall","","",9,null],[12,"set_watchdog_timer","","",9,null],[12,"connect_controller","","",9,null],[12,"disconnect_controller","","",9,null],[12,"open_protocol","","",9,null],[12,"close_protocol","","",9,null],[12,"open_protocol_information","","",9,null],[12,"protocols_per_handle","","",9,null],[12,"locate_handle_buffer","","",9,null],[12,"locate_protocol","","",9,null],[12,"install_multiple_protocol_interfaces","","",9,null],[12,"uninstall_multiple_protocol_interfaces","","",9,null],[12,"calculate_crc32","","",9,null],[12,"copy_mem","","",9,null],[12,"set_mem","","",9,null],[12,"create_event_ex","","",9,null],[3,"MemoryDescriptor","","Represents a UEFI memory descriptor.",null,null],[12,"type_","","",10,null],[12,"physical_start","","",10,null],[12,"virtual_start","","",10,null],[12,"number_of_pages","","",10,null],[12,"attribute","","",10,null],[3,"MemoryAttribute","","",null,null],[3,"OpenProtocolAttribute","","",null,null],[3,"OpenProtocolInformationEntry","","",null,null],[12,"agent_handle","","",11,null],[12,"controller_handle","","",11,null],[12,"attributes","","",11,null],[12,"open_count","","",11,null],[3,"RuntimeServices","","",null,null],[12,"hdr","","",12,null],[12,"get_time","","",12,null],[12,"set_time","","",12,null],[12,"get_wakeup_time","","",12,null],[12,"set_wakeup_time","","",12,null],[12,"set_virtual_address_map","","",12,null],[12,"convert_pointer","","",12,null],[12,"get_variable","","",12,null],[12,"get_next_variable_name","","",12,null],[12,"set_variable","","",12,null],[12,"get_next_high_monotonic_count","","",12,null],[12,"reset_system","","",12,null],[12,"update_capsule","","",12,null],[12,"query_capsule_capabilities","","",12,null],[12,"query_variable_info","","",12,null],[3,"Time","","",null,null],[12,"year","","",13,null],[12,"month","","",13,null],[12,"day","","",13,null],[12,"hour","","",13,null],[12,"minute","","",13,null],[12,"second","","",13,null],[12,"pad1","","",13,null],[12,"nanosecond","","",13,null],[12,"time_zone","","",13,null],[12,"daylight","","",13,null],[12,"pad2","","",13,null],[3,"Daylight","","",null,null],[3,"TimeCapabilities","","",null,null],[3,"DebugDisposition","","",null,null],[3,"ConfigurationTable","","",null,null],[12,"vendor_guid","","",14,null],[12,"vendor_table","","",14,null],[4,"PixelFormat","","",null,null],[13,"RedGreenBlueReserved8BitPerColor","","",15,null],[13,"BlueGreenRedReserved8BitPerColor","","",15,null],[13,"BitMask","","",15,null],[13,"BltOnly","","",15,null],[4,"BltOperation","","",null,null],[13,"VideoFill","","",16,null],[13,"VideoToBltBuffer","","",16,null],[13,"BufferToVideo","","",16,null],[13,"VideoToVideo","","",16,null],[4,"Void","","",null,null],[4,"KnownStatus","","",null,null],[13,"Success","","",17,null],[13,"WarnUnknownGlyph","","",17,null],[13,"WarnDeleteFailure","","",17,null],[13,"WarnWriteFailure","","",17,null],[13,"WarnBufferTooSmall","","",17,null],[13,"WarnStaleData","","",17,null],[13,"WarnFileSystem","","",17,null],[13,"LoadError","","",17,null],[13,"InvalidParameter","","",17,null],[13,"Unsupported","","",17,null],[13,"BadBufferSize","","",17,null],[13,"BufferTooSmall","","",17,null],[13,"NotReady","","",17,null],[13,"DeviceError","","",17,null],[13,"WriteProtected","","",17,null],[13,"OutOfResources","","",17,null],[13,"VolumeCorrupted","","",17,null],[13,"VolumeFull","","",17,null],[13,"NoMedia","","",17,null],[13,"MediaChanged","","",17,null],[13,"NotFound","","",17,null],[13,"AccessDenied","","",17,null],[13,"NoResponse","","",17,null],[13,"NoMapping","","",17,null],[13,"Timeout","","",17,null],[13,"NotStarted","","",17,null],[13,"AlreadyStarted","","",17,null],[13,"Aborted","","",17,null],[13,"IcmpError","","",17,null],[13,"TftpError","","",17,null],[13,"ProtocolError","","",17,null],[13,"IncompatibleVersion","","",17,null],[13,"SecurityViolation","","",17,null],[13,"CrcError","","",17,null],[13,"EndOfMedia","","",17,null],[13,"EndOfFile","","",17,null],[13,"InvalidLanguage","","",17,null],[13,"CompromisedData","","",17,null],[13,"IpAddressConflict","","",17,null],[13,"HttpError","","",17,null],[4,"AllocateType","","",null,null],[13,"AllocateAnyPages","","",18,null],[13,"AllocateMaxAddress","","",18,null],[13,"AllocateAddress","","",18,null],[13,"MaxAllocateType","","",18,null],[4,"MemoryType","","",null,null],[13,"ReservedMemoryType","","",19,null],[13,"LoaderCode","","",19,null],[13,"LoaderData","","",19,null],[13,"BootServicesCode","","",19,null],[13,"BootServicesData","","",19,null],[13,"RuntimeServicesCode","","",19,null],[13,"RuntimeServicesData","","",19,null],[13,"ConventionalMemory","","",19,null],[13,"UnusableMemory","","",19,null],[13,"ACPIReclaimMemory","","",19,null],[13,"ACPIMemoryNVS","","",19,null],[13,"MemoryMappedIO","","",19,null],[13,"MemoryMappedIOPortSpace","","",19,null],[13,"PalCode","","",19,null],[13,"PersistentMemory","","",19,null],[13,"MaxMemoryType","","",19,null],[4,"LocateSearchType","","",null,null],[13,"AllHandles","","",20,null],[13,"ByRegisterNotify","","",20,null],[13,"ByProtocol","","",20,null],[4,"ResetType","","",null,null],[13,"Cold","","",21,null],[13,"Warm","","",21,null],[13,"Shutdown","","",21,null],[13,"PlatformSpecific","","",21,null],[11,"partial_cmp","","",1,null],[11,"lt","","",1,null],[11,"le","","",1,null],[11,"gt","","",1,null],[11,"ge","","",1,null],[11,"eq","","",1,null],[11,"ne","","",1,null],[11,"cmp","","",1,null],[11,"fmt","","",1,null],[11,"clone","","",1,null],[11,"partial_cmp","","",15,null],[11,"eq","","",15,null],[11,"cmp","","",15,null],[11,"fmt","","",15,null],[11,"clone","","",15,null],[11,"partial_cmp","","",2,null],[11,"lt","","",2,null],[11,"le","","",2,null],[11,"gt","","",2,null],[11,"ge","","",2,null],[11,"eq","","",2,null],[11,"ne","","",2,null],[11,"cmp","","",2,null],[11,"fmt","","",2,null],[11,"clone","","",2,null],[11,"partial_cmp","","",3,null],[11,"lt","","",3,null],[11,"le","","",3,null],[11,"gt","","",3,null],[11,"ge","","",3,null],[11,"eq","","",3,null],[11,"ne","","",3,null],[11,"cmp","","",3,null],[11,"fmt","","",3,null],[11,"clone","","",3,null],[11,"partial_cmp","","",4,null],[11,"lt","","",4,null],[11,"le","","",4,null],[11,"gt","","",4,null],[11,"ge","","",4,null],[11,"eq","","",4,null],[11,"ne","","",4,null],[11,"cmp","","",4,null],[11,"fmt","","",4,null],[11,"clone","","",4,null],[11,"partial_cmp","","",16,null],[11,"eq","","",16,null],[11,"cmp","","",16,null],[11,"fmt","","",16,null],[11,"clone","","",16,null],[0,"graphics_output","","",null,null],[3,"GraphicsOutputProtocol","efi_sys::graphics_output","",null,null],[12,"query_mode","","",0,null],[12,"set_mode","","",0,null],[12,"blt","","",0,null],[12,"mode","","",0,null],[3,"PixelBitmask","","",null,null],[12,"red_mask","","",1,null],[12,"green_mask","","",1,null],[12,"blue_mask","","",1,null],[12,"reserved_mask","","",1,null],[3,"ModeInformation","","",null,null],[12,"version","","",2,null],[12,"horizontal_resolution","","",2,null],[12,"vertical_resolution","","",2,null],[12,"pixel_format","","",2,null],[12,"pixel_information","","",2,null],[12,"pixels_per_scan_line","","",2,null],[3,"Mode","","",null,null],[12,"max_mode","","",3,null],[12,"mode","","",3,null],[12,"info","","",3,null],[12,"frame_buffer_base","","",3,null],[12,"frame_buffer_size","","",3,null],[3,"BltPixel","","",null,null],[12,"blue","","",4,null],[12,"green","","",4,null],[12,"red","","",4,null],[12,"reserved","","",4,null],[4,"PixelFormat","","",null,null],[13,"RedGreenBlueReserved8BitPerColor","","",15,null],[13,"BlueGreenRedReserved8BitPerColor","","",15,null],[13,"BitMask","","",15,null],[13,"BltOnly","","",15,null],[4,"BltOperation","","",null,null],[13,"VideoFill","","",16,null],[13,"VideoToBltBuffer","","",16,null],[13,"BufferToVideo","","",16,null],[13,"VideoToVideo","","",16,null],[6,"QueryMode","","",null,null],[6,"SetMode","","",null,null],[6,"Blt","","",null,null],[17,"GRAPHICS_OUTPUT_PROTOCOL_GUID","","",null,null],[0,"simple_text_input","efi_sys","",null,null],[3,"SimpleTextInputProtocol","efi_sys::simple_text_input","",null,null],[0,"simple_text_output","efi_sys","",null,null],[3,"SimpleTextOutputProtocol","efi_sys::simple_text_output","",null,null],[12,"reset","","",5,null],[12,"output_string","","",5,null],[6,"TextReset","","",null,null],[6,"TextString","","",null,null],[17,"SIMPLE_TEXT_OUTPUT_GUID","","",null,null],[6,"QueryMode","efi_sys","",null,null],[6,"SetMode","","",null,null],[6,"Blt","","",null,null],[6,"TextReset","","",null,null],[6,"TextString","","",null,null],[6,"Status","","",null,null],[6,"Handle","","",null,null],[6,"RaiseTpl","","",null,null],[6,"Tpl","","",null,null],[6,"RestoreTpl","","",null,null],[6,"AllocatePages","","",null,null],[6,"FreePages","","",null,null],[6,"GetMemoryMap","","",null,null],[6,"AllocatePool","","",null,null],[6,"FreePool","","",null,null],[6,"Exit","","",null,null],[6,"ExitBootServices","","",null,null],[6,"GetNextMonotonicCount","","",null,null],[6,"Stall","","",null,null],[6,"SetWatchdogTimer","","",null,null],[6,"OpenProtocol","","",null,null],[6,"CloseProtocol","","",null,null],[6,"OpenProtocolInformation","","",null,null],[6,"ProtocolsPerHandle","","",null,null],[6,"LocateHandleBuffer","","",null,null],[6,"LocateProtocol","","",null,null],[6,"PhysicalAddress","","",null,null],[6,"VirtualAddress","","",null,null],[6,"GetTime","","",null,null],[6,"SetTime","","",null,null],[6,"GetWakeupTime","","",null,null],[6,"SetWakeupTime","","",null,null],[6,"SetVirtualAddressMap","","",null,null],[6,"ConvertPointer","","",null,null],[6,"ResetSystem","","",null,null],[17,"GRAPHICS_OUTPUT_PROTOCOL_GUID","","",null,null],[17,"SIMPLE_TEXT_OUTPUT_GUID","","",null,null],[17,"PAGE_SIZE","","",null,null],[17,"MAX_BIT","","",null,null],[17,"MEMORY_DESCRIPTOR_VERSION","","",null,null],[17,"MEMORY_UC","","",null,null],[17,"MEMORY_WC","","",null,null],[17,"MEMORY_WT","","",null,null],[17,"MEMORY_WB","","",null,null],[17,"MEMORY_UCE","","",null,null],[17,"MEMORY_WP","","",null,null],[17,"MEMORY_RP","","",null,null],[17,"MEMORY_XP","","",null,null],[17,"MEMORY_NV","","",null,null],[17,"MEMORY_MORE_RELIABLE","","",null,null],[17,"MEMORY_RO","","",null,null],[17,"MEMORY_RUNTIME","","",null,null],[17,"BY_HANDLE_PROTOCOL","","",null,null],[17,"GET_PROTOCOL","","",null,null],[17,"TEST_PROTOCOL","","",null,null],[17,"BY_CHILD_CONTROLLER","","",null,null],[17,"BY_DRIVER","","",null,null],[17,"EXCLUSIVE","","",null,null],[17,"ADJUST_DAYLIGHT","","",null,null],[17,"IN_DAYLIGHT","","",null,null],[17,"UNSPECIFIED_TIMEZONE","","",null,null],[17,"OPTIONAL_PTR","","",null,null],[11,"fmt","","",22,null],[11,"partial_cmp","","",6,null],[11,"lt","","",6,null],[11,"le","","",6,null],[11,"gt","","",6,null],[11,"ge","","",6,null],[11,"eq","","",6,null],[11,"ne","","",6,null],[11,"cmp","","",6,null],[11,"fmt","","",6,null],[11,"clone","","",6,null],[11,"partial_cmp","","",17,null],[11,"eq","","",17,null],[11,"cmp","","",17,null],[11,"fmt","","",17,null],[11,"clone","","",17,null],[11,"try_from","","",17,{"inputs":[{"name":"usize"}],"output":{"name":"result"}}],[11,"into","","",17,null],[11,"fmt","","",7,null],[11,"partial_cmp","","",18,null],[11,"eq","","",18,null],[11,"cmp","","",18,null],[11,"fmt","","",18,null],[11,"clone","","",18,null],[11,"partial_cmp","","",19,null],[11,"eq","","",19,null],[11,"cmp","","",19,null],[11,"fmt","","",19,null],[11,"clone","","",19,null],[11,"partial_cmp","","",10,null],[11,"lt","","",10,null],[11,"le","","",10,null],[11,"gt","","",10,null],[11,"ge","","",10,null],[11,"eq","","",10,null],[11,"ne","","",10,null],[11,"cmp","","",10,null],[11,"clone","","",10,null],[11,"fmt","","",10,null],[11,"physical_end","","",10,null],[11,"hash","","",23,null],[11,"cmp","","",23,null],[11,"partial_cmp","","",23,null],[11,"lt","","",23,null],[11,"le","","",23,null],[11,"gt","","",23,null],[11,"ge","","",23,null],[11,"clone","","",23,null],[11,"eq","","",23,null],[11,"ne","","",23,null],[11,"fmt","","",23,null],[11,"empty","","Returns an empty set of flags.",23,{"inputs":[],"output":{"name":"memoryattribute"}}],[11,"all","","Returns the set containing all flags.",23,{"inputs":[],"output":{"name":"memoryattribute"}}],[11,"bits","","Returns the raw value of the flags currently stored.",23,null],[11,"from_bits","","Convert from underlying bit representation, unless that\nrepresentation contains bits that do not correspond to a flag.",23,{"inputs":[{"name":"u64"}],"output":{"name":"option"}}],[11,"from_bits_truncate","","Convert from underlying bit representation, dropping any bits\nthat do not correspond to flags.",23,{"inputs":[{"name":"u64"}],"output":{"name":"memoryattribute"}}],[11,"is_empty","","Returns `true` if no flags are currently stored.",23,null],[11,"is_all","","Returns `true` if all flags are currently set.",23,null],[11,"intersects","","Returns `true` if there are flags common to both `self` and `other`.",23,null],[11,"contains","","Returns `true` all of the flags in `other` are contained within `self`.",23,null],[11,"insert","","Inserts the specified flags in-place.",23,null],[11,"remove","","Removes the specified flags in-place.",23,null],[11,"toggle","","Toggles the specified flags in-place.",23,null],[11,"bitor","","Returns the union of the two sets of flags.",23,null],[11,"bitor_assign","","Adds the set of flags.",23,null],[11,"bitxor","","Returns the left flags, but with all the right flags toggled.",23,null],[11,"bitxor_assign","","Toggles the set of flags.",23,null],[11,"bitand","","Returns the intersection between the two sets of flags.",23,null],[11,"bitand_assign","","Disables all flags disabled in the set.",23,null],[11,"sub","","Returns the set difference of the two sets of flags.",23,null],[11,"sub_assign","","Disables all flags enabled in the set.",23,null],[11,"not","","Returns the complement of this set of flags.",23,null],[11,"extend","","",23,null],[11,"from_iter","","",23,{"inputs":[{"name":"t"}],"output":{"name":"memoryattribute"}}],[11,"hash","","",24,null],[11,"cmp","","",24,null],[11,"partial_cmp","","",24,null],[11,"lt","","",24,null],[11,"le","","",24,null],[11,"gt","","",24,null],[11,"ge","","",24,null],[11,"clone","","",24,null],[11,"eq","","",24,null],[11,"ne","","",24,null],[11,"fmt","","",24,null],[11,"empty","","Returns an empty set of flags.",24,{"inputs":[],"output":{"name":"openprotocolattribute"}}],[11,"all","","Returns the set containing all flags.",24,{"inputs":[],"output":{"name":"openprotocolattribute"}}],[11,"bits","","Returns the raw value of the flags currently stored.",24,null],[11,"from_bits","","Convert from underlying bit representation, unless that\nrepresentation contains bits that do not correspond to a flag.",24,{"inputs":[{"name":"u32"}],"output":{"name":"option"}}],[11,"from_bits_truncate","","Convert from underlying bit representation, dropping any bits\nthat do not correspond to flags.",24,{"inputs":[{"name":"u32"}],"output":{"name":"openprotocolattribute"}}],[11,"is_empty","","Returns `true` if no flags are currently stored.",24,null],[11,"is_all","","Returns `true` if all flags are currently set.",24,null],[11,"intersects","","Returns `true` if there are flags common to both `self` and `other`.",24,null],[11,"contains","","Returns `true` all of the flags in `other` are contained within `self`.",24,null],[11,"insert","","Inserts the specified flags in-place.",24,null],[11,"remove","","Removes the specified flags in-place.",24,null],[11,"toggle","","Toggles the specified flags in-place.",24,null],[11,"bitor","","Returns the union of the two sets of flags.",24,null],[11,"bitor_assign","","Adds the set of flags.",24,null],[11,"bitxor","","Returns the left flags, but with all the right flags toggled.",24,null],[11,"bitxor_assign","","Toggles the set of flags.",24,null],[11,"bitand","","Returns the intersection between the two sets of flags.",24,null],[11,"bitand_assign","","Disables all flags disabled in the set.",24,null],[11,"sub","","Returns the set difference of the two sets of flags.",24,null],[11,"sub_assign","","Disables all flags enabled in the set.",24,null],[11,"not","","Returns the complement of this set of flags.",24,null],[11,"extend","","",24,null],[11,"from_iter","","",24,{"inputs":[{"name":"t"}],"output":{"name":"openprotocolattribute"}}],[11,"partial_cmp","","",11,null],[11,"lt","","",11,null],[11,"le","","",11,null],[11,"gt","","",11,null],[11,"ge","","",11,null],[11,"eq","","",11,null],[11,"ne","","",11,null],[11,"cmp","","",11,null],[11,"fmt","","",11,null],[11,"clone","","",11,null],[11,"partial_cmp","","",20,null],[11,"eq","","",20,null],[11,"cmp","","",20,null],[11,"fmt","","",20,null],[11,"clone","","",20,null],[11,"partial_cmp","","",13,null],[11,"lt","","",13,null],[11,"le","","",13,null],[11,"gt","","",13,null],[11,"ge","","",13,null],[11,"eq","","",13,null],[11,"ne","","",13,null],[11,"cmp","","",13,null],[11,"fmt","","",13,null],[11,"clone","","",13,null],[11,"hash","","",25,null],[11,"cmp","","",25,null],[11,"partial_cmp","","",25,null],[11,"lt","","",25,null],[11,"le","","",25,null],[11,"gt","","",25,null],[11,"ge","","",25,null],[11,"clone","","",25,null],[11,"eq","","",25,null],[11,"ne","","",25,null],[11,"fmt","","",25,null],[11,"empty","","Returns an empty set of flags.",25,{"inputs":[],"output":{"name":"daylight"}}],[11,"all","","Returns the set containing all flags.",25,{"inputs":[],"output":{"name":"daylight"}}],[11,"bits","","Returns the raw value of the flags currently stored.",25,null],[11,"from_bits","","Convert from underlying bit representation, unless that\nrepresentation contains bits that do not correspond to a flag.",25,{"inputs":[{"name":"u8"}],"output":{"name":"option"}}],[11,"from_bits_truncate","","Convert from underlying bit representation, dropping any bits\nthat do not correspond to flags.",25,{"inputs":[{"name":"u8"}],"output":{"name":"daylight"}}],[11,"is_empty","","Returns `true` if no flags are currently stored.",25,null],[11,"is_all","","Returns `true` if all flags are currently set.",25,null],[11,"intersects","","Returns `true` if there are flags common to both `self` and `other`.",25,null],[11,"contains","","Returns `true` all of the flags in `other` are contained within `self`.",25,null],[11,"insert","","Inserts the specified flags in-place.",25,null],[11,"remove","","Removes the specified flags in-place.",25,null],[11,"toggle","","Toggles the specified flags in-place.",25,null],[11,"bitor","","Returns the union of the two sets of flags.",25,null],[11,"bitor_assign","","Adds the set of flags.",25,null],[11,"bitxor","","Returns the left flags, but with all the right flags toggled.",25,null],[11,"bitxor_assign","","Toggles the set of flags.",25,null],[11,"bitand","","Returns the intersection between the two sets of flags.",25,null],[11,"bitand_assign","","Disables all flags disabled in the set.",25,null],[11,"sub","","Returns the set difference of the two sets of flags.",25,null],[11,"sub_assign","","Disables all flags enabled in the set.",25,null],[11,"not","","Returns the complement of this set of flags.",25,null],[11,"extend","","",25,null],[11,"from_iter","","",25,{"inputs":[{"name":"t"}],"output":{"name":"daylight"}}],[11,"partial_cmp","","",26,null],[11,"lt","","",26,null],[11,"le","","",26,null],[11,"gt","","",26,null],[11,"ge","","",26,null],[11,"eq","","",26,null],[11,"ne","","",26,null],[11,"cmp","","",26,null],[11,"fmt","","",26,null],[11,"clone","","",26,null],[11,"hash","","",27,null],[11,"cmp","","",27,null],[11,"partial_cmp","","",27,null],[11,"lt","","",27,null],[11,"le","","",27,null],[11,"gt","","",27,null],[11,"ge","","",27,null],[11,"clone","","",27,null],[11,"eq","","",27,null],[11,"ne","","",27,null],[11,"fmt","","",27,null],[11,"empty","","Returns an empty set of flags.",27,{"inputs":[],"output":{"name":"debugdisposition"}}],[11,"all","","Returns the set containing all flags.",27,{"inputs":[],"output":{"name":"debugdisposition"}}],[11,"bits","","Returns the raw value of the flags currently stored.",27,null],[11,"from_bits","","Convert from underlying bit representation, unless that\nrepresentation contains bits that do not correspond to a flag.",27,{"inputs":[{"name":"usize"}],"output":{"name":"option"}}],[11,"from_bits_truncate","","Convert from underlying bit representation, dropping any bits\nthat do not correspond to flags.",27,{"inputs":[{"name":"usize"}],"output":{"name":"debugdisposition"}}],[11,"is_empty","","Returns `true` if no flags are currently stored.",27,null],[11,"is_all","","Returns `true` if all flags are currently set.",27,null],[11,"intersects","","Returns `true` if there are flags common to both `self` and `other`.",27,null],[11,"contains","","Returns `true` all of the flags in `other` are contained within `self`.",27,null],[11,"insert","","Inserts the specified flags in-place.",27,null],[11,"remove","","Removes the specified flags in-place.",27,null],[11,"toggle","","Toggles the specified flags in-place.",27,null],[11,"bitor","","Returns the union of the two sets of flags.",27,null],[11,"bitor_assign","","Adds the set of flags.",27,null],[11,"bitxor","","Returns the left flags, but with all the right flags toggled.",27,null],[11,"bitxor_assign","","Toggles the set of flags.",27,null],[11,"bitand","","Returns the intersection between the two sets of flags.",27,null],[11,"bitand_assign","","Disables all flags disabled in the set.",27,null],[11,"sub","","Returns the set difference of the two sets of flags.",27,null],[11,"sub_assign","","Disables all flags enabled in the set.",27,null],[11,"not","","Returns the complement of this set of flags.",27,null],[11,"extend","","",27,null],[11,"from_iter","","",27,{"inputs":[{"name":"t"}],"output":{"name":"debugdisposition"}}],[11,"partial_cmp","","",21,null],[11,"eq","","",21,null],[11,"cmp","","",21,null],[11,"fmt","","",21,null],[11,"clone","","",21,null],[11,"fmt","","",14,null]],"paths":[[3,"GraphicsOutputProtocol"],[3,"PixelBitmask"],[3,"ModeInformation"],[3,"Mode"],[3,"BltPixel"],[3,"SimpleTextOutputProtocol"],[3,"Guid"],[3,"TableHeader"],[3,"SystemTable"],[3,"BootServices"],[3,"MemoryDescriptor"],[3,"OpenProtocolInformationEntry"],[3,"RuntimeServices"],[3,"Time"],[3,"ConfigurationTable"],[4,"PixelFormat"],[4,"BltOperation"],[4,"KnownStatus"],[4,"AllocateType"],[4,"MemoryType"],[4,"LocateSearchType"],[4,"ResetType"],[4,"Void"],[3,"MemoryAttribute"],[3,"OpenProtocolAttribute"],[3,"Daylight"],[3,"TimeCapabilities"],[3,"DebugDisposition"]]};
searchIndex["rlibc"] = {"doc":"A bare-metal library supplying functions rustc may lower code to","items":[[5,"memcpy","rlibc","",null,null],[5,"memmove","","",null,null],[5,"memset","","",null,null],[5,"memcmp","","",null,null]],"paths":[]};
searchIndex["spin"] = {"doc":"Synchronization primitives based on spinning","items":[[3,"Mutex","spin","This type provides MUTual EXclusion based on spinning.",null,null],[3,"MutexGuard","","A guard to which the protected data can be accessed",null,null],[3,"RwLock","","A reader-writer lock",null,null],[3,"RwLockReadGuard","","A guard to which the protected data can be read",null,null],[3,"RwLockWriteGuard","","A guard to which the protected data can be written",null,null],[3,"Once","","A synchronization primitive which can be used to run a one-time global\ninitialization. Unlike its std equivalent, this is generalized so that The\nclosure returns a value and it is stored. Once therefore acts something like\n1a future, too.",null,null],[11,"new","","Creates a new spinlock wrapping the supplied data.",0,{"inputs":[{"name":"t"}],"output":{"name":"mutex"}}],[11,"into_inner","","Consumes this mutex, returning the underlying data.",0,null],[11,"lock","","Locks the spinlock and returns a guard.",0,null],[11,"try_lock","","Tries to lock the mutex. If it is already locked, it will return None. Otherwise it returns\na guard within Some.",0,null],[11,"fmt","","",0,null],[11,"default","","",0,{"inputs":[],"output":{"name":"mutex"}}],[11,"deref","","",1,null],[11,"deref_mut","","",1,null],[11,"drop","","The dropping of the MutexGuard will release the lock it was created from.",1,null],[11,"new","","Creates a new spinlock wrapping the supplied data.",2,{"inputs":[{"name":"t"}],"output":{"name":"rwlock"}}],[11,"into_inner","","Consumes this `RwLock`, returning the underlying data.",2,null],[11,"read","","Locks this rwlock with shared read access, blocking the current thread\nuntil it can be acquired.",2,null],[11,"try_read","","Attempt to acquire this lock with shared read access.",2,null],[11,"write","","Lock this rwlock with exclusive write access, blocking the current\nthread until it can be acquired.",2,null],[11,"try_write","","Attempt to lock this rwlock with exclusive write access.",2,null],[11,"fmt","","",2,null],[11,"default","","",2,{"inputs":[],"output":{"name":"rwlock"}}],[11,"deref","","",3,null],[11,"deref","","",4,null],[11,"deref_mut","","",4,null],[11,"drop","","",3,null],[11,"drop","","",4,null],[11,"new","","Creates a new `Once` value.",5,{"inputs":[],"output":{"name":"once"}}],[11,"call_once","","Performs an initialization routine once and only once. The given closure\nwill be executed if this is the first time `call_once` has been called,\nand otherwise the routine will *not* be invoked.",5,null],[11,"try","","Returns a pointer iff the `Once` was previously initialized",5,null],[11,"wait","","Like try, but will spin if the `Once` is in the process of being\ninitialized",5,null]],"paths":[[3,"Mutex"],[3,"MutexGuard"],[3,"RwLock"],[3,"RwLockReadGuard"],[3,"RwLockWriteGuard"],[3,"Once"]]};
searchIndex["bitflags"] = {"doc":"A typesafe bitmask flag generator.","items":[[14,"bitflags!","bitflags","The `bitflags!` macro generates a `struct` that holds a set of C-style\nbitmask flags. It is useful for creating typesafe wrappers for C APIs.",null,null]],"paths":[]};
initSearch(searchIndex);
