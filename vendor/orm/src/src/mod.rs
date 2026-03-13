#![allow(warnings)]
#![allow(dependency_on_unit_never_type_fallback)]
use temper_core::AnyValueTrait;
use temper_core::AsAnyValue;
use temper_core::Pair;
pub (crate) fn init() -> temper_core::Result<()> {
    static INIT_ONCE: std::sync::OnceLock<temper_core::Result<()>> = std::sync::OnceLock::new();
    INIT_ONCE.get_or_init(| |{
            Ok(())
    }).clone()
}
struct ChangesetErrorStruct {
    field: std::sync::Arc<String>, message: std::sync::Arc<String>
}
#[derive(Clone)]
pub struct ChangesetError(std::sync::Arc<ChangesetErrorStruct>);
#[derive(Clone)]
pub struct ChangesetErrorBuilder {
    pub field: std::sync::Arc<String>, pub message: std::sync::Arc<String>
}
impl ChangesetErrorBuilder {
    pub fn build(self) -> ChangesetError {
        ChangesetError::new(self.field, self.message)
    }
}
impl ChangesetError {
    pub fn new(field__312: impl temper_core::ToArcString, message__313: impl temper_core::ToArcString) -> ChangesetError {
        let field__312 = field__312.to_arc_string();
        let message__313 = message__313.to_arc_string();
        let field;
        let message;
        field = field__312.clone();
        message = message__313.clone();
        let selfish = ChangesetError(std::sync::Arc::new(ChangesetErrorStruct {
                    field, message
        }));
        return selfish;
    }
    pub fn field(& self) -> std::sync::Arc<String> {
        return self.0.field.clone();
    }
    pub fn message(& self) -> std::sync::Arc<String> {
        return self.0.message.clone();
    }
}
temper_core::impl_any_value_trait!(ChangesetError, []);
pub enum ChangesetEnum {
    ChangesetImpl(ChangesetImpl)
}
pub trait ChangesetTrait: temper_core::AsAnyValue + temper_core::AnyValueTrait + std::marker::Send + std::marker::Sync {
    fn as_enum(& self) -> ChangesetEnum;
    fn clone_boxed(& self) -> Changeset;
    fn table_def(& self) -> TableDef;
    fn changes(& self) -> temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>;
    fn errors(& self) -> temper_core::List<ChangesetError>;
    fn is_valid(& self) -> bool;
    fn cast(& self, allowedFields__323: temper_core::List<SafeIdentifier>) -> Changeset;
    fn validate_required(& self, fields__326: temper_core::List<SafeIdentifier>) -> Changeset;
    fn validate_length(& self, field__329: SafeIdentifier, min__330: i32, max__331: i32) -> Changeset;
    fn validate_int(& self, field__334: SafeIdentifier) -> Changeset;
    fn validate_int64(& self, field__337: SafeIdentifier) -> Changeset;
    fn validate_float(& self, field__340: SafeIdentifier) -> Changeset;
    fn validate_bool(& self, field__343: SafeIdentifier) -> Changeset;
    fn to_insert_sql(& self) -> temper_core::Result<SqlFragment>;
    fn to_update_sql(& self, id__348: i32) -> temper_core::Result<SqlFragment>;
}
#[derive(Clone)]
pub struct Changeset(std::sync::Arc<dyn ChangesetTrait>);
impl Changeset {
    pub fn new(selfish: impl ChangesetTrait + 'static) -> Changeset {
        Changeset(std::sync::Arc::new(selfish))
    }
}
impl ChangesetTrait for Changeset {
    fn as_enum(& self) -> ChangesetEnum {
        ChangesetTrait::as_enum( & ( * self.0))
    }
    fn clone_boxed(& self) -> Changeset {
        ChangesetTrait::clone_boxed( & ( * self.0))
    }
    fn table_def(& self) -> TableDef {
        ChangesetTrait::table_def( & ( * self.0))
    }
    fn changes(& self) -> temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> {
        ChangesetTrait::changes( & ( * self.0))
    }
    fn errors(& self) -> temper_core::List<ChangesetError> {
        ChangesetTrait::errors( & ( * self.0))
    }
    fn is_valid(& self) -> bool {
        ChangesetTrait::is_valid( & ( * self.0))
    }
    fn cast(& self, arg1: temper_core::List<SafeIdentifier>) -> Changeset {
        ChangesetTrait::cast( & ( * self.0), arg1)
    }
    fn validate_required(& self, arg1: temper_core::List<SafeIdentifier>) -> Changeset {
        ChangesetTrait::validate_required( & ( * self.0), arg1)
    }
    fn validate_length(& self, arg1: SafeIdentifier, arg2: i32, arg3: i32) -> Changeset {
        ChangesetTrait::validate_length( & ( * self.0), arg1, arg2, arg3)
    }
    fn validate_int(& self, arg1: SafeIdentifier) -> Changeset {
        ChangesetTrait::validate_int( & ( * self.0), arg1)
    }
    fn validate_int64(& self, arg1: SafeIdentifier) -> Changeset {
        ChangesetTrait::validate_int64( & ( * self.0), arg1)
    }
    fn validate_float(& self, arg1: SafeIdentifier) -> Changeset {
        ChangesetTrait::validate_float( & ( * self.0), arg1)
    }
    fn validate_bool(& self, arg1: SafeIdentifier) -> Changeset {
        ChangesetTrait::validate_bool( & ( * self.0), arg1)
    }
    fn to_insert_sql(& self) -> temper_core::Result<SqlFragment> {
        ChangesetTrait::to_insert_sql( & ( * self.0))
    }
    fn to_update_sql(& self, arg1: i32) -> temper_core::Result<SqlFragment> {
        ChangesetTrait::to_update_sql( & ( * self.0), arg1)
    }
}
temper_core::impl_any_value_trait_for_interface!(Changeset);
impl std::ops::Deref for Changeset {
    type Target = dyn ChangesetTrait;
    fn deref(& self) -> & Self::Target {
        & ( * self.0)
    }
}
struct ChangesetImplStruct {
    table_def: TableDef, params: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>, changes: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>, errors: temper_core::List<ChangesetError>, is_valid: bool
}
#[derive(Clone)]
pub (crate) struct ChangesetImpl(std::sync::Arc<ChangesetImplStruct>);
impl ChangesetImpl {
    pub fn table_def(& self) -> TableDef {
        return self.0.table_def.clone();
    }
    pub fn changes(& self) -> temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> {
        return self.0.changes.clone();
    }
    pub fn errors(& self) -> temper_core::List<ChangesetError> {
        return self.0.errors.clone();
    }
    pub fn is_valid(& self) -> bool {
        return self.0.is_valid;
    }
    pub fn cast(& self, allowedFields__364: impl temper_core::ToList<SafeIdentifier>) -> Changeset {
        let allowedFields__364 = allowedFields__364.to_list();
        let mb__366: temper_core::MapBuilder<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::MapBuilder::new();
        #[derive(Clone)]
        struct ClosureGroup___1 {
            this__105: ChangesetImpl, mb__366: temper_core::MapBuilder<std::sync::Arc<String>, std::sync::Arc<String>>
        }
        impl ClosureGroup___1 {
            fn fn__5014(& self, f__367: SafeIdentifier) {
                let mut t___5012: std::sync::Arc<String>;
                let mut t___5009: std::sync::Arc<String> = f__367.sql_value();
                let val__368: std::sync::Arc<String> = temper_core::MappedTrait::get_or( & self.this__105.0.params, t___5009.clone(), std::sync::Arc::new("".to_string()));
                if ! val__368.is_empty() {
                    t___5012 = f__367.sql_value();
                    temper_core::MapBuilder::set( & self.mb__366, t___5012.clone(), val__368.clone());
                }
            }
        }
        let closure_group = ClosureGroup___1 {
            this__105: self.clone(), mb__366: mb__366.clone()
        };
        let fn__5014 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | f__367: SafeIdentifier | closure_group.fn__5014(f__367))
        };
        temper_core::listed::list_for_each( & allowedFields__364, & ( * fn__5014.clone()));
        return Changeset::new(ChangesetImpl::new(self.0.table_def.clone(), self.0.params.clone(), temper_core::MappedTrait::to_map( & mb__366), self.0.errors.clone(), self.0.is_valid));
    }
    pub fn validate_required(& self, fields__370: impl temper_core::ToList<SafeIdentifier>) -> Changeset {
        let fields__370 = fields__370.to_list();
        let return__192: Changeset;
        let mut t___5007: temper_core::List<ChangesetError>;
        let mut t___2977: TableDef;
        let mut t___2978: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>;
        let mut t___2979: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>;
        'fn__371: {
            if ! self.0.is_valid {
                return__192 = Changeset::new(self.clone());
                break 'fn__371;
            }
            let eb__372: temper_core::ListBuilder<ChangesetError> = temper_core::ListedTrait::to_list_builder( & self.0.errors);
            let mut valid__373: std::sync::Arc<std::sync::RwLock<bool>> = std::sync::Arc::new(std::sync::RwLock::new(true));
            #[derive(Clone)]
            struct ClosureGroup___2 {
                this__106: ChangesetImpl, eb__372: temper_core::ListBuilder<ChangesetError>, valid__373: std::sync::Arc<std::sync::RwLock<bool>>
            }
            impl ClosureGroup___2 {
                fn fn__5003(& self, f__374: SafeIdentifier) {
                    let mut t___5001: ChangesetError;
                    let mut t___4998: std::sync::Arc<String> = f__374.sql_value();
                    if ! temper_core::MappedTrait::has( & self.this__106.0.changes, t___4998.clone()) {
                        t___5001 = ChangesetError::new(f__374.sql_value(), "is required");
                        temper_core::listed::add( & self.eb__372, t___5001.clone(), None);
                        {
                            * self.valid__373.write().unwrap() = false;
                        }
                    }
                }
            }
            let closure_group = ClosureGroup___2 {
                this__106: self.clone(), eb__372: eb__372.clone(), valid__373: valid__373.clone()
            };
            let fn__5003 = {
                let closure_group = closure_group.clone();
                std::sync::Arc::new(move | f__374: SafeIdentifier | closure_group.fn__5003(f__374))
            };
            temper_core::listed::list_for_each( & fields__370, & ( * fn__5003.clone()));
            t___2977 = self.0.table_def.clone();
            t___2978 = self.0.params.clone();
            t___2979 = self.0.changes.clone();
            t___5007 = temper_core::ListedTrait::to_list( & eb__372);
            return__192 = Changeset::new(ChangesetImpl::new(t___2977.clone(), t___2978.clone(), t___2979.clone(), t___5007.clone(), temper_core::read_locked( & valid__373)));
        }
        return return__192.clone();
    }
    pub fn validate_length(& self, field__376: SafeIdentifier, min__377: i32, max__378: i32) -> Changeset {
        let return__193: Changeset;
        let mut t___4985: std::sync::Arc<String>;
        let mut t___4996: temper_core::List<ChangesetError>;
        let mut t___2960: bool;
        let mut t___2966: TableDef;
        let mut t___2967: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>;
        let mut t___2968: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>;
        'fn__379: {
            if ! self.0.is_valid {
                return__193 = Changeset::new(self.clone());
                break 'fn__379;
            }
            t___4985 = field__376.sql_value();
            let val__380: std::sync::Arc<String> = temper_core::MappedTrait::get_or( & self.0.changes, t___4985.clone(), std::sync::Arc::new("".to_string()));
            let len__381: i32 = temper_core::string::count_between( & val__380, 0usize, val__380.len());
            if Some(len__381) < Some(min__377) {
                t___2960 = true;
            } else {
                t___2960 = Some(len__381) > Some(max__378);
            }
            if t___2960 {
                let msg__382: std::sync::Arc<String> = std::sync::Arc::new(format!("must be between {} and {} characters", min__377, max__378));
                let eb__383: temper_core::ListBuilder<ChangesetError> = temper_core::ListedTrait::to_list_builder( & self.0.errors);
                temper_core::listed::add( & eb__383, ChangesetError::new(field__376.sql_value(), msg__382.clone()), None);
                t___2966 = self.0.table_def.clone();
                t___2967 = self.0.params.clone();
                t___2968 = self.0.changes.clone();
                t___4996 = temper_core::ListedTrait::to_list( & eb__383);
                return__193 = Changeset::new(ChangesetImpl::new(t___2966.clone(), t___2967.clone(), t___2968.clone(), t___4996.clone(), false));
                break 'fn__379;
            }
            return__193 = Changeset::new(self.clone());
        }
        return return__193.clone();
    }
    pub fn validate_int(& self, field__385: SafeIdentifier) -> Changeset {
        let return__194: Changeset;
        let mut t___4976: std::sync::Arc<String>;
        let mut t___4983: temper_core::List<ChangesetError>;
        let mut t___2951: TableDef;
        let mut t___2952: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>;
        let mut t___2953: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>;
        'fn__386: {
            if ! self.0.is_valid {
                return__194 = Changeset::new(self.clone());
                break 'fn__386;
            }
            t___4976 = field__385.sql_value();
            let val__387: std::sync::Arc<String> = temper_core::MappedTrait::get_or( & self.0.changes, t___4976.clone(), std::sync::Arc::new("".to_string()));
            if val__387.is_empty() {
                return__194 = Changeset::new(self.clone());
                break 'fn__386;
            }
            let parseOk__388: bool;
            'ok___5119: {
                'orelse___1072: {
                    match temper_core::string::to_int( & val__387, None) {
                        Ok(x) => x,
                        _ => break 'orelse___1072
                    };
                    parseOk__388 = true;
                    break 'ok___5119;
                }
                parseOk__388 = false;
            }
            if ! parseOk__388 {
                let eb__389: temper_core::ListBuilder<ChangesetError> = temper_core::ListedTrait::to_list_builder( & self.0.errors);
                temper_core::listed::add( & eb__389, ChangesetError::new(field__385.sql_value(), "must be an integer"), None);
                t___2951 = self.0.table_def.clone();
                t___2952 = self.0.params.clone();
                t___2953 = self.0.changes.clone();
                t___4983 = temper_core::ListedTrait::to_list( & eb__389);
                return__194 = Changeset::new(ChangesetImpl::new(t___2951.clone(), t___2952.clone(), t___2953.clone(), t___4983.clone(), false));
                break 'fn__386;
            }
            return__194 = Changeset::new(self.clone());
        }
        return return__194.clone();
    }
    pub fn validate_int64(& self, field__391: SafeIdentifier) -> Changeset {
        let return__195: Changeset;
        let mut t___4967: std::sync::Arc<String>;
        let mut t___4974: temper_core::List<ChangesetError>;
        let mut t___2938: TableDef;
        let mut t___2939: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>;
        let mut t___2940: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>;
        'fn__392: {
            if ! self.0.is_valid {
                return__195 = Changeset::new(self.clone());
                break 'fn__392;
            }
            t___4967 = field__391.sql_value();
            let val__393: std::sync::Arc<String> = temper_core::MappedTrait::get_or( & self.0.changes, t___4967.clone(), std::sync::Arc::new("".to_string()));
            if val__393.is_empty() {
                return__195 = Changeset::new(self.clone());
                break 'fn__392;
            }
            let parseOk__394: bool;
            'ok___5121: {
                'orelse___1073: {
                    match temper_core::string::to_int64( & val__393, None) {
                        Ok(x) => x,
                        _ => break 'orelse___1073
                    };
                    parseOk__394 = true;
                    break 'ok___5121;
                }
                parseOk__394 = false;
            }
            if ! parseOk__394 {
                let eb__395: temper_core::ListBuilder<ChangesetError> = temper_core::ListedTrait::to_list_builder( & self.0.errors);
                temper_core::listed::add( & eb__395, ChangesetError::new(field__391.sql_value(), "must be a 64-bit integer"), None);
                t___2938 = self.0.table_def.clone();
                t___2939 = self.0.params.clone();
                t___2940 = self.0.changes.clone();
                t___4974 = temper_core::ListedTrait::to_list( & eb__395);
                return__195 = Changeset::new(ChangesetImpl::new(t___2938.clone(), t___2939.clone(), t___2940.clone(), t___4974.clone(), false));
                break 'fn__392;
            }
            return__195 = Changeset::new(self.clone());
        }
        return return__195.clone();
    }
    pub fn validate_float(& self, field__397: SafeIdentifier) -> Changeset {
        let return__196: Changeset;
        let mut t___4958: std::sync::Arc<String>;
        let mut t___4965: temper_core::List<ChangesetError>;
        let mut t___2925: TableDef;
        let mut t___2926: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>;
        let mut t___2927: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>;
        'fn__398: {
            if ! self.0.is_valid {
                return__196 = Changeset::new(self.clone());
                break 'fn__398;
            }
            t___4958 = field__397.sql_value();
            let val__399: std::sync::Arc<String> = temper_core::MappedTrait::get_or( & self.0.changes, t___4958.clone(), std::sync::Arc::new("".to_string()));
            if val__399.is_empty() {
                return__196 = Changeset::new(self.clone());
                break 'fn__398;
            }
            let parseOk__400: bool;
            'ok___5123: {
                'orelse___1074: {
                    match temper_core::string::to_float64( & val__399) {
                        Ok(x) => x,
                        _ => break 'orelse___1074
                    };
                    parseOk__400 = true;
                    break 'ok___5123;
                }
                parseOk__400 = false;
            }
            if ! parseOk__400 {
                let eb__401: temper_core::ListBuilder<ChangesetError> = temper_core::ListedTrait::to_list_builder( & self.0.errors);
                temper_core::listed::add( & eb__401, ChangesetError::new(field__397.sql_value(), "must be a number"), None);
                t___2925 = self.0.table_def.clone();
                t___2926 = self.0.params.clone();
                t___2927 = self.0.changes.clone();
                t___4965 = temper_core::ListedTrait::to_list( & eb__401);
                return__196 = Changeset::new(ChangesetImpl::new(t___2925.clone(), t___2926.clone(), t___2927.clone(), t___4965.clone(), false));
                break 'fn__398;
            }
            return__196 = Changeset::new(self.clone());
        }
        return return__196.clone();
    }
    pub fn validate_bool(& self, field__403: SafeIdentifier) -> Changeset {
        let return__197: Changeset;
        let mut t___4949: std::sync::Arc<String>;
        let mut t___4956: temper_core::List<ChangesetError>;
        let mut t___2900: bool;
        let mut t___2901: bool;
        let mut t___2903: bool;
        let mut t___2904: bool;
        let mut t___2906: bool;
        let mut t___2912: TableDef;
        let mut t___2913: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>;
        let mut t___2914: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>;
        'fn__404: {
            if ! self.0.is_valid {
                return__197 = Changeset::new(self.clone());
                break 'fn__404;
            }
            t___4949 = field__403.sql_value();
            let val__405: std::sync::Arc<String> = temper_core::MappedTrait::get_or( & self.0.changes, t___4949.clone(), std::sync::Arc::new("".to_string()));
            if val__405.is_empty() {
                return__197 = Changeset::new(self.clone());
                break 'fn__404;
            }
            let isTrue__406: bool;
            if Some(val__405.as_str()) == Some("true") {
                isTrue__406 = true;
            } else {
                if Some(val__405.as_str()) == Some("1") {
                    t___2901 = true;
                } else {
                    if Some(val__405.as_str()) == Some("yes") {
                        t___2900 = true;
                    } else {
                        t___2900 = Some(val__405.as_str()) == Some("on");
                    }
                    t___2901 = t___2900;
                }
                isTrue__406 = t___2901;
            }
            let isFalse__407: bool;
            if Some(val__405.as_str()) == Some("false") {
                isFalse__407 = true;
            } else {
                if Some(val__405.as_str()) == Some("0") {
                    t___2904 = true;
                } else {
                    if Some(val__405.as_str()) == Some("no") {
                        t___2903 = true;
                    } else {
                        t___2903 = Some(val__405.as_str()) == Some("off");
                    }
                    t___2904 = t___2903;
                }
                isFalse__407 = t___2904;
            }
            if ! isTrue__406 {
                t___2906 = ! isFalse__407;
            } else {
                t___2906 = false;
            }
            if t___2906 {
                let eb__408: temper_core::ListBuilder<ChangesetError> = temper_core::ListedTrait::to_list_builder( & self.0.errors);
                temper_core::listed::add( & eb__408, ChangesetError::new(field__403.sql_value(), "must be a boolean (true/false/1/0/yes/no/on/off)"), None);
                t___2912 = self.0.table_def.clone();
                t___2913 = self.0.params.clone();
                t___2914 = self.0.changes.clone();
                t___4956 = temper_core::ListedTrait::to_list( & eb__408);
                return__197 = Changeset::new(ChangesetImpl::new(t___2912.clone(), t___2913.clone(), t___2914.clone(), t___4956.clone(), false));
                break 'fn__404;
            }
            return__197 = Changeset::new(self.clone());
        }
        return return__197.clone();
    }
    fn parse_bool_sql_part(& self, val__410: impl temper_core::ToArcString) -> temper_core::Result<SqlBoolean> {
        let val__410 = val__410.to_arc_string();
        let return__198: SqlBoolean;
        let mut t___2889: bool;
        let mut t___2890: bool;
        let mut t___2891: bool;
        let mut t___2893: bool;
        let mut t___2894: bool;
        let mut t___2895: bool;
        'fn__411: {
            if Some(val__410.as_str()) == Some("true") {
                t___2891 = true;
            } else {
                if Some(val__410.as_str()) == Some("1") {
                    t___2890 = true;
                } else {
                    if Some(val__410.as_str()) == Some("yes") {
                        t___2889 = true;
                    } else {
                        t___2889 = Some(val__410.as_str()) == Some("on");
                    }
                    t___2890 = t___2889;
                }
                t___2891 = t___2890;
            }
            if t___2891 {
                return__198 = SqlBoolean::new(true);
                break 'fn__411;
            }
            if Some(val__410.as_str()) == Some("false") {
                t___2895 = true;
            } else {
                if Some(val__410.as_str()) == Some("0") {
                    t___2894 = true;
                } else {
                    if Some(val__410.as_str()) == Some("no") {
                        t___2893 = true;
                    } else {
                        t___2893 = Some(val__410.as_str()) == Some("off");
                    }
                    t___2894 = t___2893;
                }
                t___2895 = t___2894;
            }
            if t___2895 {
                return__198 = SqlBoolean::new(false);
                break 'fn__411;
            }
            return Err(temper_core::Error::new());
        }
        return Ok(return__198.clone());
    }
    fn value_to_sql_part(& self, fieldDef__413: FieldDef, val__414: impl temper_core::ToArcString) -> temper_core::Result<SqlPart> {
        let val__414 = val__414.to_arc_string();
        let return__199: SqlPart;
        let mut t___2876: i32;
        let mut t___2879: i64;
        let mut t___2882: f64;
        let mut t___2887: temper_std::temporal::Date;
        'fn__415: {
            let ft__416: FieldType = fieldDef__413.field_type();
            if temper_core::is::<StringField>(ft__416.clone()) {
                return__199 = SqlPart::new(SqlString::new(val__414.clone()));
                break 'fn__415;
            }
            if temper_core::is::<IntField>(ft__416.clone()) {
                t___2876 = temper_core::string::to_int( & val__414, None) ? ;
                return__199 = SqlPart::new(SqlInt32::new(t___2876));
                break 'fn__415;
            }
            if temper_core::is::<Int64Field>(ft__416.clone()) {
                t___2879 = temper_core::string::to_int64( & val__414, None) ? ;
                return__199 = SqlPart::new(SqlInt64::new(t___2879));
                break 'fn__415;
            }
            if temper_core::is::<FloatField>(ft__416.clone()) {
                t___2882 = temper_core::string::to_float64( & val__414) ? ;
                return__199 = SqlPart::new(SqlFloat64::new(t___2882));
                break 'fn__415;
            }
            if temper_core::is::<BoolField>(ft__416.clone()) {
                return__199 = SqlPart::new(self.parse_bool_sql_part(val__414.clone()) ? );
                break 'fn__415;
            }
            if temper_core::is::<DateField>(ft__416.clone()) {
                t___2887 = temper_std::temporal::Date::from_iso_string(val__414.clone()) ? ;
                return__199 = SqlPart::new(SqlDate::new(t___2887.clone()));
                break 'fn__415;
            }
            return Err(temper_core::Error::new());
        }
        return Ok(return__199.clone());
    }
    pub fn to_insert_sql(& self) -> temper_core::Result<SqlFragment> {
        let mut t___4897: i32;
        let mut t___4902: std::sync::Arc<String>;
        let mut t___4903: bool;
        let mut t___4908: i32;
        let mut t___4910: std::sync::Arc<String>;
        let mut t___4914: std::sync::Arc<String>;
        let mut t___4929: i32;
        let mut t___2840: bool;
        let mut t___2848: FieldDef;
        let mut t___2853: SqlPart;
        if ! self.0.is_valid {
            return Err(temper_core::Error::new());
        }
        let mut i__419: i32 = 0;
        'loop___5183: loop {
            t___4897 = temper_core::ListedTrait::len( & self.0.table_def.fields());
            if ! (Some(i__419) < Some(t___4897)) {
                break;
            }
            let f__420: FieldDef = temper_core::ListedTrait::get( & self.0.table_def.fields(), i__419);
            if ! f__420.nullable() {
                t___4902 = f__420.name().sql_value();
                t___4903 = temper_core::MappedTrait::has( & self.0.changes, t___4902.clone());
                t___2840 = ! t___4903;
            } else {
                t___2840 = false;
            }
            if t___2840 {
                return Err(temper_core::Error::new());
            }
            i__419 = i__419.wrapping_add(1);
        }
        let pairs__421: temper_core::List<(std::sync::Arc<String>, std::sync::Arc<String>)> = temper_core::MappedTrait::to_list( & self.0.changes);
        if Some(temper_core::ListedTrait::len( & pairs__421)) == Some(0) {
            return Err(temper_core::Error::new());
        }
        let colNames__422: temper_core::ListBuilder<std::sync::Arc<String>> = temper_core::listed::new_builder();
        let valParts__423: temper_core::ListBuilder<SqlPart> = temper_core::listed::new_builder();
        let mut i__424: i32 = 0;
        'loop___5184: loop {
            t___4908 = temper_core::ListedTrait::len( & pairs__421);
            if ! (Some(i__424) < Some(t___4908)) {
                break;
            }
            let pair__425: (std::sync::Arc<String>, std::sync::Arc<String>) = temper_core::ListedTrait::get( & pairs__421, i__424);
            t___4910 = pair__425.key();
            t___2848 = self.0.table_def.field(t___4910.clone()) ? ;
            let fd__426: FieldDef = t___2848.clone();
            temper_core::listed::add( & colNames__422, fd__426.name().sql_value(), None);
            t___4914 = pair__425.value();
            t___2853 = self.value_to_sql_part(fd__426.clone(), t___4914.clone()) ? ;
            temper_core::listed::add( & valParts__423, t___2853.clone(), None);
            i__424 = i__424.wrapping_add(1);
        }
        let b__427: SqlBuilder = SqlBuilder::new();
        b__427.append_safe("INSERT INTO ");
        b__427.append_safe(self.0.table_def.table_name().sql_value());
        b__427.append_safe(" (");
        let mut t___4922: temper_core::List<std::sync::Arc<String>> = temper_core::ListedTrait::to_list( & colNames__422);
        #[derive(Clone)]
        struct ClosureGroup___3 {}
        impl ClosureGroup___3 {
            fn fn__4895(& self, c__428: impl temper_core::ToArcString) -> std::sync::Arc<String> {
                let c__428 = c__428.to_arc_string();
                return c__428.clone();
            }
        }
        let closure_group = ClosureGroup___3 {};
        let fn__4895 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | c__428: std::sync::Arc<String> | closure_group.fn__4895(c__428))
        };
        b__427.append_safe(temper_core::listed::join( & t___4922, std::sync::Arc::new(", ".to_string()), & ( * fn__4895.clone())));
        b__427.append_safe(") VALUES (");
        b__427.append_part(temper_core::ListedTrait::get( & valParts__423, 0));
        let mut j__429: i32 = 1;
        'loop___5185: loop {
            t___4929 = temper_core::ListedTrait::len( & valParts__423);
            if ! (Some(j__429) < Some(t___4929)) {
                break;
            }
            b__427.append_safe(", ");
            b__427.append_part(temper_core::ListedTrait::get( & valParts__423, j__429));
            j__429 = j__429.wrapping_add(1);
        }
        b__427.append_safe(")");
        return Ok(b__427.accumulated());
    }
    pub fn to_update_sql(& self, id__431: i32) -> temper_core::Result<SqlFragment> {
        let mut t___4882: i32;
        let mut t___4885: std::sync::Arc<String>;
        let mut t___4890: std::sync::Arc<String>;
        let mut t___2821: FieldDef;
        let mut t___2827: SqlPart;
        if ! self.0.is_valid {
            return Err(temper_core::Error::new());
        }
        let pairs__433: temper_core::List<(std::sync::Arc<String>, std::sync::Arc<String>)> = temper_core::MappedTrait::to_list( & self.0.changes);
        if Some(temper_core::ListedTrait::len( & pairs__433)) == Some(0) {
            return Err(temper_core::Error::new());
        }
        let b__434: SqlBuilder = SqlBuilder::new();
        b__434.append_safe("UPDATE ");
        b__434.append_safe(self.0.table_def.table_name().sql_value());
        b__434.append_safe(" SET ");
        let mut i__435: i32 = 0;
        'loop___5186: loop {
            t___4882 = temper_core::ListedTrait::len( & pairs__433);
            if ! (Some(i__435) < Some(t___4882)) {
                break;
            }
            if Some(i__435) > Some(0) {
                b__434.append_safe(", ");
            }
            let pair__436: (std::sync::Arc<String>, std::sync::Arc<String>) = temper_core::ListedTrait::get( & pairs__433, i__435);
            t___4885 = pair__436.key();
            t___2821 = self.0.table_def.field(t___4885.clone()) ? ;
            let fd__437: FieldDef = t___2821.clone();
            b__434.append_safe(fd__437.name().sql_value());
            b__434.append_safe(" = ");
            t___4890 = pair__436.value();
            t___2827 = self.value_to_sql_part(fd__437.clone(), t___4890.clone()) ? ;
            b__434.append_part(t___2827.clone());
            i__435 = i__435.wrapping_add(1);
        }
        b__434.append_safe(" WHERE id = ");
        b__434.append_int32(id__431);
        return Ok(b__434.accumulated());
    }
    pub fn new(_tableDef__439: TableDef, _params__440: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>, _changes__441: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>, _errors__442: impl temper_core::ToList<ChangesetError>, _isValid__443: bool) -> ChangesetImpl {
        let _errors__442 = _errors__442.to_list();
        let table_def;
        let params;
        let changes;
        let errors;
        let is_valid;
        table_def = _tableDef__439.clone();
        params = _params__440.clone();
        changes = _changes__441.clone();
        errors = _errors__442.clone();
        is_valid = _isValid__443;
        let selfish = ChangesetImpl(std::sync::Arc::new(ChangesetImplStruct {
                    table_def, params, changes, errors, is_valid
        }));
        return selfish;
    }
}
impl ChangesetTrait for ChangesetImpl {
    fn as_enum(& self) -> ChangesetEnum {
        ChangesetEnum::ChangesetImpl(self.clone())
    }
    fn clone_boxed(& self) -> Changeset {
        Changeset::new(self.clone())
    }
    fn table_def(& self) -> TableDef {
        self.table_def()
    }
    fn changes(& self) -> temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> {
        self.changes()
    }
    fn errors(& self) -> temper_core::List<ChangesetError> {
        self.errors()
    }
    fn is_valid(& self) -> bool {
        self.is_valid()
    }
    fn cast(& self, allowedFields__364: temper_core::List<SafeIdentifier>) -> Changeset {
        self.cast(allowedFields__364)
    }
    fn validate_required(& self, fields__370: temper_core::List<SafeIdentifier>) -> Changeset {
        self.validate_required(fields__370)
    }
    fn validate_length(& self, field__376: SafeIdentifier, min__377: i32, max__378: i32) -> Changeset {
        self.validate_length(field__376, min__377, max__378)
    }
    fn validate_int(& self, field__385: SafeIdentifier) -> Changeset {
        self.validate_int(field__385)
    }
    fn validate_int64(& self, field__391: SafeIdentifier) -> Changeset {
        self.validate_int64(field__391)
    }
    fn validate_float(& self, field__397: SafeIdentifier) -> Changeset {
        self.validate_float(field__397)
    }
    fn validate_bool(& self, field__403: SafeIdentifier) -> Changeset {
        self.validate_bool(field__403)
    }
    fn to_insert_sql(& self) -> temper_core::Result<SqlFragment> {
        self.to_insert_sql()
    }
    fn to_update_sql(& self, id__431: i32) -> temper_core::Result<SqlFragment> {
        self.to_update_sql(id__431)
    }
}
temper_core::impl_any_value_trait!(ChangesetImpl, [Changeset]);
struct OrderClauseStruct {
    field: SafeIdentifier, ascending: bool
}
#[derive(Clone)]
pub struct OrderClause(std::sync::Arc<OrderClauseStruct>);
#[derive(Clone)]
pub struct OrderClauseBuilder {
    pub field: SafeIdentifier, pub ascending: bool
}
impl OrderClauseBuilder {
    pub fn build(self) -> OrderClause {
        OrderClause::new(self.field, self.ascending)
    }
}
impl OrderClause {
    pub fn new(field__541: SafeIdentifier, ascending__542: bool) -> OrderClause {
        let field;
        let ascending;
        field = field__541.clone();
        ascending = ascending__542;
        let selfish = OrderClause(std::sync::Arc::new(OrderClauseStruct {
                    field, ascending
        }));
        return selfish;
    }
    pub fn field(& self) -> SafeIdentifier {
        return self.0.field.clone();
    }
    pub fn ascending(& self) -> bool {
        return self.0.ascending;
    }
}
temper_core::impl_any_value_trait!(OrderClause, []);
struct QueryStruct {
    table_name: SafeIdentifier, conditions: temper_core::List<SqlFragment>, selected_fields: temper_core::List<SafeIdentifier>, order_clauses: temper_core::List<OrderClause>, limit_val: Option<i32>, offset_val: Option<i32>
}
#[derive(Clone)]
pub struct Query(std::sync::Arc<QueryStruct>);
#[derive(Clone)]
pub struct QueryBuilder {
    pub table_name: SafeIdentifier, pub conditions: temper_core::List<SqlFragment>, pub selected_fields: temper_core::List<SafeIdentifier>, pub order_clauses: temper_core::List<OrderClause>, pub limit_val: Option<i32>, pub offset_val: Option<i32>
}
impl QueryBuilder {
    pub fn build(self) -> Query {
        Query::new(self.table_name, self.conditions, self.selected_fields, self.order_clauses, self.limit_val, self.offset_val)
    }
}
impl Query {
    pub fn r#where(& self, condition__550: SqlFragment) -> Query {
        let nb__552: temper_core::ListBuilder<SqlFragment> = temper_core::ListedTrait::to_list_builder( & self.0.conditions);
        temper_core::listed::add( & nb__552, condition__550.clone(), None);
        return Query::new(self.0.table_name.clone(), temper_core::ListedTrait::to_list( & nb__552), self.0.selected_fields.clone(), self.0.order_clauses.clone(), self.0.limit_val, self.0.offset_val);
    }
    pub fn select(& self, fields__554: impl temper_core::ToList<SafeIdentifier>) -> Query {
        let fields__554 = fields__554.to_list();
        return Query::new(self.0.table_name.clone(), self.0.conditions.clone(), fields__554.clone(), self.0.order_clauses.clone(), self.0.limit_val, self.0.offset_val);
    }
    pub fn order_by(& self, field__557: SafeIdentifier, ascending__558: bool) -> Query {
        let nb__560: temper_core::ListBuilder<OrderClause> = temper_core::ListedTrait::to_list_builder( & self.0.order_clauses);
        temper_core::listed::add( & nb__560, OrderClause::new(field__557.clone(), ascending__558), None);
        return Query::new(self.0.table_name.clone(), self.0.conditions.clone(), self.0.selected_fields.clone(), temper_core::ListedTrait::to_list( & nb__560), self.0.limit_val, self.0.offset_val);
    }
    pub fn limit(& self, n__562: i32) -> temper_core::Result<Query> {
        if Some(n__562) < Some(0) {
            return Err(temper_core::Error::new());
        }
        return Ok(Query::new(self.0.table_name.clone(), self.0.conditions.clone(), self.0.selected_fields.clone(), self.0.order_clauses.clone(), Some(n__562), self.0.offset_val));
    }
    pub fn offset(& self, n__565: i32) -> temper_core::Result<Query> {
        if Some(n__565) < Some(0) {
            return Err(temper_core::Error::new());
        }
        return Ok(Query::new(self.0.table_name.clone(), self.0.conditions.clone(), self.0.selected_fields.clone(), self.0.order_clauses.clone(), self.0.limit_val, Some(n__565)));
    }
    pub fn to_sql(& self) -> SqlFragment {
        let mut t___4466: i32;
        let b__569: SqlBuilder = SqlBuilder::new();
        b__569.append_safe("SELECT ");
        if temper_core::ListedTrait::is_empty( & self.0.selected_fields) {
            b__569.append_safe("*");
        } else {
            #[derive(Clone)]
            struct ClosureGroup___4 {}
            impl ClosureGroup___4 {
                fn fn__4451(& self, f__570: SafeIdentifier) -> std::sync::Arc<String> {
                    return f__570.sql_value();
                }
            }
            let closure_group = ClosureGroup___4 {};
            let fn__4451 = {
                let closure_group = closure_group.clone();
                std::sync::Arc::new(move | f__570: SafeIdentifier | closure_group.fn__4451(f__570))
            };
            b__569.append_safe(temper_core::listed::join( & self.0.selected_fields, std::sync::Arc::new(", ".to_string()), & ( * fn__4451.clone())));
        }
        b__569.append_safe(" FROM ");
        b__569.append_safe(self.0.table_name.sql_value());
        if ! temper_core::ListedTrait::is_empty( & self.0.conditions) {
            b__569.append_safe(" WHERE ");
            b__569.append_fragment(temper_core::ListedTrait::get( & self.0.conditions, 0));
            let mut i__571: i32 = 1;
            'loop___5195: loop {
                t___4466 = temper_core::ListedTrait::len( & self.0.conditions);
                if ! (Some(i__571) < Some(t___4466)) {
                    break;
                }
                b__569.append_safe(" AND ");
                b__569.append_fragment(temper_core::ListedTrait::get( & self.0.conditions, i__571));
                i__571 = i__571.wrapping_add(1);
            }
        }
        if ! temper_core::ListedTrait::is_empty( & self.0.order_clauses) {
            b__569.append_safe(" ORDER BY ");
            let mut first__572: std::sync::Arc<std::sync::RwLock<bool>> = std::sync::Arc::new(std::sync::RwLock::new(true));
            #[derive(Clone)]
            struct ClosureGroup___5 {
                first__572: std::sync::Arc<std::sync::RwLock<bool>>, b__569: SqlBuilder
            }
            impl ClosureGroup___5 {
                fn fn__4450(& self, oc__573: OrderClause) {
                    let mut t___2442: std::sync::Arc<String>;
                    if ! temper_core::read_locked( & self.first__572) {
                        self.b__569.append_safe(", ");
                    }
                    {
                        * self.first__572.write().unwrap() = false;
                    }
                    let mut t___4445: std::sync::Arc<String> = oc__573.field().sql_value();
                    self.b__569.append_safe(t___4445.clone());
                    if oc__573.ascending() {
                        t___2442 = std::sync::Arc::new(" ASC".to_string());
                    } else {
                        t___2442 = std::sync::Arc::new(" DESC".to_string());
                    }
                    self.b__569.append_safe(t___2442.clone());
                }
            }
            let closure_group = ClosureGroup___5 {
                first__572: first__572.clone(), b__569: b__569.clone()
            };
            let fn__4450 = {
                let closure_group = closure_group.clone();
                std::sync::Arc::new(move | oc__573: OrderClause | closure_group.fn__4450(oc__573))
            };
            temper_core::listed::list_for_each( & self.0.order_clauses, & ( * fn__4450.clone()));
        }
        let lv__574: Option<i32> = self.0.limit_val;
        if ! lv__574.is_none() {
            let lv___1116: i32 = lv__574.unwrap();
            b__569.append_safe(" LIMIT ");
            b__569.append_int32(lv___1116);
        }
        let ov__575: Option<i32> = self.0.offset_val;
        if ! ov__575.is_none() {
            let ov___1117: i32 = ov__575.unwrap();
            b__569.append_safe(" OFFSET ");
            b__569.append_int32(ov___1117);
        }
        return b__569.accumulated();
    }
    pub fn safe_to_sql(& self, defaultLimit__577: i32) -> temper_core::Result<SqlFragment> {
        let return__221: SqlFragment;
        let mut t___2435: Query;
        if Some(defaultLimit__577) < Some(0) {
            return Err(temper_core::Error::new());
        }
        if ! self.0.limit_val.is_none() {
            return__221 = self.to_sql();
        } else {
            t___2435 = self.limit(defaultLimit__577) ? ;
            return__221 = t___2435.to_sql();
        }
        return Ok(return__221.clone());
    }
    pub fn new(tableName__580: SafeIdentifier, conditions__581: impl temper_core::ToList<SqlFragment>, selectedFields__582: impl temper_core::ToList<SafeIdentifier>, orderClauses__583: impl temper_core::ToList<OrderClause>, limitVal__584: Option<i32>, offsetVal__585: Option<i32>) -> Query {
        let conditions__581 = conditions__581.to_list();
        let selectedFields__582 = selectedFields__582.to_list();
        let orderClauses__583 = orderClauses__583.to_list();
        let table_name;
        let conditions;
        let selected_fields;
        let order_clauses;
        let limit_val;
        let offset_val;
        table_name = tableName__580.clone();
        conditions = conditions__581.clone();
        selected_fields = selectedFields__582.clone();
        order_clauses = orderClauses__583.clone();
        limit_val = limitVal__584;
        offset_val = offsetVal__585;
        let selfish = Query(std::sync::Arc::new(QueryStruct {
                    table_name, conditions, selected_fields, order_clauses, limit_val, offset_val
        }));
        return selfish;
    }
    pub fn table_name(& self) -> SafeIdentifier {
        return self.0.table_name.clone();
    }
    pub fn conditions(& self) -> temper_core::List<SqlFragment> {
        return self.0.conditions.clone();
    }
    pub fn selected_fields(& self) -> temper_core::List<SafeIdentifier> {
        return self.0.selected_fields.clone();
    }
    pub fn order_clauses(& self) -> temper_core::List<OrderClause> {
        return self.0.order_clauses.clone();
    }
    pub fn limit_val(& self) -> Option<i32> {
        return self.0.limit_val;
    }
    pub fn offset_val(& self) -> Option<i32> {
        return self.0.offset_val;
    }
}
temper_core::impl_any_value_trait!(Query, []);
pub enum SafeIdentifierEnum {
    ValidatedIdentifier(ValidatedIdentifier)
}
pub trait SafeIdentifierTrait: temper_core::AsAnyValue + temper_core::AnyValueTrait + std::marker::Send + std::marker::Sync {
    fn as_enum(& self) -> SafeIdentifierEnum;
    fn clone_boxed(& self) -> SafeIdentifier;
    fn sql_value(& self) -> std::sync::Arc<String>;
}
#[derive(Clone)]
pub struct SafeIdentifier(std::sync::Arc<dyn SafeIdentifierTrait>);
impl SafeIdentifier {
    pub fn new(selfish: impl SafeIdentifierTrait + 'static) -> SafeIdentifier {
        SafeIdentifier(std::sync::Arc::new(selfish))
    }
}
impl SafeIdentifierTrait for SafeIdentifier {
    fn as_enum(& self) -> SafeIdentifierEnum {
        SafeIdentifierTrait::as_enum( & ( * self.0))
    }
    fn clone_boxed(& self) -> SafeIdentifier {
        SafeIdentifierTrait::clone_boxed( & ( * self.0))
    }
    fn sql_value(& self) -> std::sync::Arc<String> {
        SafeIdentifierTrait::sql_value( & ( * self.0))
    }
}
temper_core::impl_any_value_trait_for_interface!(SafeIdentifier);
impl std::ops::Deref for SafeIdentifier {
    type Target = dyn SafeIdentifierTrait;
    fn deref(& self) -> & Self::Target {
        & ( * self.0)
    }
}
struct ValidatedIdentifierStruct {
    value: std::sync::Arc<String>
}
#[derive(Clone)]
pub (crate) struct ValidatedIdentifier(std::sync::Arc<ValidatedIdentifierStruct>);
impl ValidatedIdentifier {
    pub fn sql_value(& self) -> std::sync::Arc<String> {
        return self.0.value.clone();
    }
    pub fn new(_value__634: impl temper_core::ToArcString) -> ValidatedIdentifier {
        let _value__634 = _value__634.to_arc_string();
        let value;
        value = _value__634.clone();
        let selfish = ValidatedIdentifier(std::sync::Arc::new(ValidatedIdentifierStruct {
                    value
        }));
        return selfish;
    }
}
impl SafeIdentifierTrait for ValidatedIdentifier {
    fn as_enum(& self) -> SafeIdentifierEnum {
        SafeIdentifierEnum::ValidatedIdentifier(self.clone())
    }
    fn clone_boxed(& self) -> SafeIdentifier {
        SafeIdentifier::new(self.clone())
    }
    fn sql_value(& self) -> std::sync::Arc<String> {
        self.sql_value()
    }
}
temper_core::impl_any_value_trait!(ValidatedIdentifier, [SafeIdentifier]);
pub enum FieldTypeEnum {
    StringField(StringField), IntField(IntField), Int64Field(Int64Field), FloatField(FloatField), BoolField(BoolField), DateField(DateField)
}
pub trait FieldTypeTrait: temper_core::AsAnyValue + temper_core::AnyValueTrait + std::marker::Send + std::marker::Sync {
    fn as_enum(& self) -> FieldTypeEnum;
    fn clone_boxed(& self) -> FieldType;
}
#[derive(Clone)]
pub struct FieldType(std::sync::Arc<dyn FieldTypeTrait>);
impl FieldType {
    pub fn new(selfish: impl FieldTypeTrait + 'static) -> FieldType {
        FieldType(std::sync::Arc::new(selfish))
    }
}
impl FieldTypeTrait for FieldType {
    fn as_enum(& self) -> FieldTypeEnum {
        FieldTypeTrait::as_enum( & ( * self.0))
    }
    fn clone_boxed(& self) -> FieldType {
        FieldTypeTrait::clone_boxed( & ( * self.0))
    }
}
temper_core::impl_any_value_trait_for_interface!(FieldType);
impl std::ops::Deref for FieldType {
    type Target = dyn FieldTypeTrait;
    fn deref(& self) -> & Self::Target {
        & ( * self.0)
    }
}
struct StringFieldStruct {}
#[derive(Clone)]
pub struct StringField(std::sync::Arc<StringFieldStruct>);
impl StringField {
    pub fn new() -> StringField {
        let selfish = StringField(std::sync::Arc::new(StringFieldStruct {}));
        return selfish;
    }
}
impl FieldTypeTrait for StringField {
    fn as_enum(& self) -> FieldTypeEnum {
        FieldTypeEnum::StringField(self.clone())
    }
    fn clone_boxed(& self) -> FieldType {
        FieldType::new(self.clone())
    }
}
temper_core::impl_any_value_trait!(StringField, [FieldType]);
struct IntFieldStruct {}
#[derive(Clone)]
pub struct IntField(std::sync::Arc<IntFieldStruct>);
impl IntField {
    pub fn new() -> IntField {
        let selfish = IntField(std::sync::Arc::new(IntFieldStruct {}));
        return selfish;
    }
}
impl FieldTypeTrait for IntField {
    fn as_enum(& self) -> FieldTypeEnum {
        FieldTypeEnum::IntField(self.clone())
    }
    fn clone_boxed(& self) -> FieldType {
        FieldType::new(self.clone())
    }
}
temper_core::impl_any_value_trait!(IntField, [FieldType]);
struct Int64FieldStruct {}
#[derive(Clone)]
pub struct Int64Field(std::sync::Arc<Int64FieldStruct>);
impl Int64Field {
    pub fn new() -> Int64Field {
        let selfish = Int64Field(std::sync::Arc::new(Int64FieldStruct {}));
        return selfish;
    }
}
impl FieldTypeTrait for Int64Field {
    fn as_enum(& self) -> FieldTypeEnum {
        FieldTypeEnum::Int64Field(self.clone())
    }
    fn clone_boxed(& self) -> FieldType {
        FieldType::new(self.clone())
    }
}
temper_core::impl_any_value_trait!(Int64Field, [FieldType]);
struct FloatFieldStruct {}
#[derive(Clone)]
pub struct FloatField(std::sync::Arc<FloatFieldStruct>);
impl FloatField {
    pub fn new() -> FloatField {
        let selfish = FloatField(std::sync::Arc::new(FloatFieldStruct {}));
        return selfish;
    }
}
impl FieldTypeTrait for FloatField {
    fn as_enum(& self) -> FieldTypeEnum {
        FieldTypeEnum::FloatField(self.clone())
    }
    fn clone_boxed(& self) -> FieldType {
        FieldType::new(self.clone())
    }
}
temper_core::impl_any_value_trait!(FloatField, [FieldType]);
struct BoolFieldStruct {}
#[derive(Clone)]
pub struct BoolField(std::sync::Arc<BoolFieldStruct>);
impl BoolField {
    pub fn new() -> BoolField {
        let selfish = BoolField(std::sync::Arc::new(BoolFieldStruct {}));
        return selfish;
    }
}
impl FieldTypeTrait for BoolField {
    fn as_enum(& self) -> FieldTypeEnum {
        FieldTypeEnum::BoolField(self.clone())
    }
    fn clone_boxed(& self) -> FieldType {
        FieldType::new(self.clone())
    }
}
temper_core::impl_any_value_trait!(BoolField, [FieldType]);
struct DateFieldStruct {}
#[derive(Clone)]
pub struct DateField(std::sync::Arc<DateFieldStruct>);
impl DateField {
    pub fn new() -> DateField {
        let selfish = DateField(std::sync::Arc::new(DateFieldStruct {}));
        return selfish;
    }
}
impl FieldTypeTrait for DateField {
    fn as_enum(& self) -> FieldTypeEnum {
        FieldTypeEnum::DateField(self.clone())
    }
    fn clone_boxed(& self) -> FieldType {
        FieldType::new(self.clone())
    }
}
temper_core::impl_any_value_trait!(DateField, [FieldType]);
struct FieldDefStruct {
    name: SafeIdentifier, field_type: FieldType, nullable: bool
}
#[derive(Clone)]
pub struct FieldDef(std::sync::Arc<FieldDefStruct>);
#[derive(Clone)]
pub struct FieldDefBuilder {
    pub name: SafeIdentifier, pub field_type: FieldType, pub nullable: bool
}
impl FieldDefBuilder {
    pub fn build(self) -> FieldDef {
        FieldDef::new(self.name, self.field_type, self.nullable)
    }
}
impl FieldDef {
    pub fn new(name__652: SafeIdentifier, fieldType__653: FieldType, nullable__654: bool) -> FieldDef {
        let name;
        let field_type;
        let nullable;
        name = name__652.clone();
        field_type = fieldType__653.clone();
        nullable = nullable__654;
        let selfish = FieldDef(std::sync::Arc::new(FieldDefStruct {
                    name, field_type, nullable
        }));
        return selfish;
    }
    pub fn name(& self) -> SafeIdentifier {
        return self.0.name.clone();
    }
    pub fn field_type(& self) -> FieldType {
        return self.0.field_type.clone();
    }
    pub fn nullable(& self) -> bool {
        return self.0.nullable;
    }
}
temper_core::impl_any_value_trait!(FieldDef, []);
struct TableDefStruct {
    table_name: SafeIdentifier, fields: temper_core::List<FieldDef>
}
#[derive(Clone)]
pub struct TableDef(std::sync::Arc<TableDefStruct>);
#[derive(Clone)]
pub struct TableDefBuilder {
    pub table_name: SafeIdentifier, pub fields: temper_core::List<FieldDef>
}
impl TableDefBuilder {
    pub fn build(self) -> TableDef {
        TableDef::new(self.table_name, self.fields)
    }
}
impl TableDef {
    pub fn field(& self, name__658: impl temper_core::ToArcString) -> temper_core::Result<FieldDef> {
        let name__658 = name__658.to_arc_string();
        let return__250: FieldDef;
        'fn__659: {
            let this__3156: temper_core::List<FieldDef> = self.0.fields.clone();
            let n__3157: i32 = temper_core::ListedTrait::len( & this__3156);
            let mut i__3158: i32 = 0;
            'loop___5202: while Some(i__3158) < Some(n__3157) {
                let el__3159: FieldDef = temper_core::ListedTrait::get( & this__3156, i__3158);
                i__3158 = i__3158.wrapping_add(1);
                let f__660: FieldDef = el__3159.clone();
                if Some(f__660.name().sql_value().as_str()) == Some(name__658.as_str()) {
                    return__250 = f__660.clone();
                    break 'fn__659;
                }
            }
            return Err(temper_core::Error::new());
        }
        return Ok(return__250.clone());
    }
    pub fn new(tableName__662: SafeIdentifier, fields__663: impl temper_core::ToList<FieldDef>) -> TableDef {
        let fields__663 = fields__663.to_list();
        let table_name;
        let fields;
        table_name = tableName__662.clone();
        fields = fields__663.clone();
        let selfish = TableDef(std::sync::Arc::new(TableDefStruct {
                    table_name, fields
        }));
        return selfish;
    }
    pub fn table_name(& self) -> SafeIdentifier {
        return self.0.table_name.clone();
    }
    pub fn fields(& self) -> temper_core::List<FieldDef> {
        return self.0.fields.clone();
    }
}
temper_core::impl_any_value_trait!(TableDef, []);
struct SqlBuilderStruct {
    buffer: temper_core::ListBuilder<SqlPart>
}
#[derive(Clone)]
pub struct SqlBuilder(std::sync::Arc<SqlBuilderStruct>);
impl SqlBuilder {
    pub fn append_safe(& self, sqlSource__685: impl temper_core::ToArcString) {
        let sqlSource__685 = sqlSource__685.to_arc_string();
        let mut t___5072: SqlSource = SqlSource::new(sqlSource__685.clone());
        temper_core::listed::add( & self.0.buffer, SqlPart::new(t___5072.clone()), None);
    }
    pub fn append_fragment(& self, fragment__688: SqlFragment) {
        let mut t___5070: temper_core::List<SqlPart> = fragment__688.parts();
        temper_core::listed::add_all( & self.0.buffer, temper_core::ToListed::to_listed(t___5070.clone()), None);
    }
    pub fn append_part(& self, part__691: SqlPart) {
        temper_core::listed::add( & self.0.buffer, part__691.clone(), None);
    }
    pub fn append_part_list(& self, values__694: impl temper_core::ToList<SqlPart>) {
        let values__694 = values__694.to_list();
        #[derive(Clone)]
        struct ClosureGroup___6 {
            this__130: SqlBuilder
        }
        impl ClosureGroup___6 {
            fn fn__5066(& self, x__696: SqlPart) {
                self.this__130.append_part(x__696.clone());
            }
        }
        let closure_group = ClosureGroup___6 {
            this__130: self.clone()
        };
        let fn__5066 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | x__696: SqlPart | closure_group.fn__5066(x__696))
        };
        self.append_list(temper_core::ToListed::to_listed(values__694.clone()), fn__5066.clone());
    }
    pub fn append_boolean(& self, value__698: bool) {
        let mut t___5063: SqlBoolean = SqlBoolean::new(value__698);
        temper_core::listed::add( & self.0.buffer, SqlPart::new(t___5063.clone()), None);
    }
    pub fn append_boolean_list(& self, values__701: impl temper_core::ToListed<bool>) {
        let values__701 = values__701.to_listed();
        #[derive(Clone)]
        struct ClosureGroup___7 {
            this__132: SqlBuilder
        }
        impl ClosureGroup___7 {
            fn fn__5060(& self, x__703: bool) {
                self.this__132.append_boolean(x__703);
            }
        }
        let closure_group = ClosureGroup___7 {
            this__132: self.clone()
        };
        let fn__5060 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | x__703: bool | closure_group.fn__5060(x__703))
        };
        self.append_list(values__701.clone(), fn__5060.clone());
    }
    pub fn append_date(& self, value__705: temper_std::temporal::Date) {
        let mut t___5057: SqlDate = SqlDate::new(value__705.clone());
        temper_core::listed::add( & self.0.buffer, SqlPart::new(t___5057.clone()), None);
    }
    pub fn append_date_list(& self, values__708: impl temper_core::ToListed<temper_std::temporal::Date>) {
        let values__708 = values__708.to_listed();
        #[derive(Clone)]
        struct ClosureGroup___8 {
            this__134: SqlBuilder
        }
        impl ClosureGroup___8 {
            fn fn__5054(& self, x__710: temper_std::temporal::Date) {
                self.this__134.append_date(x__710.clone());
            }
        }
        let closure_group = ClosureGroup___8 {
            this__134: self.clone()
        };
        let fn__5054 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | x__710: temper_std::temporal::Date | closure_group.fn__5054(x__710))
        };
        self.append_list(values__708.clone(), fn__5054.clone());
    }
    pub fn append_float64(& self, value__712: f64) {
        let mut t___5051: SqlFloat64 = SqlFloat64::new(value__712);
        temper_core::listed::add( & self.0.buffer, SqlPart::new(t___5051.clone()), None);
    }
    pub fn append_float64_list(& self, values__715: impl temper_core::ToListed<f64>) {
        let values__715 = values__715.to_listed();
        #[derive(Clone)]
        struct ClosureGroup___9 {
            this__136: SqlBuilder
        }
        impl ClosureGroup___9 {
            fn fn__5048(& self, x__717: f64) {
                self.this__136.append_float64(x__717);
            }
        }
        let closure_group = ClosureGroup___9 {
            this__136: self.clone()
        };
        let fn__5048 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | x__717: f64 | closure_group.fn__5048(x__717))
        };
        self.append_list(values__715.clone(), fn__5048.clone());
    }
    pub fn append_int32(& self, value__719: i32) {
        let mut t___5045: SqlInt32 = SqlInt32::new(value__719);
        temper_core::listed::add( & self.0.buffer, SqlPart::new(t___5045.clone()), None);
    }
    pub fn append_int32_list(& self, values__722: impl temper_core::ToListed<i32>) {
        let values__722 = values__722.to_listed();
        #[derive(Clone)]
        struct ClosureGroup___10 {
            this__138: SqlBuilder
        }
        impl ClosureGroup___10 {
            fn fn__5042(& self, x__724: i32) {
                self.this__138.append_int32(x__724);
            }
        }
        let closure_group = ClosureGroup___10 {
            this__138: self.clone()
        };
        let fn__5042 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | x__724: i32 | closure_group.fn__5042(x__724))
        };
        self.append_list(values__722.clone(), fn__5042.clone());
    }
    pub fn append_int64(& self, value__726: i64) {
        let mut t___5039: SqlInt64 = SqlInt64::new(value__726);
        temper_core::listed::add( & self.0.buffer, SqlPart::new(t___5039.clone()), None);
    }
    pub fn append_int64_list(& self, values__729: impl temper_core::ToListed<i64>) {
        let values__729 = values__729.to_listed();
        #[derive(Clone)]
        struct ClosureGroup___11 {
            this__140: SqlBuilder
        }
        impl ClosureGroup___11 {
            fn fn__5036(& self, x__731: i64) {
                self.this__140.append_int64(x__731);
            }
        }
        let closure_group = ClosureGroup___11 {
            this__140: self.clone()
        };
        let fn__5036 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | x__731: i64 | closure_group.fn__5036(x__731))
        };
        self.append_list(values__729.clone(), fn__5036.clone());
    }
    pub fn append_string(& self, value__733: impl temper_core::ToArcString) {
        let value__733 = value__733.to_arc_string();
        let mut t___5033: SqlString = SqlString::new(value__733.clone());
        temper_core::listed::add( & self.0.buffer, SqlPart::new(t___5033.clone()), None);
    }
    pub fn append_string_list(& self, values__736: impl temper_core::ToListed<std::sync::Arc<String>>) {
        let values__736 = values__736.to_listed();
        #[derive(Clone)]
        struct ClosureGroup___12 {
            this__142: SqlBuilder
        }
        impl ClosureGroup___12 {
            fn fn__5030(& self, x__738: impl temper_core::ToArcString) {
                let x__738 = x__738.to_arc_string();
                self.this__142.append_string(x__738.clone());
            }
        }
        let closure_group = ClosureGroup___12 {
            this__142: self.clone()
        };
        let fn__5030 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | x__738: std::sync::Arc<String> | closure_group.fn__5030(x__738))
        };
        self.append_list(values__736.clone(), fn__5030.clone());
    }
    fn append_list<T>(& self, values__740: impl temper_core::ToListed<T>, appendValue__741: std::sync::Arc<dyn Fn (T) + std::marker::Send + std::marker::Sync>) where T: Clone + std::marker::Send + std::marker::Sync + 'static {
        let values__740 = values__740.to_listed();
        let mut t___5025: i32;
        let mut t___5027: T;
        let mut i__743: i32 = 0;
        'loop___5203: loop {
            t___5025 = temper_core::ListedTrait::len( & ( * values__740));
            if ! (Some(i__743) < Some(t___5025)) {
                break;
            }
            if Some(i__743) > Some(0) {
                self.append_safe(", ");
            }
            t___5027 = temper_core::ListedTrait::get( & ( * values__740), i__743);
            appendValue__741(t___5027.clone());
            i__743 = i__743.wrapping_add(1);
        }
    }
    pub fn accumulated(& self) -> SqlFragment {
        return SqlFragment::new(temper_core::ListedTrait::to_list( & self.0.buffer));
    }
    pub fn new() -> SqlBuilder {
        let buffer;
        let mut t___5022: temper_core::ListBuilder<SqlPart> = temper_core::listed::new_builder();
        buffer = t___5022.clone();
        let selfish = SqlBuilder(std::sync::Arc::new(SqlBuilderStruct {
                    buffer
        }));
        return selfish;
    }
}
temper_core::impl_any_value_trait!(SqlBuilder, []);
struct SqlFragmentStruct {
    parts: temper_core::List<SqlPart>
}
#[derive(Clone)]
pub struct SqlFragment(std::sync::Arc<SqlFragmentStruct>);
impl SqlFragment {
    pub fn to_source(& self) -> SqlSource {
        return SqlSource::new(self.to_string());
    }
    pub fn to_string(& self) -> std::sync::Arc<String> {
        let mut t___5096: i32;
        let builder__755: std::sync::Arc<std::sync::RwLock<String>> = std::sync::Arc::new(std::sync::RwLock::new(String::new()));
        let mut i__756: i32 = 0;
        'loop___5204: loop {
            t___5096 = temper_core::ListedTrait::len( & self.0.parts);
            if ! (Some(i__756) < Some(t___5096)) {
                break;
            }
            temper_core::ListedTrait::get( & self.0.parts, i__756).format_to(builder__755.clone());
            i__756 = i__756.wrapping_add(1);
        }
        return temper_core::string::builder::to_string( & builder__755);
    }
    pub fn new(parts__758: impl temper_core::ToList<SqlPart>) -> SqlFragment {
        let parts__758 = parts__758.to_list();
        let parts;
        parts = parts__758.clone();
        let selfish = SqlFragment(std::sync::Arc::new(SqlFragmentStruct {
                    parts
        }));
        return selfish;
    }
    pub fn parts(& self) -> temper_core::List<SqlPart> {
        return self.0.parts.clone();
    }
}
temper_core::impl_any_value_trait!(SqlFragment, []);
pub enum SqlPartEnum {
    SqlSource(SqlSource), SqlBoolean(SqlBoolean), SqlString(SqlString), SqlInt32(SqlInt32), SqlInt64(SqlInt64), SqlFloat64(SqlFloat64), SqlDate(SqlDate)
}
pub trait SqlPartTrait: temper_core::AsAnyValue + temper_core::AnyValueTrait + std::marker::Send + std::marker::Sync {
    fn as_enum(& self) -> SqlPartEnum;
    fn clone_boxed(& self) -> SqlPart;
    fn format_to(& self, builder__760: std::sync::Arc<std::sync::RwLock<String>>);
}
#[derive(Clone)]
pub struct SqlPart(std::sync::Arc<dyn SqlPartTrait>);
impl SqlPart {
    pub fn new(selfish: impl SqlPartTrait + 'static) -> SqlPart {
        SqlPart(std::sync::Arc::new(selfish))
    }
}
impl SqlPartTrait for SqlPart {
    fn as_enum(& self) -> SqlPartEnum {
        SqlPartTrait::as_enum( & ( * self.0))
    }
    fn clone_boxed(& self) -> SqlPart {
        SqlPartTrait::clone_boxed( & ( * self.0))
    }
    fn format_to(& self, arg1: std::sync::Arc<std::sync::RwLock<String>>) -> () {
        SqlPartTrait::format_to( & ( * self.0), arg1)
    }
}
temper_core::impl_any_value_trait_for_interface!(SqlPart);
impl std::ops::Deref for SqlPart {
    type Target = dyn SqlPartTrait;
    fn deref(& self) -> & Self::Target {
        & ( * self.0)
    }
}
struct SqlSourceStruct {
    source: std::sync::Arc<String>
}
#[derive(Clone)]
pub struct SqlSource(std::sync::Arc<SqlSourceStruct>);
impl SqlSource {
    pub fn format_to(& self, builder__764: std::sync::Arc<std::sync::RwLock<String>>) {
        temper_core::string::builder::append( & builder__764, self.0.source.clone());
    }
    pub fn new(source__767: impl temper_core::ToArcString) -> SqlSource {
        let source__767 = source__767.to_arc_string();
        let source;
        source = source__767.clone();
        let selfish = SqlSource(std::sync::Arc::new(SqlSourceStruct {
                    source
        }));
        return selfish;
    }
    pub fn source(& self) -> std::sync::Arc<String> {
        return self.0.source.clone();
    }
}
impl SqlPartTrait for SqlSource {
    fn as_enum(& self) -> SqlPartEnum {
        SqlPartEnum::SqlSource(self.clone())
    }
    fn clone_boxed(& self) -> SqlPart {
        SqlPart::new(self.clone())
    }
    fn format_to(& self, builder__764: std::sync::Arc<std::sync::RwLock<String>>) {
        self.format_to(builder__764)
    }
}
temper_core::impl_any_value_trait!(SqlSource, [SqlPart]);
struct SqlBooleanStruct {
    value: bool
}
#[derive(Clone)]
pub struct SqlBoolean(std::sync::Arc<SqlBooleanStruct>);
impl SqlBoolean {
    pub fn format_to(& self, builder__770: std::sync::Arc<std::sync::RwLock<String>>) {
        let mut t___3032: std::sync::Arc<String>;
        if self.0.value {
            t___3032 = std::sync::Arc::new("TRUE".to_string());
        } else {
            t___3032 = std::sync::Arc::new("FALSE".to_string());
        }
        temper_core::string::builder::append( & builder__770, t___3032.clone());
    }
    pub fn new(value__773: bool) -> SqlBoolean {
        let value;
        value = value__773;
        let selfish = SqlBoolean(std::sync::Arc::new(SqlBooleanStruct {
                    value
        }));
        return selfish;
    }
    pub fn value(& self) -> bool {
        return self.0.value;
    }
}
impl SqlPartTrait for SqlBoolean {
    fn as_enum(& self) -> SqlPartEnum {
        SqlPartEnum::SqlBoolean(self.clone())
    }
    fn clone_boxed(& self) -> SqlPart {
        SqlPart::new(self.clone())
    }
    fn format_to(& self, builder__770: std::sync::Arc<std::sync::RwLock<String>>) {
        self.format_to(builder__770)
    }
}
temper_core::impl_any_value_trait!(SqlBoolean, [SqlPart]);
struct SqlDateStruct {
    value: temper_std::temporal::Date
}
#[derive(Clone)]
pub struct SqlDate(std::sync::Arc<SqlDateStruct>);
impl SqlDate {
    pub fn format_to(& self, builder__776: std::sync::Arc<std::sync::RwLock<String>>) {
        temper_core::string::builder::append( & builder__776, "'");
        let mut t___5077: std::sync::Arc<String> = self.0.value.to_string();
        #[derive(Clone)]
        struct ClosureGroup___13 {
            builder__776: std::sync::Arc<std::sync::RwLock<String>>
        }
        impl ClosureGroup___13 {
            fn fn__5075(& self, c__778: i32) {
                if Some(c__778) == Some(39) {
                    temper_core::string::builder::append( & self.builder__776, "''");
                } else {
                    'ok___5135: {
                        'orelse___1071: {
                            match temper_core::string::builder::append_code_point( & self.builder__776, c__778) {
                                Ok(x) => x,
                                _ => break 'orelse___1071
                            };
                            break 'ok___5135;
                        }
                        return panic!();
                    }
                }
            }
        }
        let closure_group = ClosureGroup___13 {
            builder__776: builder__776.clone()
        };
        let fn__5075 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | c__778: i32 | closure_group.fn__5075(c__778))
        };
        temper_core::string::for_each( & t___5077, & ( * fn__5075.clone()));
        temper_core::string::builder::append( & builder__776, "'");
    }
    pub fn new(value__780: temper_std::temporal::Date) -> SqlDate {
        let value;
        value = value__780.clone();
        let selfish = SqlDate(std::sync::Arc::new(SqlDateStruct {
                    value
        }));
        return selfish;
    }
    pub fn value(& self) -> temper_std::temporal::Date {
        return self.0.value.clone();
    }
}
impl SqlPartTrait for SqlDate {
    fn as_enum(& self) -> SqlPartEnum {
        SqlPartEnum::SqlDate(self.clone())
    }
    fn clone_boxed(& self) -> SqlPart {
        SqlPart::new(self.clone())
    }
    fn format_to(& self, builder__776: std::sync::Arc<std::sync::RwLock<String>>) {
        self.format_to(builder__776)
    }
}
temper_core::impl_any_value_trait!(SqlDate, [SqlPart]);
struct SqlFloat64Struct {
    value: f64
}
#[derive(Clone)]
pub struct SqlFloat64(std::sync::Arc<SqlFloat64Struct>);
impl SqlFloat64 {
    pub fn format_to(& self, builder__783: std::sync::Arc<std::sync::RwLock<String>>) {
        let mut t___3021: bool;
        let mut t___3022: bool;
        let s__785: std::sync::Arc<String> = temper_core::float64::to_string(self.0.value);
        if Some(s__785.as_str()) == Some("NaN") {
            t___3022 = true;
        } else {
            if Some(s__785.as_str()) == Some("Infinity") {
                t___3021 = true;
            } else {
                t___3021 = Some(s__785.as_str()) == Some("-Infinity");
            }
            t___3022 = t___3021;
        }
        if t___3022 {
            temper_core::string::builder::append( & builder__783, "NULL");
        } else {
            temper_core::string::builder::append( & builder__783, s__785.clone());
        }
    }
    pub fn new(value__787: f64) -> SqlFloat64 {
        let value;
        value = value__787;
        let selfish = SqlFloat64(std::sync::Arc::new(SqlFloat64Struct {
                    value
        }));
        return selfish;
    }
    pub fn value(& self) -> f64 {
        return self.0.value;
    }
}
impl SqlPartTrait for SqlFloat64 {
    fn as_enum(& self) -> SqlPartEnum {
        SqlPartEnum::SqlFloat64(self.clone())
    }
    fn clone_boxed(& self) -> SqlPart {
        SqlPart::new(self.clone())
    }
    fn format_to(& self, builder__783: std::sync::Arc<std::sync::RwLock<String>>) {
        self.format_to(builder__783)
    }
}
temper_core::impl_any_value_trait!(SqlFloat64, [SqlPart]);
struct SqlInt32Struct {
    value: i32
}
#[derive(Clone)]
pub struct SqlInt32(std::sync::Arc<SqlInt32Struct>);
impl SqlInt32 {
    pub fn format_to(& self, builder__790: std::sync::Arc<std::sync::RwLock<String>>) {
        let mut t___5086: std::sync::Arc<String> = temper_core::int_to_string(self.0.value, None);
        temper_core::string::builder::append( & builder__790, t___5086.clone());
    }
    pub fn new(value__793: i32) -> SqlInt32 {
        let value;
        value = value__793;
        let selfish = SqlInt32(std::sync::Arc::new(SqlInt32Struct {
                    value
        }));
        return selfish;
    }
    pub fn value(& self) -> i32 {
        return self.0.value;
    }
}
impl SqlPartTrait for SqlInt32 {
    fn as_enum(& self) -> SqlPartEnum {
        SqlPartEnum::SqlInt32(self.clone())
    }
    fn clone_boxed(& self) -> SqlPart {
        SqlPart::new(self.clone())
    }
    fn format_to(& self, builder__790: std::sync::Arc<std::sync::RwLock<String>>) {
        self.format_to(builder__790)
    }
}
temper_core::impl_any_value_trait!(SqlInt32, [SqlPart]);
struct SqlInt64Struct {
    value: i64
}
#[derive(Clone)]
pub struct SqlInt64(std::sync::Arc<SqlInt64Struct>);
impl SqlInt64 {
    pub fn format_to(& self, builder__796: std::sync::Arc<std::sync::RwLock<String>>) {
        let mut t___5084: std::sync::Arc<String> = temper_core::int64_to_string(self.0.value, None);
        temper_core::string::builder::append( & builder__796, t___5084.clone());
    }
    pub fn new(value__799: i64) -> SqlInt64 {
        let value;
        value = value__799;
        let selfish = SqlInt64(std::sync::Arc::new(SqlInt64Struct {
                    value
        }));
        return selfish;
    }
    pub fn value(& self) -> i64 {
        return self.0.value;
    }
}
impl SqlPartTrait for SqlInt64 {
    fn as_enum(& self) -> SqlPartEnum {
        SqlPartEnum::SqlInt64(self.clone())
    }
    fn clone_boxed(& self) -> SqlPart {
        SqlPart::new(self.clone())
    }
    fn format_to(& self, builder__796: std::sync::Arc<std::sync::RwLock<String>>) {
        self.format_to(builder__796)
    }
}
temper_core::impl_any_value_trait!(SqlInt64, [SqlPart]);
struct SqlStringStruct {
    value: std::sync::Arc<String>
}
#[derive(Clone)]
pub struct SqlString(std::sync::Arc<SqlStringStruct>);
impl SqlString {
    pub fn format_to(& self, builder__802: std::sync::Arc<std::sync::RwLock<String>>) {
        temper_core::string::builder::append( & builder__802, "'");
        #[derive(Clone)]
        struct ClosureGroup___14 {
            builder__802: std::sync::Arc<std::sync::RwLock<String>>
        }
        impl ClosureGroup___14 {
            fn fn__5089(& self, c__804: i32) {
                if Some(c__804) == Some(39) {
                    temper_core::string::builder::append( & self.builder__802, "''");
                } else {
                    'ok___5140: {
                        'orelse___1070: {
                            match temper_core::string::builder::append_code_point( & self.builder__802, c__804) {
                                Ok(x) => x,
                                _ => break 'orelse___1070
                            };
                            break 'ok___5140;
                        }
                        return panic!();
                    }
                }
            }
        }
        let closure_group = ClosureGroup___14 {
            builder__802: builder__802.clone()
        };
        let fn__5089 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | c__804: i32 | closure_group.fn__5089(c__804))
        };
        temper_core::string::for_each( & self.0.value, & ( * fn__5089.clone()));
        temper_core::string::builder::append( & builder__802, "'");
    }
    pub fn new(value__806: impl temper_core::ToArcString) -> SqlString {
        let value__806 = value__806.to_arc_string();
        let value;
        value = value__806.clone();
        let selfish = SqlString(std::sync::Arc::new(SqlStringStruct {
                    value
        }));
        return selfish;
    }
    pub fn value(& self) -> std::sync::Arc<String> {
        return self.0.value.clone();
    }
}
impl SqlPartTrait for SqlString {
    fn as_enum(& self) -> SqlPartEnum {
        SqlPartEnum::SqlString(self.clone())
    }
    fn clone_boxed(& self) -> SqlPart {
        SqlPart::new(self.clone())
    }
    fn format_to(& self, builder__802: std::sync::Arc<std::sync::RwLock<String>>) {
        self.format_to(builder__802)
    }
}
temper_core::impl_any_value_trait!(SqlString, [SqlPart]);
pub fn changeset(tableDef__444: TableDef, params__445: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>) -> Changeset {
    let mut t___4872: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & []);
    return Changeset::new(ChangesetImpl::new(tableDef__444.clone(), params__445.clone(), t___4872.clone(), [], true));
}
fn isIdentStart__305(c__635: i32) -> bool {
    let return__230: bool;
    let mut t___2795: bool;
    let mut t___2796: bool;
    if Some(c__635) >= Some(97) {
        t___2795 = Some(c__635) <= Some(122);
    } else {
        t___2795 = false;
    }
    if t___2795 {
        return__230 = true;
    } else {
        if Some(c__635) >= Some(65) {
            t___2796 = Some(c__635) <= Some(90);
        } else {
            t___2796 = false;
        }
        if t___2796 {
            return__230 = true;
        } else {
            return__230 = Some(c__635) == Some(95);
        }
    }
    return return__230;
}
fn isIdentPart__306(c__637: i32) -> bool {
    let return__231: bool;
    if isIdentStart__305(c__637) {
        return__231 = true;
    } else {
        if Some(c__637) >= Some(48) {
            return__231 = Some(c__637) <= Some(57);
        } else {
            return__231 = false;
        }
    }
    return return__231;
}
pub fn safe_identifier(name__639: impl temper_core::ToArcString) -> temper_core::Result<SafeIdentifier> {
    let name__639 = name__639.to_arc_string();
    let mut t___4870: usize;
    if name__639.is_empty() {
        return Err(temper_core::Error::new());
    }
    let mut idx__641: usize = 0usize;
    if ! isIdentStart__305(temper_core::string::get( & name__639, idx__641)) {
        return Err(temper_core::Error::new());
    }
    let mut t___4867: usize = temper_core::string::next( & name__639, idx__641);
    idx__641 = t___4867;
    'loop___5205: loop {
        if ! temper_core::string::has_index( & name__639, idx__641) {
            break;
        }
        if ! isIdentPart__306(temper_core::string::get( & name__639, idx__641)) {
            return Err(temper_core::Error::new());
        }
        t___4870 = temper_core::string::next( & name__639, idx__641);
        idx__641 = t___4870;
    }
    return Ok(SafeIdentifier::new(ValidatedIdentifier::new(name__639.clone())));
}
fn csid__302(name__447: impl temper_core::ToArcString) -> SafeIdentifier {
    let name__447 = name__447.to_arc_string();
    let return__203: SafeIdentifier;
    let mut t___2783: SafeIdentifier;
    'ok___5145: {
        'orelse___1075: {
            t___2783 = match safe_identifier(name__447.clone()) {
                Ok(x) => x,
                _ => break 'orelse___1075
            };
            return__203 = t___2783.clone();
            break 'ok___5145;
        }
        return__203 = temper_core::cast::<SafeIdentifier>(panic!()).unwrap();
    }
    return return__203.clone();
}
fn userTable__303() -> TableDef {
    return TableDef::new(csid__302("users"), [FieldDef::new(csid__302("name"), FieldType::new(StringField::new()), false), FieldDef::new(csid__302("email"), FieldType::new(StringField::new()), false), FieldDef::new(csid__302("age"), FieldType::new(IntField::new()), true), FieldDef::new(csid__302("score"), FieldType::new(FloatField::new()), true), FieldDef::new(csid__302("active"), FieldType::new(BoolField::new()), true)]);
}
pub fn delete_sql(tableDef__534: TableDef, id__535: i32) -> SqlFragment {
    let b__537: SqlBuilder = SqlBuilder::new();
    b__537.append_safe("DELETE FROM ");
    b__537.append_safe(tableDef__534.table_name().sql_value());
    b__537.append_safe(" WHERE id = ");
    b__537.append_int32(id__535);
    return b__537.accumulated();
}
pub fn from(tableName__586: SafeIdentifier) -> Query {
    return Query::new(tableName__586.clone(), [], [], [], None, None);
}
fn sid__304(name__588: impl temper_core::ToArcString) -> SafeIdentifier {
    let name__588 = name__588.to_arc_string();
    let return__223: SafeIdentifier;
    let mut t___2420: SafeIdentifier;
    'ok___5156: {
        'orelse___1083: {
            t___2420 = match safe_identifier(name__588.clone()) {
                Ok(x) => x,
                _ => break 'orelse___1083
            };
            return__223 = t___2420.clone();
            break 'ok___5156;
        }
        return__223 = temper_core::cast::<SafeIdentifier>(panic!()).unwrap();
    }
    return return__223.clone();
}
#[cfg(test)]
mod tests {
    #[test]
    fn castWhitelistsAllowedFields__908() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___22 = temper_std::testing::Test::new();
        let params__451: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("name".to_string()), std::sync::Arc::new("Alice".to_string())), (std::sync::Arc::new("email".to_string()), std::sync::Arc::new("alice@example.com".to_string())), (std::sync::Arc::new("admin".to_string()), std::sync::Arc::new("true".to_string()))]);
        let mut t___4828: TableDef = userTable__303();
        let mut t___4829: SafeIdentifier = csid__302("name");
        let mut t___4830: SafeIdentifier = csid__302("email");
        let cs__452: Changeset = changeset(t___4828.clone(), params__451.clone()).cast(std::sync::Arc::new(vec![t___4829.clone(), t___4830.clone()]));
        let mut t___4833: bool = temper_core::MappedTrait::has( & cs__452.changes(), std::sync::Arc::new("name".to_string()));
        #[derive(Clone)]
        struct ClosureGroup___15 {}
        impl ClosureGroup___15 {
            fn fn__4823(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("name should be in changes".to_string());
            }
        }
        let closure_group = ClosureGroup___15 {};
        let fn__4823 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__4823())
        };
        test___22.assert(t___4833, fn__4823.clone());
        let mut t___4837: bool = temper_core::MappedTrait::has( & cs__452.changes(), std::sync::Arc::new("email".to_string()));
        #[derive(Clone)]
        struct ClosureGroup___16 {}
        impl ClosureGroup___16 {
            fn fn__4822(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("email should be in changes".to_string());
            }
        }
        let closure_group = ClosureGroup___16 {};
        let fn__4822 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__4822())
        };
        test___22.assert(t___4837, fn__4822.clone());
        let mut t___4843: bool = ! temper_core::MappedTrait::has( & cs__452.changes(), std::sync::Arc::new("admin".to_string()));
        #[derive(Clone)]
        struct ClosureGroup___17 {}
        impl ClosureGroup___17 {
            fn fn__4821(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("admin must be dropped (not in whitelist)".to_string());
            }
        }
        let closure_group = ClosureGroup___17 {};
        let fn__4821 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__4821())
        };
        test___22.assert(t___4843, fn__4821.clone());
        let mut t___4845: bool = cs__452.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___18 {}
        impl ClosureGroup___18 {
            fn fn__4820(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("should still be valid".to_string());
            }
        }
        let closure_group = ClosureGroup___18 {};
        let fn__4820 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__4820())
        };
        test___22.assert(t___4845, fn__4820.clone());
        test___22.soft_fail_to_hard()
    }
    #[test]
    fn castIsReplacingNotAdditiveSecondCallResetsWhitelist__909() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___23 = temper_std::testing::Test::new();
        let params__454: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("name".to_string()), std::sync::Arc::new("Alice".to_string())), (std::sync::Arc::new("email".to_string()), std::sync::Arc::new("alice@example.com".to_string()))]);
        let mut t___4806: TableDef = userTable__303();
        let mut t___4807: SafeIdentifier = csid__302("name");
        let cs__455: Changeset = changeset(t___4806.clone(), params__454.clone()).cast(std::sync::Arc::new(vec![t___4807.clone()])).cast(std::sync::Arc::new(vec![csid__302("email")]));
        let mut t___4814: bool = ! temper_core::MappedTrait::has( & cs__455.changes(), std::sync::Arc::new("name".to_string()));
        #[derive(Clone)]
        struct ClosureGroup___19 {}
        impl ClosureGroup___19 {
            fn fn__4802(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("name must be excluded by second cast".to_string());
            }
        }
        let closure_group = ClosureGroup___19 {};
        let fn__4802 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__4802())
        };
        test___23.assert(t___4814, fn__4802.clone());
        let mut t___4817: bool = temper_core::MappedTrait::has( & cs__455.changes(), std::sync::Arc::new("email".to_string()));
        #[derive(Clone)]
        struct ClosureGroup___20 {}
        impl ClosureGroup___20 {
            fn fn__4801(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("email should be present".to_string());
            }
        }
        let closure_group = ClosureGroup___20 {};
        let fn__4801 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__4801())
        };
        test___23.assert(t___4817, fn__4801.clone());
        test___23.soft_fail_to_hard()
    }
    #[test]
    fn castIgnoresEmptyStringValues__910() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___24 = temper_std::testing::Test::new();
        let params__457: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("name".to_string()), std::sync::Arc::new("".to_string())), (std::sync::Arc::new("email".to_string()), std::sync::Arc::new("bob@example.com".to_string()))]);
        let mut t___4788: TableDef = userTable__303();
        let mut t___4789: SafeIdentifier = csid__302("name");
        let mut t___4790: SafeIdentifier = csid__302("email");
        let cs__458: Changeset = changeset(t___4788.clone(), params__457.clone()).cast(std::sync::Arc::new(vec![t___4789.clone(), t___4790.clone()]));
        let mut t___4795: bool = ! temper_core::MappedTrait::has( & cs__458.changes(), std::sync::Arc::new("name".to_string()));
        #[derive(Clone)]
        struct ClosureGroup___21 {}
        impl ClosureGroup___21 {
            fn fn__4784(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("empty name should not be in changes".to_string());
            }
        }
        let closure_group = ClosureGroup___21 {};
        let fn__4784 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__4784())
        };
        test___24.assert(t___4795, fn__4784.clone());
        let mut t___4798: bool = temper_core::MappedTrait::has( & cs__458.changes(), std::sync::Arc::new("email".to_string()));
        #[derive(Clone)]
        struct ClosureGroup___22 {}
        impl ClosureGroup___22 {
            fn fn__4783(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("email should be in changes".to_string());
            }
        }
        let closure_group = ClosureGroup___22 {};
        let fn__4783 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__4783())
        };
        test___24.assert(t___4798, fn__4783.clone());
        test___24.soft_fail_to_hard()
    }
    #[test]
    fn validateRequiredPassesWhenFieldPresent__911() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___25 = temper_std::testing::Test::new();
        let params__460: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("name".to_string()), std::sync::Arc::new("Alice".to_string()))]);
        let mut t___4770: TableDef = userTable__303();
        let mut t___4771: SafeIdentifier = csid__302("name");
        let cs__461: Changeset = changeset(t___4770.clone(), params__460.clone()).cast(std::sync::Arc::new(vec![t___4771.clone()])).validate_required(std::sync::Arc::new(vec![csid__302("name")]));
        let mut t___4775: bool = cs__461.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___23 {}
        impl ClosureGroup___23 {
            fn fn__4767(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("should be valid".to_string());
            }
        }
        let closure_group = ClosureGroup___23 {};
        let fn__4767 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__4767())
        };
        test___25.assert(t___4775, fn__4767.clone());
        let mut t___4781: bool = Some(temper_core::ListedTrait::len( & cs__461.errors())) == Some(0);
        #[derive(Clone)]
        struct ClosureGroup___24 {}
        impl ClosureGroup___24 {
            fn fn__4766(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("no errors expected".to_string());
            }
        }
        let closure_group = ClosureGroup___24 {};
        let fn__4766 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__4766())
        };
        test___25.assert(t___4781, fn__4766.clone());
        test___25.soft_fail_to_hard()
    }
    #[test]
    fn validateRequiredFailsWhenFieldMissing__912() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___26 = temper_std::testing::Test::new();
        let params__463: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & []);
        let mut t___4746: TableDef = userTable__303();
        let mut t___4747: SafeIdentifier = csid__302("name");
        let cs__464: Changeset = changeset(t___4746.clone(), params__463.clone()).cast(std::sync::Arc::new(vec![t___4747.clone()])).validate_required(std::sync::Arc::new(vec![csid__302("name")]));
        let mut t___4753: bool = ! cs__464.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___25 {}
        impl ClosureGroup___25 {
            fn fn__4744(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("should be invalid".to_string());
            }
        }
        let closure_group = ClosureGroup___25 {};
        let fn__4744 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__4744())
        };
        test___26.assert(t___4753, fn__4744.clone());
        let mut t___4758: bool = Some(temper_core::ListedTrait::len( & cs__464.errors())) == Some(1);
        #[derive(Clone)]
        struct ClosureGroup___26 {}
        impl ClosureGroup___26 {
            fn fn__4743(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("should have one error".to_string());
            }
        }
        let closure_group = ClosureGroup___26 {};
        let fn__4743 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__4743())
        };
        test___26.assert(t___4758, fn__4743.clone());
        let mut t___4764: bool = Some(temper_core::ListedTrait::get( & cs__464.errors(), 0).field().as_str()) == Some("name");
        #[derive(Clone)]
        struct ClosureGroup___27 {}
        impl ClosureGroup___27 {
            fn fn__4742(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("error should name the field".to_string());
            }
        }
        let closure_group = ClosureGroup___27 {};
        let fn__4742 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__4742())
        };
        test___26.assert(t___4764, fn__4742.clone());
        test___26.soft_fail_to_hard()
    }
    #[test]
    fn validateLengthPassesWithinRange__913() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___27 = temper_std::testing::Test::new();
        let params__466: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("name".to_string()), std::sync::Arc::new("Alice".to_string()))]);
        let mut t___4734: TableDef = userTable__303();
        let mut t___4735: SafeIdentifier = csid__302("name");
        let cs__467: Changeset = changeset(t___4734.clone(), params__466.clone()).cast(std::sync::Arc::new(vec![t___4735.clone()])).validate_length(csid__302("name"), 2, 50);
        let mut t___4739: bool = cs__467.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___28 {}
        impl ClosureGroup___28 {
            fn fn__4731(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("should be valid".to_string());
            }
        }
        let closure_group = ClosureGroup___28 {};
        let fn__4731 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__4731())
        };
        test___27.assert(t___4739, fn__4731.clone());
        test___27.soft_fail_to_hard()
    }
    #[test]
    fn validateLengthFailsWhenTooShort__914() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___28 = temper_std::testing::Test::new();
        let params__469: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("name".to_string()), std::sync::Arc::new("A".to_string()))]);
        let mut t___4722: TableDef = userTable__303();
        let mut t___4723: SafeIdentifier = csid__302("name");
        let cs__470: Changeset = changeset(t___4722.clone(), params__469.clone()).cast(std::sync::Arc::new(vec![t___4723.clone()])).validate_length(csid__302("name"), 2, 50);
        let mut t___4729: bool = ! cs__470.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___29 {}
        impl ClosureGroup___29 {
            fn fn__4719(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("should be invalid".to_string());
            }
        }
        let closure_group = ClosureGroup___29 {};
        let fn__4719 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__4719())
        };
        test___28.assert(t___4729, fn__4719.clone());
        test___28.soft_fail_to_hard()
    }
    #[test]
    fn validateLengthFailsWhenTooLong__915() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___29 = temper_std::testing::Test::new();
        let params__472: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("name".to_string()), std::sync::Arc::new("ABCDEFGHIJKLMNOPQRSTUVWXYZ".to_string()))]);
        let mut t___4710: TableDef = userTable__303();
        let mut t___4711: SafeIdentifier = csid__302("name");
        let cs__473: Changeset = changeset(t___4710.clone(), params__472.clone()).cast(std::sync::Arc::new(vec![t___4711.clone()])).validate_length(csid__302("name"), 2, 10);
        let mut t___4717: bool = ! cs__473.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___30 {}
        impl ClosureGroup___30 {
            fn fn__4707(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("should be invalid".to_string());
            }
        }
        let closure_group = ClosureGroup___30 {};
        let fn__4707 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__4707())
        };
        test___29.assert(t___4717, fn__4707.clone());
        test___29.soft_fail_to_hard()
    }
    #[test]
    fn validateIntPassesForValidInteger__916() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___30 = temper_std::testing::Test::new();
        let params__475: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("age".to_string()), std::sync::Arc::new("30".to_string()))]);
        let mut t___4699: TableDef = userTable__303();
        let mut t___4700: SafeIdentifier = csid__302("age");
        let cs__476: Changeset = changeset(t___4699.clone(), params__475.clone()).cast(std::sync::Arc::new(vec![t___4700.clone()])).validate_int(csid__302("age"));
        let mut t___4704: bool = cs__476.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___31 {}
        impl ClosureGroup___31 {
            fn fn__4696(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("should be valid".to_string());
            }
        }
        let closure_group = ClosureGroup___31 {};
        let fn__4696 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__4696())
        };
        test___30.assert(t___4704, fn__4696.clone());
        test___30.soft_fail_to_hard()
    }
    #[test]
    fn validateIntFailsForNonInteger__917() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___31 = temper_std::testing::Test::new();
        let params__478: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("age".to_string()), std::sync::Arc::new("not-a-number".to_string()))]);
        let mut t___4687: TableDef = userTable__303();
        let mut t___4688: SafeIdentifier = csid__302("age");
        let cs__479: Changeset = changeset(t___4687.clone(), params__478.clone()).cast(std::sync::Arc::new(vec![t___4688.clone()])).validate_int(csid__302("age"));
        let mut t___4694: bool = ! cs__479.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___32 {}
        impl ClosureGroup___32 {
            fn fn__4684(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("should be invalid".to_string());
            }
        }
        let closure_group = ClosureGroup___32 {};
        let fn__4684 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__4684())
        };
        test___31.assert(t___4694, fn__4684.clone());
        test___31.soft_fail_to_hard()
    }
    #[test]
    fn validateFloatPassesForValidFloat__918() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___32 = temper_std::testing::Test::new();
        let params__481: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("score".to_string()), std::sync::Arc::new("9.5".to_string()))]);
        let mut t___4676: TableDef = userTable__303();
        let mut t___4677: SafeIdentifier = csid__302("score");
        let cs__482: Changeset = changeset(t___4676.clone(), params__481.clone()).cast(std::sync::Arc::new(vec![t___4677.clone()])).validate_float(csid__302("score"));
        let mut t___4681: bool = cs__482.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___33 {}
        impl ClosureGroup___33 {
            fn fn__4673(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("should be valid".to_string());
            }
        }
        let closure_group = ClosureGroup___33 {};
        let fn__4673 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__4673())
        };
        test___32.assert(t___4681, fn__4673.clone());
        test___32.soft_fail_to_hard()
    }
    #[test]
    fn validateInt64_passesForValid64_bitInteger__919() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___33 = temper_std::testing::Test::new();
        let params__484: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("age".to_string()), std::sync::Arc::new("9999999999".to_string()))]);
        let mut t___4665: TableDef = userTable__303();
        let mut t___4666: SafeIdentifier = csid__302("age");
        let cs__485: Changeset = changeset(t___4665.clone(), params__484.clone()).cast(std::sync::Arc::new(vec![t___4666.clone()])).validate_int64(csid__302("age"));
        let mut t___4670: bool = cs__485.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___34 {}
        impl ClosureGroup___34 {
            fn fn__4662(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("should be valid".to_string());
            }
        }
        let closure_group = ClosureGroup___34 {};
        let fn__4662 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__4662())
        };
        test___33.assert(t___4670, fn__4662.clone());
        test___33.soft_fail_to_hard()
    }
    #[test]
    fn validateInt64_failsForNonInteger__920() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___34 = temper_std::testing::Test::new();
        let params__487: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("age".to_string()), std::sync::Arc::new("not-a-number".to_string()))]);
        let mut t___4653: TableDef = userTable__303();
        let mut t___4654: SafeIdentifier = csid__302("age");
        let cs__488: Changeset = changeset(t___4653.clone(), params__487.clone()).cast(std::sync::Arc::new(vec![t___4654.clone()])).validate_int64(csid__302("age"));
        let mut t___4660: bool = ! cs__488.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___35 {}
        impl ClosureGroup___35 {
            fn fn__4650(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("should be invalid".to_string());
            }
        }
        let closure_group = ClosureGroup___35 {};
        let fn__4650 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__4650())
        };
        test___34.assert(t___4660, fn__4650.clone());
        test___34.soft_fail_to_hard()
    }
    #[test]
    fn validateBoolAcceptsTrue1_yesOn__921() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___35 = temper_std::testing::Test::new();
        #[derive(Clone)]
        struct ClosureGroup___36 {
            test___35: temper_std::testing::Test
        }
        impl ClosureGroup___36 {
            fn fn__4647(& self, v__490: impl temper_core::ToArcString) {
                let v__490 = v__490.to_arc_string();
                let params__491: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("active".to_string()), v__490.clone())]);
                let mut t___4639: TableDef = userTable__303();
                let mut t___4640: SafeIdentifier = csid__302("active");
                let cs__492: Changeset = changeset(t___4639.clone(), params__491.clone()).cast(std::sync::Arc::new(vec![t___4640.clone()])).validate_bool(csid__302("active"));
                let mut t___4644: bool = cs__492.is_valid();
                #[derive(Clone)]
                struct ClosureGroup___37 {
                    v__490: std::sync::Arc<String>
                }
                impl ClosureGroup___37 {
                    fn fn__4636(& self) -> std::sync::Arc<String> {
                        return std::sync::Arc::new(format!("should accept: {}", self.v__490.clone()));
                    }
                }
                let closure_group = ClosureGroup___37 {
                    v__490: v__490.clone()
                };
                let fn__4636 = {
                    let closure_group = closure_group.clone();
                    std::sync::Arc::new(move | | closure_group.fn__4636())
                };
                self.test___35.assert(t___4644, fn__4636.clone());
            }
        }
        let closure_group = ClosureGroup___36 {
            test___35: test___35.clone()
        };
        let fn__4647 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | v__490: std::sync::Arc<String> | closure_group.fn__4647(v__490))
        };
        temper_core::listed::list_for_each( & std::sync::Arc::new(vec![std::sync::Arc::new("true".to_string()), std::sync::Arc::new("1".to_string()), std::sync::Arc::new("yes".to_string()), std::sync::Arc::new("on".to_string())]), & ( * fn__4647.clone()));
        test___35.soft_fail_to_hard()
    }
    #[test]
    fn validateBoolAcceptsFalse0_noOff__922() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___36 = temper_std::testing::Test::new();
        #[derive(Clone)]
        struct ClosureGroup___38 {
            test___36: temper_std::testing::Test
        }
        impl ClosureGroup___38 {
            fn fn__4633(& self, v__494: impl temper_core::ToArcString) {
                let v__494 = v__494.to_arc_string();
                let params__495: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("active".to_string()), v__494.clone())]);
                let mut t___4625: TableDef = userTable__303();
                let mut t___4626: SafeIdentifier = csid__302("active");
                let cs__496: Changeset = changeset(t___4625.clone(), params__495.clone()).cast(std::sync::Arc::new(vec![t___4626.clone()])).validate_bool(csid__302("active"));
                let mut t___4630: bool = cs__496.is_valid();
                #[derive(Clone)]
                struct ClosureGroup___39 {
                    v__494: std::sync::Arc<String>
                }
                impl ClosureGroup___39 {
                    fn fn__4622(& self) -> std::sync::Arc<String> {
                        return std::sync::Arc::new(format!("should accept: {}", self.v__494.clone()));
                    }
                }
                let closure_group = ClosureGroup___39 {
                    v__494: v__494.clone()
                };
                let fn__4622 = {
                    let closure_group = closure_group.clone();
                    std::sync::Arc::new(move | | closure_group.fn__4622())
                };
                self.test___36.assert(t___4630, fn__4622.clone());
            }
        }
        let closure_group = ClosureGroup___38 {
            test___36: test___36.clone()
        };
        let fn__4633 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | v__494: std::sync::Arc<String> | closure_group.fn__4633(v__494))
        };
        temper_core::listed::list_for_each( & std::sync::Arc::new(vec![std::sync::Arc::new("false".to_string()), std::sync::Arc::new("0".to_string()), std::sync::Arc::new("no".to_string()), std::sync::Arc::new("off".to_string())]), & ( * fn__4633.clone()));
        test___36.soft_fail_to_hard()
    }
    #[test]
    fn validateBoolRejectsAmbiguousValues__923() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___37 = temper_std::testing::Test::new();
        #[derive(Clone)]
        struct ClosureGroup___40 {
            test___37: temper_std::testing::Test
        }
        impl ClosureGroup___40 {
            fn fn__4619(& self, v__498: impl temper_core::ToArcString) {
                let v__498 = v__498.to_arc_string();
                let params__499: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("active".to_string()), v__498.clone())]);
                let mut t___4610: TableDef = userTable__303();
                let mut t___4611: SafeIdentifier = csid__302("active");
                let cs__500: Changeset = changeset(t___4610.clone(), params__499.clone()).cast(std::sync::Arc::new(vec![t___4611.clone()])).validate_bool(csid__302("active"));
                let mut t___4617: bool = ! cs__500.is_valid();
                #[derive(Clone)]
                struct ClosureGroup___41 {
                    v__498: std::sync::Arc<String>
                }
                impl ClosureGroup___41 {
                    fn fn__4607(& self) -> std::sync::Arc<String> {
                        return std::sync::Arc::new(format!("should reject ambiguous: {}", self.v__498.clone()));
                    }
                }
                let closure_group = ClosureGroup___41 {
                    v__498: v__498.clone()
                };
                let fn__4607 = {
                    let closure_group = closure_group.clone();
                    std::sync::Arc::new(move | | closure_group.fn__4607())
                };
                self.test___37.assert(t___4617, fn__4607.clone());
            }
        }
        let closure_group = ClosureGroup___40 {
            test___37: test___37.clone()
        };
        let fn__4619 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | v__498: std::sync::Arc<String> | closure_group.fn__4619(v__498))
        };
        temper_core::listed::list_for_each( & std::sync::Arc::new(vec![std::sync::Arc::new("TRUE".to_string()), std::sync::Arc::new("Yes".to_string()), std::sync::Arc::new("maybe".to_string()), std::sync::Arc::new("2".to_string()), std::sync::Arc::new("enabled".to_string())]), & ( * fn__4619.clone()));
        test___37.soft_fail_to_hard()
    }
    #[test]
    fn toInsertSqlEscapesBobbyTables__924() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___38 = temper_std::testing::Test::new();
        let mut t___2584: SqlFragment;
        let params__502: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("name".to_string()), std::sync::Arc::new("Robert'); DROP TABLE users;--".to_string())), (std::sync::Arc::new("email".to_string()), std::sync::Arc::new("bobby@evil.com".to_string()))]);
        let mut t___4595: TableDef = userTable__303();
        let mut t___4596: SafeIdentifier = csid__302("name");
        let mut t___4597: SafeIdentifier = csid__302("email");
        let cs__503: Changeset = changeset(t___4595.clone(), params__502.clone()).cast(std::sync::Arc::new(vec![t___4596.clone(), t___4597.clone()])).validate_required(std::sync::Arc::new(vec![csid__302("name"), csid__302("email")]));
        let sqlFrag__504: SqlFragment;
        'ok___5147: {
            'orelse___1076: {
                t___2584 = match cs__503.to_insert_sql() {
                    Ok(x) => x,
                    _ => break 'orelse___1076
                };
                sqlFrag__504 = t___2584.clone();
                break 'ok___5147;
            }
            sqlFrag__504 = panic!();
        }
        let s__505: std::sync::Arc<String> = sqlFrag__504.to_string();
        let mut t___4604: bool = temper_core::string::index_of( & s__505, "''", None).is_some();
        #[derive(Clone)]
        struct ClosureGroup___42 {
            s__505: std::sync::Arc<String>
        }
        impl ClosureGroup___42 {
            fn fn__4591(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("single quote must be doubled: {}", self.s__505.clone()));
            }
        }
        let closure_group = ClosureGroup___42 {
            s__505: s__505.clone()
        };
        let fn__4591 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__4591())
        };
        test___38.assert(t___4604, fn__4591.clone());
        test___38.soft_fail_to_hard()
    }
    #[test]
    fn toInsertSqlProducesCorrectSqlForStringField__925() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___39 = temper_std::testing::Test::new();
        let mut t___2563: SqlFragment;
        let params__507: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("name".to_string()), std::sync::Arc::new("Alice".to_string())), (std::sync::Arc::new("email".to_string()), std::sync::Arc::new("a@example.com".to_string()))]);
        let mut t___4575: TableDef = userTable__303();
        let mut t___4576: SafeIdentifier = csid__302("name");
        let mut t___4577: SafeIdentifier = csid__302("email");
        let cs__508: Changeset = changeset(t___4575.clone(), params__507.clone()).cast(std::sync::Arc::new(vec![t___4576.clone(), t___4577.clone()])).validate_required(std::sync::Arc::new(vec![csid__302("name"), csid__302("email")]));
        let sqlFrag__509: SqlFragment;
        'ok___5150: {
            'orelse___1077: {
                t___2563 = match cs__508.to_insert_sql() {
                    Ok(x) => x,
                    _ => break 'orelse___1077
                };
                sqlFrag__509 = t___2563.clone();
                break 'ok___5150;
            }
            sqlFrag__509 = panic!();
        }
        let s__510: std::sync::Arc<String> = sqlFrag__509.to_string();
        let mut t___4584: bool = temper_core::string::index_of( & s__510, "INSERT INTO users", None).is_some();
        #[derive(Clone)]
        struct ClosureGroup___43 {
            s__510: std::sync::Arc<String>
        }
        impl ClosureGroup___43 {
            fn fn__4571(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("has INSERT INTO: {}", self.s__510.clone()));
            }
        }
        let closure_group = ClosureGroup___43 {
            s__510: s__510.clone()
        };
        let fn__4571 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__4571())
        };
        test___39.assert(t___4584, fn__4571.clone());
        let mut t___4588: bool = temper_core::string::index_of( & s__510, "'Alice'", None).is_some();
        #[derive(Clone)]
        struct ClosureGroup___44 {
            s__510: std::sync::Arc<String>
        }
        impl ClosureGroup___44 {
            fn fn__4570(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("has quoted name: {}", self.s__510.clone()));
            }
        }
        let closure_group = ClosureGroup___44 {
            s__510: s__510.clone()
        };
        let fn__4570 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__4570())
        };
        test___39.assert(t___4588, fn__4570.clone());
        test___39.soft_fail_to_hard()
    }
    #[test]
    fn toInsertSqlProducesCorrectSqlForIntField__926() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___40 = temper_std::testing::Test::new();
        let mut t___2546: SqlFragment;
        let params__512: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("name".to_string()), std::sync::Arc::new("Bob".to_string())), (std::sync::Arc::new("email".to_string()), std::sync::Arc::new("b@example.com".to_string())), (std::sync::Arc::new("age".to_string()), std::sync::Arc::new("25".to_string()))]);
        let mut t___4557: TableDef = userTable__303();
        let mut t___4558: SafeIdentifier = csid__302("name");
        let mut t___4559: SafeIdentifier = csid__302("email");
        let mut t___4560: SafeIdentifier = csid__302("age");
        let cs__513: Changeset = changeset(t___4557.clone(), params__512.clone()).cast(std::sync::Arc::new(vec![t___4558.clone(), t___4559.clone(), t___4560.clone()])).validate_required(std::sync::Arc::new(vec![csid__302("name"), csid__302("email")]));
        let sqlFrag__514: SqlFragment;
        'ok___5151: {
            'orelse___1078: {
                t___2546 = match cs__513.to_insert_sql() {
                    Ok(x) => x,
                    _ => break 'orelse___1078
                };
                sqlFrag__514 = t___2546.clone();
                break 'ok___5151;
            }
            sqlFrag__514 = panic!();
        }
        let s__515: std::sync::Arc<String> = sqlFrag__514.to_string();
        let mut t___4567: bool = temper_core::string::index_of( & s__515, "25", None).is_some();
        #[derive(Clone)]
        struct ClosureGroup___45 {
            s__515: std::sync::Arc<String>
        }
        impl ClosureGroup___45 {
            fn fn__4552(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("age rendered unquoted: {}", self.s__515.clone()));
            }
        }
        let closure_group = ClosureGroup___45 {
            s__515: s__515.clone()
        };
        let fn__4552 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__4552())
        };
        test___40.assert(t___4567, fn__4552.clone());
        test___40.soft_fail_to_hard()
    }
    #[test]
    fn toInsertSqlBubblesOnInvalidChangeset__927() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___41 = temper_std::testing::Test::new();
        let params__517: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & []);
        let mut t___4545: TableDef = userTable__303();
        let mut t___4546: SafeIdentifier = csid__302("name");
        let cs__518: Changeset = changeset(t___4545.clone(), params__517.clone()).cast(std::sync::Arc::new(vec![t___4546.clone()])).validate_required(std::sync::Arc::new(vec![csid__302("name")]));
        let didBubble__519: bool;
        'ok___5152: {
            'orelse___1079: {
                match cs__518.to_insert_sql() {
                    Ok(x) => x,
                    _ => break 'orelse___1079
                };
                didBubble__519 = false;
                break 'ok___5152;
            }
            didBubble__519 = true;
        }
        #[derive(Clone)]
        struct ClosureGroup___46 {}
        impl ClosureGroup___46 {
            fn fn__4543(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("invalid changeset should bubble".to_string());
            }
        }
        let closure_group = ClosureGroup___46 {};
        let fn__4543 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__4543())
        };
        test___41.assert(didBubble__519, fn__4543.clone());
        test___41.soft_fail_to_hard()
    }
    #[test]
    fn toInsertSqlEnforcesNonNullableFieldsIndependentlyOfIsValid__928() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___42 = temper_std::testing::Test::new();
        let strictTable__521: TableDef = TableDef::new(csid__302("posts"), [FieldDef::new(csid__302("title"), FieldType::new(StringField::new()), false), FieldDef::new(csid__302("body"), FieldType::new(StringField::new()), true)]);
        let params__522: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("body".to_string()), std::sync::Arc::new("hello".to_string()))]);
        let mut t___4536: SafeIdentifier = csid__302("body");
        let cs__523: Changeset = changeset(strictTable__521.clone(), params__522.clone()).cast(std::sync::Arc::new(vec![t___4536.clone()]));
        let mut t___4538: bool = cs__523.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___47 {}
        impl ClosureGroup___47 {
            fn fn__4525(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("changeset should appear valid (no explicit validation run)".to_string());
            }
        }
        let closure_group = ClosureGroup___47 {};
        let fn__4525 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__4525())
        };
        test___42.assert(t___4538, fn__4525.clone());
        let didBubble__524: bool;
        'ok___5153: {
            'orelse___1080: {
                match cs__523.to_insert_sql() {
                    Ok(x) => x,
                    _ => break 'orelse___1080
                };
                didBubble__524 = false;
                break 'ok___5153;
            }
            didBubble__524 = true;
        }
        #[derive(Clone)]
        struct ClosureGroup___48 {}
        impl ClosureGroup___48 {
            fn fn__4524(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("toInsertSql should enforce nullable regardless of isValid".to_string());
            }
        }
        let closure_group = ClosureGroup___48 {};
        let fn__4524 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__4524())
        };
        test___42.assert(didBubble__524, fn__4524.clone());
        test___42.soft_fail_to_hard()
    }
    #[test]
    fn toUpdateSqlProducesCorrectSql__929() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___43 = temper_std::testing::Test::new();
        let mut t___2506: SqlFragment;
        let params__526: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("name".to_string()), std::sync::Arc::new("Bob".to_string()))]);
        let mut t___4515: TableDef = userTable__303();
        let mut t___4516: SafeIdentifier = csid__302("name");
        let cs__527: Changeset = changeset(t___4515.clone(), params__526.clone()).cast(std::sync::Arc::new(vec![t___4516.clone()])).validate_required(std::sync::Arc::new(vec![csid__302("name")]));
        let sqlFrag__528: SqlFragment;
        'ok___5154: {
            'orelse___1081: {
                t___2506 = match cs__527.to_update_sql(42) {
                    Ok(x) => x,
                    _ => break 'orelse___1081
                };
                sqlFrag__528 = t___2506.clone();
                break 'ok___5154;
            }
            sqlFrag__528 = panic!();
        }
        let s__529: std::sync::Arc<String> = sqlFrag__528.to_string();
        let mut t___4522: bool = Some(s__529.as_str()) == Some("UPDATE users SET name = 'Bob' WHERE id = 42");
        #[derive(Clone)]
        struct ClosureGroup___49 {
            s__529: std::sync::Arc<String>
        }
        impl ClosureGroup___49 {
            fn fn__4512(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("got: {}", self.s__529.clone()));
            }
        }
        let closure_group = ClosureGroup___49 {
            s__529: s__529.clone()
        };
        let fn__4512 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__4512())
        };
        test___43.assert(t___4522, fn__4512.clone());
        test___43.soft_fail_to_hard()
    }
    #[test]
    fn toUpdateSqlBubblesOnInvalidChangeset__930() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___44 = temper_std::testing::Test::new();
        let params__531: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & []);
        let mut t___4505: TableDef = userTable__303();
        let mut t___4506: SafeIdentifier = csid__302("name");
        let cs__532: Changeset = changeset(t___4505.clone(), params__531.clone()).cast(std::sync::Arc::new(vec![t___4506.clone()])).validate_required(std::sync::Arc::new(vec![csid__302("name")]));
        let didBubble__533: bool;
        'ok___5155: {
            'orelse___1082: {
                match cs__532.to_update_sql(1) {
                    Ok(x) => x,
                    _ => break 'orelse___1082
                };
                didBubble__533 = false;
                break 'ok___5155;
            }
            didBubble__533 = true;
        }
        #[derive(Clone)]
        struct ClosureGroup___50 {}
        impl ClosureGroup___50 {
            fn fn__4503(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("invalid changeset should bubble".to_string());
            }
        }
        let closure_group = ClosureGroup___50 {};
        let fn__4503 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__4503())
        };
        test___44.assert(didBubble__533, fn__4503.clone());
        test___44.soft_fail_to_hard()
    }
    #[test]
    fn bareFromProducesSelect__955() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___45 = temper_std::testing::Test::new();
        let q__591: Query = from(sid__304("users"));
        let mut t___4438: bool = Some(q__591.to_sql().to_string().as_str()) == Some("SELECT * FROM users");
        #[derive(Clone)]
        struct ClosureGroup___51 {}
        impl ClosureGroup___51 {
            fn fn__4433(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("bare query".to_string());
            }
        }
        let closure_group = ClosureGroup___51 {};
        let fn__4433 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__4433())
        };
        test___45.assert(t___4438, fn__4433.clone());
        test___45.soft_fail_to_hard()
    }
    #[test]
    fn selectRestrictsColumns__956() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___46 = temper_std::testing::Test::new();
        let mut t___4424: SafeIdentifier = sid__304("users");
        let mut t___4425: SafeIdentifier = sid__304("id");
        let mut t___4426: SafeIdentifier = sid__304("name");
        let q__593: Query = from(t___4424.clone()).select([t___4425.clone(), t___4426.clone()]);
        let mut t___4431: bool = Some(q__593.to_sql().to_string().as_str()) == Some("SELECT id, name FROM users");
        #[derive(Clone)]
        struct ClosureGroup___52 {}
        impl ClosureGroup___52 {
            fn fn__4423(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("select columns".to_string());
            }
        }
        let closure_group = ClosureGroup___52 {};
        let fn__4423 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__4423())
        };
        test___46.assert(t___4431, fn__4423.clone());
        test___46.soft_fail_to_hard()
    }
    #[test]
    fn whereAddsConditionWithIntValue__957() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___47 = temper_std::testing::Test::new();
        let mut t___4412: SafeIdentifier = sid__304("users");
        let mut t___4413: SqlBuilder = SqlBuilder::new();
        t___4413.append_safe("age > ");
        t___4413.append_int32(18);
        let mut t___4416: SqlFragment = t___4413.accumulated();
        let q__595: Query = from(t___4412.clone()).r#where(t___4416.clone());
        let mut t___4421: bool = Some(q__595.to_sql().to_string().as_str()) == Some("SELECT * FROM users WHERE age > 18");
        #[derive(Clone)]
        struct ClosureGroup___53 {}
        impl ClosureGroup___53 {
            fn fn__4411(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("where int".to_string());
            }
        }
        let closure_group = ClosureGroup___53 {};
        let fn__4411 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__4411())
        };
        test___47.assert(t___4421, fn__4411.clone());
        test___47.soft_fail_to_hard()
    }
    #[test]
    fn whereAddsConditionWithBoolValue__959() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___48 = temper_std::testing::Test::new();
        let mut t___4400: SafeIdentifier = sid__304("users");
        let mut t___4401: SqlBuilder = SqlBuilder::new();
        t___4401.append_safe("active = ");
        t___4401.append_boolean(true);
        let mut t___4404: SqlFragment = t___4401.accumulated();
        let q__597: Query = from(t___4400.clone()).r#where(t___4404.clone());
        let mut t___4409: bool = Some(q__597.to_sql().to_string().as_str()) == Some("SELECT * FROM users WHERE active = TRUE");
        #[derive(Clone)]
        struct ClosureGroup___54 {}
        impl ClosureGroup___54 {
            fn fn__4399(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("where bool".to_string());
            }
        }
        let closure_group = ClosureGroup___54 {};
        let fn__4399 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__4399())
        };
        test___48.assert(t___4409, fn__4399.clone());
        test___48.soft_fail_to_hard()
    }
    #[test]
    fn chainedWhereUsesAnd__961() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___49 = temper_std::testing::Test::new();
        let mut t___4383: SafeIdentifier = sid__304("users");
        let mut t___4384: SqlBuilder = SqlBuilder::new();
        t___4384.append_safe("age > ");
        t___4384.append_int32(18);
        let mut t___4387: SqlFragment = t___4384.accumulated();
        let mut t___4388: Query = from(t___4383.clone()).r#where(t___4387.clone());
        let mut t___4389: SqlBuilder = SqlBuilder::new();
        t___4389.append_safe("active = ");
        t___4389.append_boolean(true);
        let q__599: Query = t___4388.r#where(t___4389.accumulated());
        let mut t___4397: bool = Some(q__599.to_sql().to_string().as_str()) == Some("SELECT * FROM users WHERE age > 18 AND active = TRUE");
        #[derive(Clone)]
        struct ClosureGroup___55 {}
        impl ClosureGroup___55 {
            fn fn__4382(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("chained where".to_string());
            }
        }
        let closure_group = ClosureGroup___55 {};
        let fn__4382 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__4382())
        };
        test___49.assert(t___4397, fn__4382.clone());
        test___49.soft_fail_to_hard()
    }
    #[test]
    fn orderByAsc__964() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___50 = temper_std::testing::Test::new();
        let mut t___4374: SafeIdentifier = sid__304("users");
        let mut t___4375: SafeIdentifier = sid__304("name");
        let q__601: Query = from(t___4374.clone()).order_by(t___4375.clone(), true);
        let mut t___4380: bool = Some(q__601.to_sql().to_string().as_str()) == Some("SELECT * FROM users ORDER BY name ASC");
        #[derive(Clone)]
        struct ClosureGroup___56 {}
        impl ClosureGroup___56 {
            fn fn__4373(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("order asc".to_string());
            }
        }
        let closure_group = ClosureGroup___56 {};
        let fn__4373 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__4373())
        };
        test___50.assert(t___4380, fn__4373.clone());
        test___50.soft_fail_to_hard()
    }
    #[test]
    fn orderByDesc__965() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___51 = temper_std::testing::Test::new();
        let mut t___4365: SafeIdentifier = sid__304("users");
        let mut t___4366: SafeIdentifier = sid__304("created_at");
        let q__603: Query = from(t___4365.clone()).order_by(t___4366.clone(), false);
        let mut t___4371: bool = Some(q__603.to_sql().to_string().as_str()) == Some("SELECT * FROM users ORDER BY created_at DESC");
        #[derive(Clone)]
        struct ClosureGroup___57 {}
        impl ClosureGroup___57 {
            fn fn__4364(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("order desc".to_string());
            }
        }
        let closure_group = ClosureGroup___57 {};
        let fn__4364 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__4364())
        };
        test___51.assert(t___4371, fn__4364.clone());
        test___51.soft_fail_to_hard()
    }
    #[test]
    fn limitAndOffset__966() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___52 = temper_std::testing::Test::new();
        let mut t___2354: Query;
        let mut t___2355: Query;
        let q__605: Query;
        'ok___5157: {
            'orelse___1084: {
                t___2354 = match from(sid__304("users")).limit(10) {
                    Ok(x) => x,
                    _ => break 'orelse___1084
                };
                t___2355 = match t___2354.offset(20) {
                    Ok(x) => x,
                    _ => break 'orelse___1084
                };
                q__605 = t___2355.clone();
                break 'ok___5157;
            }
            q__605 = panic!();
        }
        let mut t___4362: bool = Some(q__605.to_sql().to_string().as_str()) == Some("SELECT * FROM users LIMIT 10 OFFSET 20");
        #[derive(Clone)]
        struct ClosureGroup___58 {}
        impl ClosureGroup___58 {
            fn fn__4357(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("limit/offset".to_string());
            }
        }
        let closure_group = ClosureGroup___58 {};
        let fn__4357 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__4357())
        };
        test___52.assert(t___4362, fn__4357.clone());
        test___52.soft_fail_to_hard()
    }
    #[test]
    fn limitBubblesOnNegative__967() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___53 = temper_std::testing::Test::new();
        let didBubble__607: bool;
        'ok___5158: {
            'orelse___1085: {
                match from(sid__304("users")).limit(-1) {
                    Ok(x) => x,
                    _ => break 'orelse___1085
                };
                didBubble__607 = false;
                break 'ok___5158;
            }
            didBubble__607 = true;
        }
        #[derive(Clone)]
        struct ClosureGroup___59 {}
        impl ClosureGroup___59 {
            fn fn__4353(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("negative limit should bubble".to_string());
            }
        }
        let closure_group = ClosureGroup___59 {};
        let fn__4353 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__4353())
        };
        test___53.assert(didBubble__607, fn__4353.clone());
        test___53.soft_fail_to_hard()
    }
    #[test]
    fn offsetBubblesOnNegative__968() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___54 = temper_std::testing::Test::new();
        let didBubble__609: bool;
        'ok___5159: {
            'orelse___1086: {
                match from(sid__304("users")).offset(-1) {
                    Ok(x) => x,
                    _ => break 'orelse___1086
                };
                didBubble__609 = false;
                break 'ok___5159;
            }
            didBubble__609 = true;
        }
        #[derive(Clone)]
        struct ClosureGroup___60 {}
        impl ClosureGroup___60 {
            fn fn__4349(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("negative offset should bubble".to_string());
            }
        }
        let closure_group = ClosureGroup___60 {};
        let fn__4349 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__4349())
        };
        test___54.assert(didBubble__609, fn__4349.clone());
        test___54.soft_fail_to_hard()
    }
    #[test]
    fn complexComposedQuery__969() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___55 = temper_std::testing::Test::new();
        let mut t___4327: SafeIdentifier;
        let mut t___4328: SafeIdentifier;
        let mut t___4329: SafeIdentifier;
        let mut t___4330: SafeIdentifier;
        let mut t___4331: Query;
        let mut t___4332: SqlBuilder;
        let mut t___4336: Query;
        let mut t___4337: SqlBuilder;
        let mut t___2340: Query;
        let mut t___2341: Query;
        let minAge__611: i32 = 21;
        let q__612: Query;
        'ok___5160: {
            'orelse___1087: {
                t___4327 = sid__304("users");
                t___4328 = sid__304("id");
                t___4329 = sid__304("name");
                t___4330 = sid__304("email");
                t___4331 = from(t___4327.clone()).select([t___4328.clone(), t___4329.clone(), t___4330.clone()]);
                t___4332 = SqlBuilder::new();
                t___4332.append_safe("age >= ");
                t___4332.append_int32(21);
                t___4336 = t___4331.r#where(t___4332.accumulated());
                t___4337 = SqlBuilder::new();
                t___4337.append_safe("active = ");
                t___4337.append_boolean(true);
                t___2340 = match t___4336.r#where(t___4337.accumulated()).order_by(sid__304("name"), true).limit(25) {
                    Ok(x) => x,
                    _ => break 'orelse___1087
                };
                t___2341 = match t___2340.offset(0) {
                    Ok(x) => x,
                    _ => break 'orelse___1087
                };
                q__612 = t___2341.clone();
                break 'ok___5160;
            }
            q__612 = panic!();
        }
        let mut t___4347: bool = Some(q__612.to_sql().to_string().as_str()) == Some("SELECT id, name, email FROM users WHERE age >= 21 AND active = TRUE ORDER BY name ASC LIMIT 25 OFFSET 0");
        #[derive(Clone)]
        struct ClosureGroup___61 {}
        impl ClosureGroup___61 {
            fn fn__4326(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("complex query".to_string());
            }
        }
        let closure_group = ClosureGroup___61 {};
        let fn__4326 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__4326())
        };
        test___55.assert(t___4347, fn__4326.clone());
        test___55.soft_fail_to_hard()
    }
    #[test]
    fn safeToSqlAppliesDefaultLimitWhenNoneSet__972() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___56 = temper_std::testing::Test::new();
        let mut t___2317: SqlFragment;
        let mut t___2318: SqlFragment;
        let q__614: Query = from(sid__304("users"));
        'ok___5161: {
            'orelse___1088: {
                t___2317 = match q__614.safe_to_sql(100) {
                    Ok(x) => x,
                    _ => break 'orelse___1088
                };
                t___2318 = t___2317.clone();
                break 'ok___5161;
            }
            t___2318 = panic!();
        }
        let s__615: std::sync::Arc<String> = t___2318.to_string();
        let mut t___4324: bool = Some(s__615.as_str()) == Some("SELECT * FROM users LIMIT 100");
        #[derive(Clone)]
        struct ClosureGroup___62 {
            s__615: std::sync::Arc<String>
        }
        impl ClosureGroup___62 {
            fn fn__4320(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("should have limit: {}", self.s__615.clone()));
            }
        }
        let closure_group = ClosureGroup___62 {
            s__615: s__615.clone()
        };
        let fn__4320 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__4320())
        };
        test___56.assert(t___4324, fn__4320.clone());
        test___56.soft_fail_to_hard()
    }
    #[test]
    fn safeToSqlRespectsExplicitLimit__973() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___57 = temper_std::testing::Test::new();
        let mut t___2309: Query;
        let mut t___2312: SqlFragment;
        let mut t___2313: SqlFragment;
        let q__617: Query;
        'ok___5162: {
            'orelse___1089: {
                t___2309 = match from(sid__304("users")).limit(5) {
                    Ok(x) => x,
                    _ => break 'orelse___1089
                };
                q__617 = t___2309.clone();
                break 'ok___5162;
            }
            q__617 = panic!();
        }
        'ok___5163: {
            'orelse___1090: {
                t___2312 = match q__617.safe_to_sql(100) {
                    Ok(x) => x,
                    _ => break 'orelse___1090
                };
                t___2313 = t___2312.clone();
                break 'ok___5163;
            }
            t___2313 = panic!();
        }
        let s__618: std::sync::Arc<String> = t___2313.to_string();
        let mut t___4318: bool = Some(s__618.as_str()) == Some("SELECT * FROM users LIMIT 5");
        #[derive(Clone)]
        struct ClosureGroup___63 {
            s__618: std::sync::Arc<String>
        }
        impl ClosureGroup___63 {
            fn fn__4314(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("explicit limit preserved: {}", self.s__618.clone()));
            }
        }
        let closure_group = ClosureGroup___63 {
            s__618: s__618.clone()
        };
        let fn__4314 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__4314())
        };
        test___57.assert(t___4318, fn__4314.clone());
        test___57.soft_fail_to_hard()
    }
    #[test]
    fn safeToSqlBubblesOnNegativeDefaultLimit__974() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___58 = temper_std::testing::Test::new();
        let didBubble__620: bool;
        'ok___5164: {
            'orelse___1091: {
                match from(sid__304("users")).safe_to_sql(-1) {
                    Ok(x) => x,
                    _ => break 'orelse___1091
                };
                didBubble__620 = false;
                break 'ok___5164;
            }
            didBubble__620 = true;
        }
        #[derive(Clone)]
        struct ClosureGroup___64 {}
        impl ClosureGroup___64 {
            fn fn__4310(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("negative defaultLimit should bubble".to_string());
            }
        }
        let closure_group = ClosureGroup___64 {};
        let fn__4310 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__4310())
        };
        test___58.assert(didBubble__620, fn__4310.clone());
        test___58.soft_fail_to_hard()
    }
    #[test]
    fn whereWithInjectionAttemptInStringValueIsEscaped__975() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___59 = temper_std::testing::Test::new();
        let evil__622: std::sync::Arc<String> = std::sync::Arc::new("'; DROP TABLE users; --".to_string());
        let mut t___4294: SafeIdentifier = sid__304("users");
        let mut t___4295: SqlBuilder = SqlBuilder::new();
        t___4295.append_safe("name = ");
        t___4295.append_string("'; DROP TABLE users; --");
        let mut t___4298: SqlFragment = t___4295.accumulated();
        let q__623: Query = from(t___4294.clone()).r#where(t___4298.clone());
        let s__624: std::sync::Arc<String> = q__623.to_sql().to_string();
        let mut t___4303: bool = temper_core::string::index_of( & s__624, "''", None).is_some();
        #[derive(Clone)]
        struct ClosureGroup___65 {
            s__624: std::sync::Arc<String>
        }
        impl ClosureGroup___65 {
            fn fn__4293(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("quotes must be doubled: {}", self.s__624.clone()));
            }
        }
        let closure_group = ClosureGroup___65 {
            s__624: s__624.clone()
        };
        let fn__4293 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__4293())
        };
        test___59.assert(t___4303, fn__4293.clone());
        let mut t___4307: bool = temper_core::string::index_of( & s__624, "SELECT * FROM users WHERE name =", None).is_some();
        #[derive(Clone)]
        struct ClosureGroup___66 {
            s__624: std::sync::Arc<String>
        }
        impl ClosureGroup___66 {
            fn fn__4292(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("structure intact: {}", self.s__624.clone()));
            }
        }
        let closure_group = ClosureGroup___66 {
            s__624: s__624.clone()
        };
        let fn__4292 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__4292())
        };
        test___59.assert(t___4307, fn__4292.clone());
        test___59.soft_fail_to_hard()
    }
    #[test]
    fn safeIdentifierRejectsUserSuppliedTableNameWithMetacharacters__977() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___60 = temper_std::testing::Test::new();
        let attack__626: std::sync::Arc<String> = std::sync::Arc::new("users; DROP TABLE users; --".to_string());
        let didBubble__627: bool;
        'ok___5165: {
            'orelse___1092: {
                match safe_identifier("users; DROP TABLE users; --") {
                    Ok(x) => x,
                    _ => break 'orelse___1092
                };
                didBubble__627 = false;
                break 'ok___5165;
            }
            didBubble__627 = true;
        }
        #[derive(Clone)]
        struct ClosureGroup___67 {}
        impl ClosureGroup___67 {
            fn fn__4289(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("metacharacter-containing name must be rejected at construction".to_string());
            }
        }
        let closure_group = ClosureGroup___67 {};
        let fn__4289 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__4289())
        };
        test___60.assert(didBubble__627, fn__4289.clone());
        test___60.soft_fail_to_hard()
    }
    #[test]
    fn safeIdentifierAcceptsValidNames__978() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___67 = temper_std::testing::Test::new();
        let mut t___2282: SafeIdentifier;
        let id__665: SafeIdentifier;
        'ok___5166: {
            'orelse___1093: {
                t___2282 = match safe_identifier("user_name") {
                    Ok(x) => x,
                    _ => break 'orelse___1093
                };
                id__665 = t___2282.clone();
                break 'ok___5166;
            }
            id__665 = temper_core::cast::<SafeIdentifier>(panic!()).unwrap();
        }
        let mut t___4287: bool = Some(id__665.sql_value().as_str()) == Some("user_name");
        #[derive(Clone)]
        struct ClosureGroup___68 {}
        impl ClosureGroup___68 {
            fn fn__4284(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("value should round-trip".to_string());
            }
        }
        let closure_group = ClosureGroup___68 {};
        let fn__4284 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__4284())
        };
        test___67.assert(t___4287, fn__4284.clone());
        test___67.soft_fail_to_hard()
    }
    #[test]
    fn safeIdentifierRejectsEmptyString__979() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___68 = temper_std::testing::Test::new();
        let didBubble__667: bool;
        'ok___5167: {
            'orelse___1094: {
                match safe_identifier("") {
                    Ok(x) => x,
                    _ => break 'orelse___1094
                };
                didBubble__667 = false;
                break 'ok___5167;
            }
            didBubble__667 = true;
        }
        #[derive(Clone)]
        struct ClosureGroup___69 {}
        impl ClosureGroup___69 {
            fn fn__4281(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("empty string should bubble".to_string());
            }
        }
        let closure_group = ClosureGroup___69 {};
        let fn__4281 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__4281())
        };
        test___68.assert(didBubble__667, fn__4281.clone());
        test___68.soft_fail_to_hard()
    }
    #[test]
    fn safeIdentifierRejectsLeadingDigit__980() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___69 = temper_std::testing::Test::new();
        let didBubble__669: bool;
        'ok___5168: {
            'orelse___1095: {
                match safe_identifier("1col") {
                    Ok(x) => x,
                    _ => break 'orelse___1095
                };
                didBubble__669 = false;
                break 'ok___5168;
            }
            didBubble__669 = true;
        }
        #[derive(Clone)]
        struct ClosureGroup___70 {}
        impl ClosureGroup___70 {
            fn fn__4278(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("leading digit should bubble".to_string());
            }
        }
        let closure_group = ClosureGroup___70 {};
        let fn__4278 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__4278())
        };
        test___69.assert(didBubble__669, fn__4278.clone());
        test___69.soft_fail_to_hard()
    }
    #[test]
    fn safeIdentifierRejectsSqlMetacharacters__981() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___70 = temper_std::testing::Test::new();
        let cases__671: temper_core::List<std::sync::Arc<String>> = std::sync::Arc::new(vec![std::sync::Arc::new("name); DROP TABLE".to_string()), std::sync::Arc::new("col'".to_string()), std::sync::Arc::new("a b".to_string()), std::sync::Arc::new("a-b".to_string()), std::sync::Arc::new("a.b".to_string()), std::sync::Arc::new("a;b".to_string())]);
        #[derive(Clone)]
        struct ClosureGroup___71 {
            test___70: temper_std::testing::Test
        }
        impl ClosureGroup___71 {
            fn fn__4275(& self, c__672: impl temper_core::ToArcString) {
                let c__672 = c__672.to_arc_string();
                let didBubble__673: bool;
                'ok___5169: {
                    'orelse___1096: {
                        match safe_identifier(c__672.clone()) {
                            Ok(x) => x,
                            _ => break 'orelse___1096
                        };
                        didBubble__673 = false;
                        break 'ok___5169;
                    }
                    didBubble__673 = true;
                }
                #[derive(Clone)]
                struct ClosureGroup___72 {
                    c__672: std::sync::Arc<String>
                }
                impl ClosureGroup___72 {
                    fn fn__4272(& self) -> std::sync::Arc<String> {
                        return std::sync::Arc::new(format!("should reject: {}", self.c__672.clone()));
                    }
                }
                let closure_group = ClosureGroup___72 {
                    c__672: c__672.clone()
                };
                let fn__4272 = {
                    let closure_group = closure_group.clone();
                    std::sync::Arc::new(move | | closure_group.fn__4272())
                };
                self.test___70.assert(didBubble__673, fn__4272.clone());
            }
        }
        let closure_group = ClosureGroup___71 {
            test___70: test___70.clone()
        };
        let fn__4275 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | c__672: std::sync::Arc<String> | closure_group.fn__4275(c__672))
        };
        temper_core::listed::list_for_each( & cases__671, & ( * fn__4275.clone()));
        test___70.soft_fail_to_hard()
    }
    #[test]
    fn tableDefFieldLookupFound__982() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___71 = temper_std::testing::Test::new();
        let mut t___2259: SafeIdentifier;
        let mut t___2260: SafeIdentifier;
        let mut t___2261: SafeIdentifier;
        let mut t___2262: SafeIdentifier;
        let mut t___2265: SafeIdentifier;
        let mut t___2266: SafeIdentifier;
        let mut t___2270: FieldDef;
        'ok___5170: {
            'orelse___1097: {
                t___2259 = match safe_identifier("users") {
                    Ok(x) => x,
                    _ => break 'orelse___1097
                };
                t___2260 = t___2259.clone();
                break 'ok___5170;
            }
            t___2260 = temper_core::cast::<SafeIdentifier>(panic!()).unwrap();
        }
        'ok___5171: {
            'orelse___1098: {
                t___2261 = match safe_identifier("name") {
                    Ok(x) => x,
                    _ => break 'orelse___1098
                };
                t___2262 = t___2261.clone();
                break 'ok___5171;
            }
            t___2262 = temper_core::cast::<SafeIdentifier>(panic!()).unwrap();
        }
        let mut t___4262: StringField = StringField::new();
        let mut t___4263: FieldDef = FieldDef::new(t___2262.clone(), FieldType::new(t___4262.clone()), false);
        'ok___5172: {
            'orelse___1099: {
                t___2265 = match safe_identifier("age") {
                    Ok(x) => x,
                    _ => break 'orelse___1099
                };
                t___2266 = t___2265.clone();
                break 'ok___5172;
            }
            t___2266 = temper_core::cast::<SafeIdentifier>(panic!()).unwrap();
        }
        let mut t___4264: IntField = IntField::new();
        let mut t___4265: FieldDef = FieldDef::new(t___2266.clone(), FieldType::new(t___4264.clone()), false);
        let td__675: TableDef = TableDef::new(t___2260.clone(), [t___4263.clone(), t___4265.clone()]);
        let f__676: FieldDef;
        'ok___5173: {
            'orelse___1100: {
                t___2270 = match td__675.field("age") {
                    Ok(x) => x,
                    _ => break 'orelse___1100
                };
                f__676 = t___2270.clone();
                break 'ok___5173;
            }
            f__676 = panic!();
        }
        let mut t___4270: bool = Some(f__676.name().sql_value().as_str()) == Some("age");
        #[derive(Clone)]
        struct ClosureGroup___73 {}
        impl ClosureGroup___73 {
            fn fn__4261(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("should find age field".to_string());
            }
        }
        let closure_group = ClosureGroup___73 {};
        let fn__4261 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__4261())
        };
        test___71.assert(t___4270, fn__4261.clone());
        test___71.soft_fail_to_hard()
    }
    #[test]
    fn tableDefFieldLookupNotFoundBubbles__983() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___72 = temper_std::testing::Test::new();
        let mut t___2250: SafeIdentifier;
        let mut t___2251: SafeIdentifier;
        let mut t___2252: SafeIdentifier;
        let mut t___2253: SafeIdentifier;
        'ok___5174: {
            'orelse___1101: {
                t___2250 = match safe_identifier("users") {
                    Ok(x) => x,
                    _ => break 'orelse___1101
                };
                t___2251 = t___2250.clone();
                break 'ok___5174;
            }
            t___2251 = temper_core::cast::<SafeIdentifier>(panic!()).unwrap();
        }
        'ok___5175: {
            'orelse___1102: {
                t___2252 = match safe_identifier("name") {
                    Ok(x) => x,
                    _ => break 'orelse___1102
                };
                t___2253 = t___2252.clone();
                break 'ok___5175;
            }
            t___2253 = temper_core::cast::<SafeIdentifier>(panic!()).unwrap();
        }
        let mut t___4256: StringField = StringField::new();
        let mut t___4257: FieldDef = FieldDef::new(t___2253.clone(), FieldType::new(t___4256.clone()), false);
        let td__678: TableDef = TableDef::new(t___2251.clone(), [t___4257.clone()]);
        let didBubble__679: bool;
        'ok___5176: {
            'orelse___1103: {
                match td__678.field("nonexistent") {
                    Ok(x) => x,
                    _ => break 'orelse___1103
                };
                didBubble__679 = false;
                break 'ok___5176;
            }
            didBubble__679 = true;
        }
        #[derive(Clone)]
        struct ClosureGroup___74 {}
        impl ClosureGroup___74 {
            fn fn__4255(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("unknown field should bubble".to_string());
            }
        }
        let closure_group = ClosureGroup___74 {};
        let fn__4255 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__4255())
        };
        test___72.assert(didBubble__679, fn__4255.clone());
        test___72.soft_fail_to_hard()
    }
    #[test]
    fn fieldDefNullableFlag__984() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___73 = temper_std::testing::Test::new();
        let mut t___2238: SafeIdentifier;
        let mut t___2239: SafeIdentifier;
        let mut t___2242: SafeIdentifier;
        let mut t___2243: SafeIdentifier;
        'ok___5177: {
            'orelse___1104: {
                t___2238 = match safe_identifier("email") {
                    Ok(x) => x,
                    _ => break 'orelse___1104
                };
                t___2239 = t___2238.clone();
                break 'ok___5177;
            }
            t___2239 = temper_core::cast::<SafeIdentifier>(panic!()).unwrap();
        }
        let mut t___4244: StringField = StringField::new();
        let required__681: FieldDef = FieldDef::new(t___2239.clone(), FieldType::new(t___4244.clone()), false);
        'ok___5178: {
            'orelse___1105: {
                t___2242 = match safe_identifier("bio") {
                    Ok(x) => x,
                    _ => break 'orelse___1105
                };
                t___2243 = t___2242.clone();
                break 'ok___5178;
            }
            t___2243 = temper_core::cast::<SafeIdentifier>(panic!()).unwrap();
        }
        let mut t___4246: StringField = StringField::new();
        let optional__682: FieldDef = FieldDef::new(t___2243.clone(), FieldType::new(t___4246.clone()), true);
        let mut t___4250: bool = ! required__681.nullable();
        #[derive(Clone)]
        struct ClosureGroup___75 {}
        impl ClosureGroup___75 {
            fn fn__4243(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("required field should not be nullable".to_string());
            }
        }
        let closure_group = ClosureGroup___75 {};
        let fn__4243 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__4243())
        };
        test___73.assert(t___4250, fn__4243.clone());
        let mut t___4252: bool = optional__682.nullable();
        #[derive(Clone)]
        struct ClosureGroup___76 {}
        impl ClosureGroup___76 {
            fn fn__4242(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("optional field should be nullable".to_string());
            }
        }
        let closure_group = ClosureGroup___76 {};
        let fn__4242 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__4242())
        };
        test___73.assert(t___4252, fn__4242.clone());
        test___73.soft_fail_to_hard()
    }
    #[test]
    fn stringEscaping__985() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___77 = temper_std::testing::Test::new();
        #[derive(Clone)]
        struct ClosureGroup___77 {}
        impl ClosureGroup___77 {
            fn build__808(& self, name__810: impl temper_core::ToArcString) -> std::sync::Arc<String> {
                let name__810 = name__810.to_arc_string();
                let mut t___4224: SqlBuilder = SqlBuilder::new();
                t___4224.append_safe("select * from hi where name = ");
                t___4224.append_string(name__810.clone());
                return t___4224.accumulated().to_string();
            }
            fn buildWrong__809(& self, name__812: impl temper_core::ToArcString) -> std::sync::Arc<String> {
                let name__812 = name__812.to_arc_string();
                return std::sync::Arc::new(format!("select * from hi where name = '{}'", name__812.clone()));
            }
        }
        let closure_group = ClosureGroup___77 {};
        let build__808 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | name__810: std::sync::Arc<String> | closure_group.build__808(name__810))
        };
        let buildWrong__809 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | name__812: std::sync::Arc<String> | closure_group.buildWrong__809(name__812))
        };
        let actual___987: std::sync::Arc<String> = build__808(std::sync::Arc::new("world".to_string()));
        let mut t___4234: bool = Some(actual___987.as_str()) == Some("select * from hi where name = 'world'");
        #[derive(Clone)]
        struct ClosureGroup___78 {
            actual___987: std::sync::Arc<String>
        }
        impl ClosureGroup___78 {
            fn fn__4231(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected build(\"world\") == (select * from hi where name = 'world') not ({})", self.actual___987.clone()));
            }
        }
        let closure_group = ClosureGroup___78 {
            actual___987: actual___987.clone()
        };
        let fn__4231 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__4231())
        };
        test___77.assert(t___4234, fn__4231.clone());
        let bobbyTables__814: std::sync::Arc<String> = std::sync::Arc::new("Robert'); drop table hi;--".to_string());
        let actual___989: std::sync::Arc<String> = build__808(std::sync::Arc::new("Robert'); drop table hi;--".to_string()));
        let mut t___4238: bool = Some(actual___989.as_str()) == Some("select * from hi where name = 'Robert''); drop table hi;--'");
        #[derive(Clone)]
        struct ClosureGroup___79 {
            actual___989: std::sync::Arc<String>
        }
        impl ClosureGroup___79 {
            fn fn__4230(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected build(bobbyTables) == (select * from hi where name = 'Robert''); drop table hi;--') not ({})", self.actual___989.clone()));
            }
        }
        let closure_group = ClosureGroup___79 {
            actual___989: actual___989.clone()
        };
        let fn__4230 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__4230())
        };
        test___77.assert(t___4238, fn__4230.clone());
        #[derive(Clone)]
        struct ClosureGroup___80 {}
        impl ClosureGroup___80 {
            fn fn__4229(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("expected buildWrong(bobbyTables) == (select * from hi where name = 'Robert'); drop table hi;--') not (select * from hi where name = 'Robert'); drop table hi;--')".to_string());
            }
        }
        let closure_group = ClosureGroup___80 {};
        let fn__4229 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__4229())
        };
        test___77.assert(true, fn__4229.clone());
        test___77.soft_fail_to_hard()
    }
    #[test]
    fn stringEdgeCases__993() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___78 = temper_std::testing::Test::new();
        let mut t___4192: SqlBuilder = SqlBuilder::new();
        t___4192.append_safe("v = ");
        t___4192.append_string("");
        let actual___994: std::sync::Arc<String> = t___4192.accumulated().to_string();
        let mut t___4198: bool = Some(actual___994.as_str()) == Some("v = ''");
        #[derive(Clone)]
        struct ClosureGroup___81 {
            actual___994: std::sync::Arc<String>
        }
        impl ClosureGroup___81 {
            fn fn__4191(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"v = \", \\interpolate, \"\").toString() == (v = '') not ({})", self.actual___994.clone()));
            }
        }
        let closure_group = ClosureGroup___81 {
            actual___994: actual___994.clone()
        };
        let fn__4191 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__4191())
        };
        test___78.assert(t___4198, fn__4191.clone());
        let mut t___4200: SqlBuilder = SqlBuilder::new();
        t___4200.append_safe("v = ");
        t___4200.append_string("a''b");
        let actual___997: std::sync::Arc<String> = t___4200.accumulated().to_string();
        let mut t___4206: bool = Some(actual___997.as_str()) == Some("v = 'a''''b'");
        #[derive(Clone)]
        struct ClosureGroup___82 {
            actual___997: std::sync::Arc<String>
        }
        impl ClosureGroup___82 {
            fn fn__4190(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"v = \", \\interpolate, \"a''b\").toString() == (v = 'a''''b') not ({})", self.actual___997.clone()));
            }
        }
        let closure_group = ClosureGroup___82 {
            actual___997: actual___997.clone()
        };
        let fn__4190 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__4190())
        };
        test___78.assert(t___4206, fn__4190.clone());
        let mut t___4208: SqlBuilder = SqlBuilder::new();
        t___4208.append_safe("v = ");
        t___4208.append_string("Hello 世界");
        let actual___1000: std::sync::Arc<String> = t___4208.accumulated().to_string();
        let mut t___4214: bool = Some(actual___1000.as_str()) == Some("v = 'Hello 世界'");
        #[derive(Clone)]
        struct ClosureGroup___83 {
            actual___1000: std::sync::Arc<String>
        }
        impl ClosureGroup___83 {
            fn fn__4189(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"v = \", \\interpolate, \"Hello 世界\").toString() == (v = 'Hello 世界') not ({})", self.actual___1000.clone()));
            }
        }
        let closure_group = ClosureGroup___83 {
            actual___1000: actual___1000.clone()
        };
        let fn__4189 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__4189())
        };
        test___78.assert(t___4214, fn__4189.clone());
        let mut t___4216: SqlBuilder = SqlBuilder::new();
        t___4216.append_safe("v = ");
        t___4216.append_string("Line1\x0aLine2");
        let actual___1003: std::sync::Arc<String> = t___4216.accumulated().to_string();
        let mut t___4222: bool = Some(actual___1003.as_str()) == Some("v = 'Line1\x0aLine2'");
        #[derive(Clone)]
        struct ClosureGroup___84 {
            actual___1003: std::sync::Arc<String>
        }
        impl ClosureGroup___84 {
            fn fn__4188(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"v = \", \\interpolate, \"Line1\\nLine2\").toString() == (v = 'Line1\x0aLine2') not ({})", self.actual___1003.clone()));
            }
        }
        let closure_group = ClosureGroup___84 {
            actual___1003: actual___1003.clone()
        };
        let fn__4188 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__4188())
        };
        test___78.assert(t___4222, fn__4188.clone());
        test___78.soft_fail_to_hard()
    }
    #[test]
    fn numbersAndBooleans__1006() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___79 = temper_std::testing::Test::new();
        let mut t___2183: temper_std::temporal::Date;
        let mut t___4163: SqlBuilder = SqlBuilder::new();
        t___4163.append_safe("select ");
        t___4163.append_int32(42);
        t___4163.append_safe(", ");
        t___4163.append_int64(43);
        t___4163.append_safe(", ");
        t___4163.append_float64(19.99f64);
        t___4163.append_safe(", ");
        t___4163.append_boolean(true);
        t___4163.append_safe(", ");
        t___4163.append_boolean(false);
        let actual___1007: std::sync::Arc<String> = t___4163.accumulated().to_string();
        let mut t___4177: bool = Some(actual___1007.as_str()) == Some("select 42, 43, 19.99, TRUE, FALSE");
        #[derive(Clone)]
        struct ClosureGroup___85 {
            actual___1007: std::sync::Arc<String>
        }
        impl ClosureGroup___85 {
            fn fn__4162(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"select \", \\interpolate, 42, \", \", \\interpolate, 43, \", \", \\interpolate, 19.99, \", \", \\interpolate, true, \", \", \\interpolate, false).toString() == (select 42, 43, 19.99, TRUE, FALSE) not ({})", self.actual___1007.clone()));
            }
        }
        let closure_group = ClosureGroup___85 {
            actual___1007: actual___1007.clone()
        };
        let fn__4162 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__4162())
        };
        test___79.assert(t___4177, fn__4162.clone());
        let date__817: temper_std::temporal::Date;
        'ok___5179: {
            'orelse___1106: {
                t___2183 = match temper_std::temporal::Date::new(2024, 12, 25) {
                    Ok(x) => x,
                    _ => break 'orelse___1106
                };
                date__817 = t___2183.clone();
                break 'ok___5179;
            }
            date__817 = panic!();
        }
        let mut t___4179: SqlBuilder = SqlBuilder::new();
        t___4179.append_safe("insert into t values (");
        t___4179.append_date(date__817.clone());
        t___4179.append_safe(")");
        let actual___1010: std::sync::Arc<String> = t___4179.accumulated().to_string();
        let mut t___4186: bool = Some(actual___1010.as_str()) == Some("insert into t values ('2024-12-25')");
        #[derive(Clone)]
        struct ClosureGroup___86 {
            actual___1010: std::sync::Arc<String>
        }
        impl ClosureGroup___86 {
            fn fn__4161(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"insert into t values (\", \\interpolate, date, \")\").toString() == (insert into t values ('2024-12-25')) not ({})", self.actual___1010.clone()));
            }
        }
        let closure_group = ClosureGroup___86 {
            actual___1010: actual___1010.clone()
        };
        let fn__4161 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__4161())
        };
        test___79.assert(t___4186, fn__4161.clone());
        test___79.soft_fail_to_hard()
    }
    #[test]
    fn lists__1013() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___80 = temper_std::testing::Test::new();
        let mut t___2155: temper_std::temporal::Date;
        let mut t___2156: temper_std::temporal::Date;
        let mut t___2157: temper_std::temporal::Date;
        let mut t___2158: temper_std::temporal::Date;
        let mut t___4107: SqlBuilder = SqlBuilder::new();
        t___4107.append_safe("v IN (");
        t___4107.append_string_list(temper_core::ToListed::to_listed([std::sync::Arc::new("a".to_string()), std::sync::Arc::new("b".to_string()), std::sync::Arc::new("c'd".to_string())]));
        t___4107.append_safe(")");
        let actual___1014: std::sync::Arc<String> = t___4107.accumulated().to_string();
        let mut t___4114: bool = Some(actual___1014.as_str()) == Some("v IN ('a', 'b', 'c''d')");
        #[derive(Clone)]
        struct ClosureGroup___87 {
            actual___1014: std::sync::Arc<String>
        }
        impl ClosureGroup___87 {
            fn fn__4106(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"v IN (\", \\interpolate, list(\"a\", \"b\", \"c'd\"), \")\").toString() == (v IN ('a', 'b', 'c''d')) not ({})", self.actual___1014.clone()));
            }
        }
        let closure_group = ClosureGroup___87 {
            actual___1014: actual___1014.clone()
        };
        let fn__4106 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__4106())
        };
        test___80.assert(t___4114, fn__4106.clone());
        let mut t___4116: SqlBuilder = SqlBuilder::new();
        t___4116.append_safe("v IN (");
        t___4116.append_int32_list(temper_core::ToListed::to_listed([1, 2, 3]));
        t___4116.append_safe(")");
        let actual___1017: std::sync::Arc<String> = t___4116.accumulated().to_string();
        let mut t___4123: bool = Some(actual___1017.as_str()) == Some("v IN (1, 2, 3)");
        #[derive(Clone)]
        struct ClosureGroup___88 {
            actual___1017: std::sync::Arc<String>
        }
        impl ClosureGroup___88 {
            fn fn__4105(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"v IN (\", \\interpolate, list(1, 2, 3), \")\").toString() == (v IN (1, 2, 3)) not ({})", self.actual___1017.clone()));
            }
        }
        let closure_group = ClosureGroup___88 {
            actual___1017: actual___1017.clone()
        };
        let fn__4105 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__4105())
        };
        test___80.assert(t___4123, fn__4105.clone());
        let mut t___4125: SqlBuilder = SqlBuilder::new();
        t___4125.append_safe("v IN (");
        t___4125.append_int64_list(temper_core::ToListed::to_listed([1, 2]));
        t___4125.append_safe(")");
        let actual___1020: std::sync::Arc<String> = t___4125.accumulated().to_string();
        let mut t___4132: bool = Some(actual___1020.as_str()) == Some("v IN (1, 2)");
        #[derive(Clone)]
        struct ClosureGroup___89 {
            actual___1020: std::sync::Arc<String>
        }
        impl ClosureGroup___89 {
            fn fn__4104(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"v IN (\", \\interpolate, list(1, 2), \")\").toString() == (v IN (1, 2)) not ({})", self.actual___1020.clone()));
            }
        }
        let closure_group = ClosureGroup___89 {
            actual___1020: actual___1020.clone()
        };
        let fn__4104 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__4104())
        };
        test___80.assert(t___4132, fn__4104.clone());
        let mut t___4134: SqlBuilder = SqlBuilder::new();
        t___4134.append_safe("v IN (");
        t___4134.append_float64_list(temper_core::ToListed::to_listed([1.0f64, 2.0f64]));
        t___4134.append_safe(")");
        let actual___1023: std::sync::Arc<String> = t___4134.accumulated().to_string();
        let mut t___4141: bool = Some(actual___1023.as_str()) == Some("v IN (1.0, 2.0)");
        #[derive(Clone)]
        struct ClosureGroup___90 {
            actual___1023: std::sync::Arc<String>
        }
        impl ClosureGroup___90 {
            fn fn__4103(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"v IN (\", \\interpolate, list(1.0, 2.0), \")\").toString() == (v IN (1.0, 2.0)) not ({})", self.actual___1023.clone()));
            }
        }
        let closure_group = ClosureGroup___90 {
            actual___1023: actual___1023.clone()
        };
        let fn__4103 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__4103())
        };
        test___80.assert(t___4141, fn__4103.clone());
        let mut t___4143: SqlBuilder = SqlBuilder::new();
        t___4143.append_safe("v IN (");
        t___4143.append_boolean_list(temper_core::ToListed::to_listed([true, false]));
        t___4143.append_safe(")");
        let actual___1026: std::sync::Arc<String> = t___4143.accumulated().to_string();
        let mut t___4150: bool = Some(actual___1026.as_str()) == Some("v IN (TRUE, FALSE)");
        #[derive(Clone)]
        struct ClosureGroup___91 {
            actual___1026: std::sync::Arc<String>
        }
        impl ClosureGroup___91 {
            fn fn__4102(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"v IN (\", \\interpolate, list(true, false), \")\").toString() == (v IN (TRUE, FALSE)) not ({})", self.actual___1026.clone()));
            }
        }
        let closure_group = ClosureGroup___91 {
            actual___1026: actual___1026.clone()
        };
        let fn__4102 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__4102())
        };
        test___80.assert(t___4150, fn__4102.clone());
        'ok___5180: {
            'orelse___1107: {
                t___2155 = match temper_std::temporal::Date::new(2024, 1, 1) {
                    Ok(x) => x,
                    _ => break 'orelse___1107
                };
                t___2156 = t___2155.clone();
                break 'ok___5180;
            }
            t___2156 = panic!();
        }
        'ok___5181: {
            'orelse___1108: {
                t___2157 = match temper_std::temporal::Date::new(2024, 12, 25) {
                    Ok(x) => x,
                    _ => break 'orelse___1108
                };
                t___2158 = t___2157.clone();
                break 'ok___5181;
            }
            t___2158 = panic!();
        }
        let dates__819: temper_core::List<temper_std::temporal::Date> = std::sync::Arc::new(vec![t___2156.clone(), t___2158.clone()]);
        let mut t___4152: SqlBuilder = SqlBuilder::new();
        t___4152.append_safe("v IN (");
        t___4152.append_date_list(temper_core::ToListed::to_listed(dates__819.clone()));
        t___4152.append_safe(")");
        let actual___1029: std::sync::Arc<String> = t___4152.accumulated().to_string();
        let mut t___4159: bool = Some(actual___1029.as_str()) == Some("v IN ('2024-01-01', '2024-12-25')");
        #[derive(Clone)]
        struct ClosureGroup___92 {
            actual___1029: std::sync::Arc<String>
        }
        impl ClosureGroup___92 {
            fn fn__4101(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"v IN (\", \\interpolate, dates, \")\").toString() == (v IN ('2024-01-01', '2024-12-25')) not ({})", self.actual___1029.clone()));
            }
        }
        let closure_group = ClosureGroup___92 {
            actual___1029: actual___1029.clone()
        };
        let fn__4101 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__4101())
        };
        test___80.assert(t___4159, fn__4101.clone());
        test___80.soft_fail_to_hard()
    }
    #[test]
    fn sqlFloat64_naNRendersAsNull__1032() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___81 = temper_std::testing::Test::new();
        let nan__821: f64;
        nan__821 = temper_core::float64::div(0.0f64, 0.0f64) ? ;
        let mut t___4093: SqlBuilder = SqlBuilder::new();
        t___4093.append_safe("v = ");
        t___4093.append_float64(nan__821);
        let actual___1033: std::sync::Arc<String> = t___4093.accumulated().to_string();
        let mut t___4099: bool = Some(actual___1033.as_str()) == Some("v = NULL");
        #[derive(Clone)]
        struct ClosureGroup___93 {
            actual___1033: std::sync::Arc<String>
        }
        impl ClosureGroup___93 {
            fn fn__4092(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"v = \", \\interpolate, nan).toString() == (v = NULL) not ({})", self.actual___1033.clone()));
            }
        }
        let closure_group = ClosureGroup___93 {
            actual___1033: actual___1033.clone()
        };
        let fn__4092 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__4092())
        };
        test___81.assert(t___4099, fn__4092.clone());
        test___81.soft_fail_to_hard()
    }
    #[test]
    fn sqlFloat64_infinityRendersAsNull__1036() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___82 = temper_std::testing::Test::new();
        let inf__823: f64;
        inf__823 = temper_core::float64::div(1.0f64, 0.0f64) ? ;
        let mut t___4084: SqlBuilder = SqlBuilder::new();
        t___4084.append_safe("v = ");
        t___4084.append_float64(inf__823);
        let actual___1037: std::sync::Arc<String> = t___4084.accumulated().to_string();
        let mut t___4090: bool = Some(actual___1037.as_str()) == Some("v = NULL");
        #[derive(Clone)]
        struct ClosureGroup___94 {
            actual___1037: std::sync::Arc<String>
        }
        impl ClosureGroup___94 {
            fn fn__4083(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"v = \", \\interpolate, inf).toString() == (v = NULL) not ({})", self.actual___1037.clone()));
            }
        }
        let closure_group = ClosureGroup___94 {
            actual___1037: actual___1037.clone()
        };
        let fn__4083 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__4083())
        };
        test___82.assert(t___4090, fn__4083.clone());
        test___82.soft_fail_to_hard()
    }
    #[test]
    fn sqlFloat64_negativeInfinityRendersAsNull__1040() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___83 = temper_std::testing::Test::new();
        let ninf__825: f64;
        ninf__825 = temper_core::float64::div(-1.0f64, 0.0f64) ? ;
        let mut t___4075: SqlBuilder = SqlBuilder::new();
        t___4075.append_safe("v = ");
        t___4075.append_float64(ninf__825);
        let actual___1041: std::sync::Arc<String> = t___4075.accumulated().to_string();
        let mut t___4081: bool = Some(actual___1041.as_str()) == Some("v = NULL");
        #[derive(Clone)]
        struct ClosureGroup___95 {
            actual___1041: std::sync::Arc<String>
        }
        impl ClosureGroup___95 {
            fn fn__4074(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"v = \", \\interpolate, ninf).toString() == (v = NULL) not ({})", self.actual___1041.clone()));
            }
        }
        let closure_group = ClosureGroup___95 {
            actual___1041: actual___1041.clone()
        };
        let fn__4074 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__4074())
        };
        test___83.assert(t___4081, fn__4074.clone());
        test___83.soft_fail_to_hard()
    }
    #[test]
    fn sqlFloat64_normalValuesStillWork__1044() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___84 = temper_std::testing::Test::new();
        let mut t___4050: SqlBuilder = SqlBuilder::new();
        t___4050.append_safe("v = ");
        t___4050.append_float64(3.14f64);
        let actual___1045: std::sync::Arc<String> = t___4050.accumulated().to_string();
        let mut t___4056: bool = Some(actual___1045.as_str()) == Some("v = 3.14");
        #[derive(Clone)]
        struct ClosureGroup___96 {
            actual___1045: std::sync::Arc<String>
        }
        impl ClosureGroup___96 {
            fn fn__4049(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"v = \", \\interpolate, 3.14).toString() == (v = 3.14) not ({})", self.actual___1045.clone()));
            }
        }
        let closure_group = ClosureGroup___96 {
            actual___1045: actual___1045.clone()
        };
        let fn__4049 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__4049())
        };
        test___84.assert(t___4056, fn__4049.clone());
        let mut t___4058: SqlBuilder = SqlBuilder::new();
        t___4058.append_safe("v = ");
        t___4058.append_float64(0.0f64);
        let actual___1048: std::sync::Arc<String> = t___4058.accumulated().to_string();
        let mut t___4064: bool = Some(actual___1048.as_str()) == Some("v = 0.0");
        #[derive(Clone)]
        struct ClosureGroup___97 {
            actual___1048: std::sync::Arc<String>
        }
        impl ClosureGroup___97 {
            fn fn__4048(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"v = \", \\interpolate, 0.0).toString() == (v = 0.0) not ({})", self.actual___1048.clone()));
            }
        }
        let closure_group = ClosureGroup___97 {
            actual___1048: actual___1048.clone()
        };
        let fn__4048 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__4048())
        };
        test___84.assert(t___4064, fn__4048.clone());
        let mut t___4066: SqlBuilder = SqlBuilder::new();
        t___4066.append_safe("v = ");
        t___4066.append_float64(-42.5f64);
        let actual___1051: std::sync::Arc<String> = t___4066.accumulated().to_string();
        let mut t___4072: bool = Some(actual___1051.as_str()) == Some("v = -42.5");
        #[derive(Clone)]
        struct ClosureGroup___98 {
            actual___1051: std::sync::Arc<String>
        }
        impl ClosureGroup___98 {
            fn fn__4047(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"v = \", \\interpolate, -42.5).toString() == (v = -42.5) not ({})", self.actual___1051.clone()));
            }
        }
        let closure_group = ClosureGroup___98 {
            actual___1051: actual___1051.clone()
        };
        let fn__4047 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__4047())
        };
        test___84.assert(t___4072, fn__4047.clone());
        test___84.soft_fail_to_hard()
    }
    #[test]
    fn sqlDateRendersWithQuotes__1054() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___85 = temper_std::testing::Test::new();
        let mut t___2051: temper_std::temporal::Date;
        let d__828: temper_std::temporal::Date;
        'ok___5182: {
            'orelse___1109: {
                t___2051 = match temper_std::temporal::Date::new(2024, 6, 15) {
                    Ok(x) => x,
                    _ => break 'orelse___1109
                };
                d__828 = t___2051.clone();
                break 'ok___5182;
            }
            d__828 = panic!();
        }
        let mut t___4039: SqlBuilder = SqlBuilder::new();
        t___4039.append_safe("v = ");
        t___4039.append_date(d__828.clone());
        let actual___1055: std::sync::Arc<String> = t___4039.accumulated().to_string();
        let mut t___4045: bool = Some(actual___1055.as_str()) == Some("v = '2024-06-15'");
        #[derive(Clone)]
        struct ClosureGroup___99 {
            actual___1055: std::sync::Arc<String>
        }
        impl ClosureGroup___99 {
            fn fn__4038(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"v = \", \\interpolate, d).toString() == (v = '2024-06-15') not ({})", self.actual___1055.clone()));
            }
        }
        let closure_group = ClosureGroup___99 {
            actual___1055: actual___1055.clone()
        };
        let fn__4038 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__4038())
        };
        test___85.assert(t___4045, fn__4038.clone());
        test___85.soft_fail_to_hard()
    }
    #[test]
    fn nesting__1058() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___86 = temper_std::testing::Test::new();
        let name__830: std::sync::Arc<String> = std::sync::Arc::new("Someone".to_string());
        let mut t___4007: SqlBuilder = SqlBuilder::new();
        t___4007.append_safe("where p.last_name = ");
        t___4007.append_string("Someone");
        let condition__831: SqlFragment = t___4007.accumulated();
        let mut t___4011: SqlBuilder = SqlBuilder::new();
        t___4011.append_safe("select p.id from person p ");
        t___4011.append_fragment(condition__831.clone());
        let actual___1060: std::sync::Arc<String> = t___4011.accumulated().to_string();
        let mut t___4017: bool = Some(actual___1060.as_str()) == Some("select p.id from person p where p.last_name = 'Someone'");
        #[derive(Clone)]
        struct ClosureGroup___100 {
            actual___1060: std::sync::Arc<String>
        }
        impl ClosureGroup___100 {
            fn fn__4006(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"select p.id from person p \", \\interpolate, condition).toString() == (select p.id from person p where p.last_name = 'Someone') not ({})", self.actual___1060.clone()));
            }
        }
        let closure_group = ClosureGroup___100 {
            actual___1060: actual___1060.clone()
        };
        let fn__4006 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__4006())
        };
        test___86.assert(t___4017, fn__4006.clone());
        let mut t___4019: SqlBuilder = SqlBuilder::new();
        t___4019.append_safe("select p.id from person p ");
        t___4019.append_part(SqlPart::new(condition__831.to_source()));
        let actual___1063: std::sync::Arc<String> = t___4019.accumulated().to_string();
        let mut t___4026: bool = Some(actual___1063.as_str()) == Some("select p.id from person p where p.last_name = 'Someone'");
        #[derive(Clone)]
        struct ClosureGroup___101 {
            actual___1063: std::sync::Arc<String>
        }
        impl ClosureGroup___101 {
            fn fn__4005(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"select p.id from person p \", \\interpolate, condition.toSource()).toString() == (select p.id from person p where p.last_name = 'Someone') not ({})", self.actual___1063.clone()));
            }
        }
        let closure_group = ClosureGroup___101 {
            actual___1063: actual___1063.clone()
        };
        let fn__4005 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__4005())
        };
        test___86.assert(t___4026, fn__4005.clone());
        let parts__832: temper_core::List<SqlPart> = std::sync::Arc::new(vec![SqlPart::new(SqlString::new("a'b")), SqlPart::new(SqlInt32::new(3))]);
        let mut t___4030: SqlBuilder = SqlBuilder::new();
        t___4030.append_safe("select ");
        t___4030.append_part_list(parts__832.clone());
        let actual___1066: std::sync::Arc<String> = t___4030.accumulated().to_string();
        let mut t___4036: bool = Some(actual___1066.as_str()) == Some("select 'a''b', 3");
        #[derive(Clone)]
        struct ClosureGroup___102 {
            actual___1066: std::sync::Arc<String>
        }
        impl ClosureGroup___102 {
            fn fn__4004(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"select \", \\interpolate, parts).toString() == (select 'a''b', 3) not ({})", self.actual___1066.clone()));
            }
        }
        let closure_group = ClosureGroup___102 {
            actual___1066: actual___1066.clone()
        };
        let fn__4004 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__4004())
        };
        test___86.assert(t___4036, fn__4004.clone());
        test___86.soft_fail_to_hard()
    }
    use super::*;
}
