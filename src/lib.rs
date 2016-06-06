#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(improper_ctypes)]
#![allow(dead_code)]

extern crate libc;

use ::std::default::Default;


#[cfg(target_pointer_width = "32")]
pub type SQInteger = ::libc::int32_t;
#[cfg(target_pointer_width = "32")]
pub type SQUnsignedInteger = ::libc::uint32_t;

#[cfg(target_pointer_width = "64")]
pub type SQInteger = ::libc::int64_t;
#[cfg(target_pointer_width = "64")]
pub type SQUnsignedInteger = ::libc::uint64_t;

pub type SQInt32 = ::libc::int32_t;
pub type SQUnsignedInteger32 = ::libc::uint32_t;

pub type SQHash = SQUnsignedInteger;

#[cfg(feature = "use_double")]
pub type SQFloat = ::libc::c_double;
#[cfg(not(feature = "use_double"))]
pub type SQFloat = ::libc::c_float;

pub type SQRawObjectVal = ::libc::int64_t;
pub type SQUserPointer = *mut ::libc::c_void;
pub type SQBool = SQUnsignedInteger;

#[cfg(not(feature = "use_unicode"))]
pub type SQChar = ::libc::c_char;
#[cfg(feature = "use_unicode")]
pub type SQChar = ::libc::uint16_t;

pub type SQRESULT = SQInteger;

#[repr(C)]
pub struct SQVM;
pub type HSQUIRRELVM = *mut SQVM;


const SQOBJECT_REF_COUNTED: u32 = 0x08000000;
const SQOBJECT_NUMERIC: u32 =     0x04000000;
const SQOBJECT_DELEGABLE: u32 =   0x02000000;
const SQOBJECT_CANBEFALSE: u32 =  0x01000000;

const _RT_MASK: u32 =          0x00FFFFFF;
const _RT_NULL: u32 =          0x00000001;
const _RT_INTEGER: u32 =       0x00000002;
const _RT_FLOAT: u32 =         0x00000004;
const _RT_BOOL: u32 =          0x00000008;
const _RT_STRING: u32 =        0x00000010;
const _RT_TABLE: u32 =         0x00000020;
const _RT_ARRAY: u32 =         0x00000040;
const _RT_USERDATA: u32 =      0x00000080;
const _RT_CLOSURE: u32 =       0x00000100;
const _RT_NATIVECLOSURE: u32 = 0x00000200;
const _RT_GENERATOR: u32 =     0x00000400;
const _RT_USERPOINTER: u32 =   0x00000800;
const _RT_THREAD: u32 =        0x00001000;
const _RT_FUNCPROTO: u32 =     0x00002000;
const _RT_CLASS: u32 =         0x00004000;
const _RT_INSTANCE: u32 =      0x00008000;
const _RT_WEAKREF: u32 =       0x00010000;
const _RT_OUTER: u32 =         0x00020000;

pub fn _RAW_TYPE(_type: u32) -> u32 { _type & _RT_MASK }

#[derive(Copy, Clone)]
#[repr(u32)]
pub enum SQObjectType {
    OT_NULL =          _RT_NULL | SQOBJECT_CANBEFALSE,
    OT_INTEGER =       _RT_INTEGER | SQOBJECT_NUMERIC | SQOBJECT_CANBEFALSE,
    OT_FLOAT =         _RT_FLOAT | SQOBJECT_NUMERIC | SQOBJECT_CANBEFALSE,
    OT_BOOL =          _RT_BOOL | SQOBJECT_CANBEFALSE,
    OT_STRING =        _RT_STRING | SQOBJECT_REF_COUNTED,
    OT_TABLE =         _RT_TABLE | SQOBJECT_REF_COUNTED | SQOBJECT_DELEGABLE,
    OT_ARRAY =         _RT_ARRAY | SQOBJECT_REF_COUNTED,
    OT_USERDATA =      _RT_USERDATA | SQOBJECT_REF_COUNTED | SQOBJECT_DELEGABLE,
    OT_CLOSURE =       _RT_CLOSURE | SQOBJECT_REF_COUNTED,
    OT_NATIVECLOSURE = _RT_NATIVECLOSURE | SQOBJECT_REF_COUNTED,
    OT_GENERATOR =     _RT_GENERATOR | SQOBJECT_REF_COUNTED,
    OT_USERPOINTER =   _RT_USERPOINTER,
    OT_THREAD =        _RT_THREAD | SQOBJECT_REF_COUNTED ,
    OT_CLASS =         _RT_CLASS | SQOBJECT_REF_COUNTED,
    OT_INSTANCE =      _RT_INSTANCE | SQOBJECT_REF_COUNTED | SQOBJECT_DELEGABLE,
    OT_WEAKREF =       _RT_WEAKREF | SQOBJECT_REF_COUNTED,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SQObjectValue {
    internal: [u64; 1usize],
}

impl Default for SQObjectValue {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SQObject {
    pub _type: SQObjectType,
    pub _unVal: SQObjectValue,
}

impl Default for SQObject {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SQMemberHandle {
    pub _static: SQBool,
    pub _index: SQInteger,
}

impl Default for SQMemberHandle {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SQStackInfos {
    pub funcname: *const SQChar,
    pub source: *const SQChar,
    pub line: SQInteger,
}

impl Default for SQStackInfos {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}

pub type HSQOBJECT =       SQObject;
pub type HSQMEMBERHANDLE = SQMemberHandle;
pub type SQFUNCTION = extern "C" fn(v: HSQUIRRELVM) -> SQInteger;
pub type SQRELEASEHOOK = extern "C" fn(ptr: SQUserPointer, size: SQInteger) -> SQInteger;
pub type SQCOMPILERERROR = unsafe extern "C" fn(v:      HSQUIRRELVM,
                                                desc:   *const SQChar,
                                                source: *const SQChar,
                                                line:   SQInteger,
                                                column: SQInteger);
pub type SQPRINTFUNCTION = unsafe extern "C" fn(v:   HSQUIRRELVM,
                                                fmt: *const SQChar, 
                                                ...);
pub type SQDEBUGHOOK = unsafe extern "C" fn(v:          HSQUIRRELVM,
                                            _type:      SQInteger,
                                            sourcename: *const SQChar,
                                            line:       SQInteger,
                                            funcname:   *const SQChar);
pub type SQWRITEFUNC = extern "C" fn(arg1: SQUserPointer,
                                     arg2: SQUserPointer, 
                                     arg3: SQInteger) -> SQInteger;
pub type SQREADFUNC = extern "C" fn(arg1: SQUserPointer,
                                    arg2: SQUserPointer, 
                                    arg3: SQInteger) -> SQInteger;
pub type SQLEXREADFUNC = extern "C" fn(arg1: SQUserPointer) -> SQInteger;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SQRegFunction {
    pub name: *const SQChar,
    pub f: SQFUNCTION,
    pub nparamscheck: SQInteger,
    pub typemask: *const SQChar,
}

impl Default for SQRegFunction {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SQFunctionInfo {
    pub funcid: SQUserPointer,
    pub name: *const SQChar,
    pub source: *const SQChar,
}

impl Default for SQFunctionInfo {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}

extern "C" {
    /*vm*/
    pub fn sq_open(initialstacksize: SQInteger) -> HSQUIRRELVM;
    pub fn sq_newthread(friendvm: HSQUIRRELVM, initialstacksize: SQInteger) -> HSQUIRRELVM;
    pub fn sq_seterrorhandler(v: HSQUIRRELVM);
    pub fn sq_close(v: HSQUIRRELVM);
    pub fn sq_setforeignptr(v: HSQUIRRELVM, p: SQUserPointer);
    pub fn sq_getforeignptr(v: HSQUIRRELVM) -> SQUserPointer;
    pub fn sq_setprintfunc(v: HSQUIRRELVM, printfunc: SQPRINTFUNCTION, errfunc: SQPRINTFUNCTION);
    pub fn sq_getprintfunc(v: HSQUIRRELVM) -> SQPRINTFUNCTION;
    pub fn sq_geterrorfunc(v: HSQUIRRELVM) -> SQPRINTFUNCTION;
    pub fn sq_suspendvm(v: HSQUIRRELVM) -> SQRESULT;
    pub fn sq_wakeupvm(v: HSQUIRRELVM, resumedret: SQBool, retval: SQBool,
                       raiseerror: SQBool, throwerror: SQBool) -> SQRESULT;
    pub fn sq_getvmstate(v: HSQUIRRELVM) -> SQInteger;
    pub fn sq_getversion() -> SQInteger;

    /*compiler*/
    pub fn sq_compile(v: HSQUIRRELVM, read: SQLEXREADFUNC, p: SQUserPointer,
                      sourcename: *const SQChar, raiseerror: SQBool) -> SQRESULT;
    pub fn sq_compilebuffer(v: HSQUIRRELVM, s: *const SQChar, size: SQInteger,
                            sourcename: *const SQChar, raiseerror: SQBool) -> SQRESULT;
    pub fn sq_enabledebuginfo(v: HSQUIRRELVM, enable: SQBool);
    pub fn sq_notifyallexceptions(v: HSQUIRRELVM, enable: SQBool);
    pub fn sq_setcompilererrorhandler(v: HSQUIRRELVM, f: SQCOMPILERERROR);

    /*stack operations*/
    pub fn sq_push(v: HSQUIRRELVM, idx: SQInteger);
    pub fn sq_pop(v: HSQUIRRELVM, nelemstopop: SQInteger);
    pub fn sq_poptop(v: HSQUIRRELVM);
    pub fn sq_remove(v: HSQUIRRELVM, idx: SQInteger);
    pub fn sq_gettop(v: HSQUIRRELVM) -> SQInteger;
    pub fn sq_settop(v: HSQUIRRELVM, newtop: SQInteger);
    pub fn sq_reservestack(v: HSQUIRRELVM, nsize: SQInteger) -> SQRESULT;
    pub fn sq_cmp(v: HSQUIRRELVM) -> SQInteger;
    pub fn sq_move(dest: HSQUIRRELVM, src: HSQUIRRELVM, idx: SQInteger);

    /*object creation handling*/
    pub fn sq_newuserdata(v: HSQUIRRELVM, size: SQUnsignedInteger) -> SQUserPointer;
    pub fn sq_newtable(v: HSQUIRRELVM);
    pub fn sq_newtableex(v: HSQUIRRELVM, initialcapacity: SQInteger);
    pub fn sq_newarray(v: HSQUIRRELVM, size: SQInteger);
    pub fn sq_newclosure(v: HSQUIRRELVM, func: SQFUNCTION,
                         nfreevars: SQUnsignedInteger);
    pub fn sq_setparamscheck(v: HSQUIRRELVM, nparamscheck: SQInteger,
                             typemask: *const SQChar) -> SQRESULT;
    pub fn sq_bindenv(v: HSQUIRRELVM, idx: SQInteger) -> SQRESULT;
    pub fn sq_pushstring(v: HSQUIRRELVM, s: *const SQChar, len: SQInteger);
    pub fn sq_pushfloat(v: HSQUIRRELVM, f: SQFloat);
    pub fn sq_pushinteger(v: HSQUIRRELVM, n: SQInteger);
    pub fn sq_pushbool(v: HSQUIRRELVM, b: SQBool);
    pub fn sq_pushuserpointer(v: HSQUIRRELVM, p: SQUserPointer);
    pub fn sq_pushnull(v: HSQUIRRELVM);
    pub fn sq_gettype(v: HSQUIRRELVM, idx: SQInteger) -> SQObjectType;
    pub fn sq_typeof(v: HSQUIRRELVM, idx: SQInteger) -> SQRESULT;
    pub fn sq_getsize(v: HSQUIRRELVM, idx: SQInteger) -> SQInteger;
    pub fn sq_gethash(v: HSQUIRRELVM, idx: SQInteger) -> SQHash;
    pub fn sq_getbase(v: HSQUIRRELVM, idx: SQInteger) -> SQRESULT;
    pub fn sq_instanceof(v: HSQUIRRELVM) -> SQBool;
    pub fn sq_tostring(v: HSQUIRRELVM, idx: SQInteger) -> SQRESULT;
    pub fn sq_tobool(v: HSQUIRRELVM, idx: SQInteger, b: *mut SQBool);
    pub fn sq_getstring(v: HSQUIRRELVM, idx: SQInteger, c: *mut *const SQChar) -> SQRESULT;
    pub fn sq_getinteger(v: HSQUIRRELVM, idx: SQInteger, i: *mut SQInteger) -> SQRESULT;
    pub fn sq_getfloat(v: HSQUIRRELVM, idx: SQInteger, f: *mut SQFloat) -> SQRESULT;
    pub fn sq_getbool(v: HSQUIRRELVM, idx: SQInteger, b: *mut SQBool) -> SQRESULT;
    pub fn sq_getthread(v: HSQUIRRELVM, idx: SQInteger, thread: *mut HSQUIRRELVM) -> SQRESULT;
    pub fn sq_getuserpointer(v: HSQUIRRELVM, idx: SQInteger, p: *mut SQUserPointer) -> SQRESULT;
    pub fn sq_getuserdata(v: HSQUIRRELVM, idx: SQInteger,
                          p: *mut SQUserPointer, typetag: *mut SQUserPointer) -> SQRESULT;
    pub fn sq_settypetag(v: HSQUIRRELVM, idx: SQInteger, typetag: SQUserPointer) -> SQRESULT;
    pub fn sq_gettypetag(v: HSQUIRRELVM, idx: SQInteger, typetag: *mut SQUserPointer) -> SQRESULT;
    pub fn sq_setreleasehook(v: HSQUIRRELVM, idx: SQInteger, hook: SQRELEASEHOOK);
    pub fn sq_getscratchpad(v: HSQUIRRELVM, minsize: SQInteger) -> *mut SQChar;
    pub fn sq_getfunctioninfo(v: HSQUIRRELVM, level: SQInteger, fi: *mut SQFunctionInfo) -> SQRESULT;
    pub fn sq_getclosureinfo(v: HSQUIRRELVM, idx: SQInteger, nparams: *mut SQUnsignedInteger,
                             nfreevars: *mut SQUnsignedInteger) -> SQRESULT;
    pub fn sq_getclosurename(v: HSQUIRRELVM, idx: SQInteger) -> SQRESULT;
    pub fn sq_setnativeclosurename(v: HSQUIRRELVM, idx: SQInteger, name: *const SQChar) -> SQRESULT;
    pub fn sq_setinstanceup(v: HSQUIRRELVM, idx: SQInteger, p: SQUserPointer) -> SQRESULT;
    pub fn sq_getinstanceup(v: HSQUIRRELVM, idx: SQInteger, p: *mut SQUserPointer, 
                            typetag: SQUserPointer) -> SQRESULT;
    pub fn sq_setclassudsize(v: HSQUIRRELVM, idx: SQInteger, udsize: SQInteger) -> SQRESULT;
    pub fn sq_newclass(v: HSQUIRRELVM, hasbase: SQBool) -> SQRESULT;
    pub fn sq_createinstance(v: HSQUIRRELVM, idx: SQInteger) -> SQRESULT;
    pub fn sq_setattributes(v: HSQUIRRELVM, idx: SQInteger) -> SQRESULT;
    pub fn sq_getattributes(v: HSQUIRRELVM, idx: SQInteger) -> SQRESULT;
    pub fn sq_getclass(v: HSQUIRRELVM, idx: SQInteger) -> SQRESULT;
    pub fn sq_weakref(v: HSQUIRRELVM, idx: SQInteger);
    pub fn sq_getdefaultdelegate(v: HSQUIRRELVM, t: SQObjectType) -> SQRESULT;
    pub fn sq_getmemberhandle(v: HSQUIRRELVM, idx: SQInteger, handle: *mut HSQMEMBERHANDLE) -> SQRESULT;
    pub fn sq_getbyhandle(v: HSQUIRRELVM, idx: SQInteger, handle: *const HSQMEMBERHANDLE) -> SQRESULT;
    pub fn sq_setbyhandle(v: HSQUIRRELVM, idx: SQInteger, handle: *const HSQMEMBERHANDLE) -> SQRESULT;

    /*object manipulation*/
    pub fn sq_pushroottable(v: HSQUIRRELVM);
    pub fn sq_pushregistrytable(v: HSQUIRRELVM);
    pub fn sq_pushconsttable(v: HSQUIRRELVM);
    pub fn sq_setroottable(v: HSQUIRRELVM) -> SQRESULT;
    pub fn sq_setconsttable(v: HSQUIRRELVM) -> SQRESULT;
    pub fn sq_newslot(v: HSQUIRRELVM, idx: SQInteger, bstatic: SQBool) -> SQRESULT;
    pub fn sq_deleteslot(v: HSQUIRRELVM, idx: SQInteger, pushval: SQBool) -> SQRESULT;
    pub fn sq_set(v: HSQUIRRELVM, idx: SQInteger) -> SQRESULT;
    pub fn sq_get(v: HSQUIRRELVM, idx: SQInteger) -> SQRESULT;
    pub fn sq_rawget(v: HSQUIRRELVM, idx: SQInteger) -> SQRESULT;
    pub fn sq_rawset(v: HSQUIRRELVM, idx: SQInteger) -> SQRESULT;
    pub fn sq_rawdeleteslot(v: HSQUIRRELVM, idx: SQInteger, pushval: SQBool) -> SQRESULT;
    pub fn sq_newmember(v: HSQUIRRELVM, idx: SQInteger, bstatic: SQBool) -> SQRESULT;
    pub fn sq_rawnewmember(v: HSQUIRRELVM, idx: SQInteger, bstatic: SQBool) -> SQRESULT;
    pub fn sq_arrayappend(v: HSQUIRRELVM, idx: SQInteger) -> SQRESULT;
    pub fn sq_arraypop(v: HSQUIRRELVM, idx: SQInteger, pushval: SQBool) -> SQRESULT;
    pub fn sq_arrayresize(v: HSQUIRRELVM, idx: SQInteger, newsize: SQInteger) -> SQRESULT;
    pub fn sq_arrayreverse(v: HSQUIRRELVM, idx: SQInteger) -> SQRESULT;
    pub fn sq_arrayremove(v: HSQUIRRELVM, idx: SQInteger, itemidx: SQInteger) -> SQRESULT;
    pub fn sq_arrayinsert(v: HSQUIRRELVM, idx: SQInteger, destpos: SQInteger) -> SQRESULT;
    pub fn sq_setdelegate(v: HSQUIRRELVM, idx: SQInteger) -> SQRESULT;
    pub fn sq_getdelegate(v: HSQUIRRELVM, idx: SQInteger) -> SQRESULT;
    pub fn sq_clone(v: HSQUIRRELVM, idx: SQInteger) -> SQRESULT;
    pub fn sq_setfreevariable(v: HSQUIRRELVM, idx: SQInteger, nval: SQUnsignedInteger) -> SQRESULT;
    pub fn sq_next(v: HSQUIRRELVM, idx: SQInteger) -> SQRESULT;
    pub fn sq_getweakrefval(v: HSQUIRRELVM, idx: SQInteger) -> SQRESULT;
    pub fn sq_clear(v: HSQUIRRELVM, idx: SQInteger) -> SQRESULT;

    /*calls*/
    pub fn sq_call(v: HSQUIRRELVM, params: SQInteger, retval: SQBool, raiseerror: SQBool) -> SQRESULT;
    pub fn sq_resume(v: HSQUIRRELVM, retval: SQBool, raiseerror: SQBool) -> SQRESULT;
    pub fn sq_getlocal(v: HSQUIRRELVM, level: SQUnsignedInteger, idx: SQUnsignedInteger) -> *const SQChar;
    pub fn sq_getcallee(v: HSQUIRRELVM) -> SQRESULT;
    pub fn sq_getfreevariable(v: HSQUIRRELVM, idx: SQInteger, nval: SQUnsignedInteger) -> *const SQChar;
    pub fn sq_throwerror(v: HSQUIRRELVM, err: *const SQChar) -> SQRESULT;
    pub fn sq_throwobject(v: HSQUIRRELVM) -> SQRESULT;
    pub fn sq_reseterror(v: HSQUIRRELVM);
    pub fn sq_getlasterror(v: HSQUIRRELVM);

    /*raw object handling*/
    pub fn sq_getstackobj(v: HSQUIRRELVM, idx: SQInteger, po: *mut HSQOBJECT) -> SQRESULT;
    pub fn sq_pushobject(v: HSQUIRRELVM, obj: HSQOBJECT);
    pub fn sq_addref(v: HSQUIRRELVM, po: *mut HSQOBJECT);
    pub fn sq_release(v: HSQUIRRELVM, po: *mut HSQOBJECT) -> SQBool;
    pub fn sq_getrefcount(v: HSQUIRRELVM, po: *mut HSQOBJECT) -> SQUnsignedInteger;
    pub fn sq_resetobject(po: *mut HSQOBJECT);
    pub fn sq_objtostring(o: *const HSQOBJECT) -> *const SQChar;
    pub fn sq_objtobool(o: *const HSQOBJECT) -> SQBool;
    pub fn sq_objtointeger(o: *const HSQOBJECT) -> SQInteger;
    pub fn sq_objtofloat(o: *const HSQOBJECT) -> SQFloat;
    pub fn sq_objtouserpointer(o: *const HSQOBJECT) -> SQUserPointer;
    pub fn sq_getobjtypetag(o: *const HSQOBJECT, typetag: *mut SQUserPointer) -> SQRESULT;

    /*GC*/
    pub fn sq_collectgarbage(v: HSQUIRRELVM) -> SQInteger;
    pub fn sq_resurrectunreachable(v: HSQUIRRELVM) -> SQRESULT;

    /*serialization*/
    pub fn sq_writeclosure(vm: HSQUIRRELVM, writef: SQWRITEFUNC, up: SQUserPointer) -> SQRESULT;
    pub fn sq_readclosure(vm: HSQUIRRELVM, readf: SQREADFUNC, up: SQUserPointer) -> SQRESULT;

    /*mem allocation*/
    pub fn sq_malloc(size: SQUnsignedInteger) -> *mut ::libc::c_void;
    pub fn sq_realloc(p: *mut ::libc::c_void, oldsize: SQUnsignedInteger, 
                      newsize: SQUnsignedInteger) -> *mut ::libc::c_void;
    pub fn sq_free(p: *mut ::libc::c_void, size: SQUnsignedInteger);

    /*debug*/
    pub fn sq_stackinfos(v: HSQUIRRELVM, level: SQInteger, si: *mut SQStackInfos) -> SQRESULT;
    pub fn sq_setdebughook(v: HSQUIRRELVM);
    pub fn sq_setnativedebughook(v: HSQUIRRELVM, hook: SQDEBUGHOOK);
}
