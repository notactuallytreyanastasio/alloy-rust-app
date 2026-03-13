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
    pub fn new(field__513: impl temper_core::ToArcString, message__514: impl temper_core::ToArcString) -> ChangesetError {
        let field__513 = field__513.to_arc_string();
        let message__514 = message__514.to_arc_string();
        let field;
        let message;
        field = field__513.clone();
        message = message__514.clone();
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
    fn cast(& self, allowedFields__524: temper_core::List<SafeIdentifier>) -> Changeset;
    fn validate_required(& self, fields__527: temper_core::List<SafeIdentifier>) -> Changeset;
    fn validate_length(& self, field__530: SafeIdentifier, min__531: i32, max__532: i32) -> Changeset;
    fn validate_int(& self, field__535: SafeIdentifier) -> Changeset;
    fn validate_int64(& self, field__538: SafeIdentifier) -> Changeset;
    fn validate_float(& self, field__541: SafeIdentifier) -> Changeset;
    fn validate_bool(& self, field__544: SafeIdentifier) -> Changeset;
    fn to_insert_sql(& self) -> temper_core::Result<SqlFragment>;
    fn to_update_sql(& self, id__549: i32) -> temper_core::Result<SqlFragment>;
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
    pub fn cast(& self, allowedFields__565: impl temper_core::ToList<SafeIdentifier>) -> Changeset {
        let allowedFields__565 = allowedFields__565.to_list();
        let mb__567: temper_core::MapBuilder<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::MapBuilder::new();
        #[derive(Clone)]
        struct ClosureGroup___1 {
            this__179: ChangesetImpl, mb__567: temper_core::MapBuilder<std::sync::Arc<String>, std::sync::Arc<String>>
        }
        impl ClosureGroup___1 {
            fn fn__10837(& self, f__568: SafeIdentifier) {
                let mut t___10835: std::sync::Arc<String>;
                let mut t___10832: std::sync::Arc<String> = f__568.sql_value();
                let val__569: std::sync::Arc<String> = temper_core::MappedTrait::get_or( & self.this__179.0.params, t___10832.clone(), std::sync::Arc::new("".to_string()));
                if ! val__569.is_empty() {
                    t___10835 = f__568.sql_value();
                    temper_core::MapBuilder::set( & self.mb__567, t___10835.clone(), val__569.clone());
                }
            }
        }
        let closure_group = ClosureGroup___1 {
            this__179: self.clone(), mb__567: mb__567.clone()
        };
        let fn__10837 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | f__568: SafeIdentifier | closure_group.fn__10837(f__568))
        };
        temper_core::listed::list_for_each( & allowedFields__565, & ( * fn__10837.clone()));
        return Changeset::new(ChangesetImpl::new(self.0.table_def.clone(), self.0.params.clone(), temper_core::MappedTrait::to_map( & mb__567), self.0.errors.clone(), self.0.is_valid));
    }
    pub fn validate_required(& self, fields__571: impl temper_core::ToList<SafeIdentifier>) -> Changeset {
        let fields__571 = fields__571.to_list();
        let return__306: Changeset;
        let mut t___10830: temper_core::List<ChangesetError>;
        let mut t___6226: TableDef;
        let mut t___6227: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>;
        let mut t___6228: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>;
        'fn__572: {
            if ! self.0.is_valid {
                return__306 = Changeset::new(self.clone());
                break 'fn__572;
            }
            let eb__573: temper_core::ListBuilder<ChangesetError> = temper_core::ListedTrait::to_list_builder( & self.0.errors);
            let mut valid__574: std::sync::Arc<std::sync::RwLock<bool>> = std::sync::Arc::new(std::sync::RwLock::new(true));
            #[derive(Clone)]
            struct ClosureGroup___2 {
                this__180: ChangesetImpl, eb__573: temper_core::ListBuilder<ChangesetError>, valid__574: std::sync::Arc<std::sync::RwLock<bool>>
            }
            impl ClosureGroup___2 {
                fn fn__10826(& self, f__575: SafeIdentifier) {
                    let mut t___10824: ChangesetError;
                    let mut t___10821: std::sync::Arc<String> = f__575.sql_value();
                    if ! temper_core::MappedTrait::has( & self.this__180.0.changes, t___10821.clone()) {
                        t___10824 = ChangesetError::new(f__575.sql_value(), "is required");
                        temper_core::listed::add( & self.eb__573, t___10824.clone(), None);
                        {
                            * self.valid__574.write().unwrap() = false;
                        }
                    }
                }
            }
            let closure_group = ClosureGroup___2 {
                this__180: self.clone(), eb__573: eb__573.clone(), valid__574: valid__574.clone()
            };
            let fn__10826 = {
                let closure_group = closure_group.clone();
                std::sync::Arc::new(move | f__575: SafeIdentifier | closure_group.fn__10826(f__575))
            };
            temper_core::listed::list_for_each( & fields__571, & ( * fn__10826.clone()));
            t___6226 = self.0.table_def.clone();
            t___6227 = self.0.params.clone();
            t___6228 = self.0.changes.clone();
            t___10830 = temper_core::ListedTrait::to_list( & eb__573);
            return__306 = Changeset::new(ChangesetImpl::new(t___6226.clone(), t___6227.clone(), t___6228.clone(), t___10830.clone(), temper_core::read_locked( & valid__574)));
        }
        return return__306.clone();
    }
    pub fn validate_length(& self, field__577: SafeIdentifier, min__578: i32, max__579: i32) -> Changeset {
        let return__307: Changeset;
        let mut t___10808: std::sync::Arc<String>;
        let mut t___10819: temper_core::List<ChangesetError>;
        let mut t___6209: bool;
        let mut t___6215: TableDef;
        let mut t___6216: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>;
        let mut t___6217: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>;
        'fn__580: {
            if ! self.0.is_valid {
                return__307 = Changeset::new(self.clone());
                break 'fn__580;
            }
            t___10808 = field__577.sql_value();
            let val__581: std::sync::Arc<String> = temper_core::MappedTrait::get_or( & self.0.changes, t___10808.clone(), std::sync::Arc::new("".to_string()));
            let len__582: i32 = temper_core::string::count_between( & val__581, 0usize, val__581.len());
            if Some(len__582) < Some(min__578) {
                t___6209 = true;
            } else {
                t___6209 = Some(len__582) > Some(max__579);
            }
            if t___6209 {
                let msg__583: std::sync::Arc<String> = std::sync::Arc::new(format!("must be between {} and {} characters", min__578, max__579));
                let eb__584: temper_core::ListBuilder<ChangesetError> = temper_core::ListedTrait::to_list_builder( & self.0.errors);
                temper_core::listed::add( & eb__584, ChangesetError::new(field__577.sql_value(), msg__583.clone()), None);
                t___6215 = self.0.table_def.clone();
                t___6216 = self.0.params.clone();
                t___6217 = self.0.changes.clone();
                t___10819 = temper_core::ListedTrait::to_list( & eb__584);
                return__307 = Changeset::new(ChangesetImpl::new(t___6215.clone(), t___6216.clone(), t___6217.clone(), t___10819.clone(), false));
                break 'fn__580;
            }
            return__307 = Changeset::new(self.clone());
        }
        return return__307.clone();
    }
    pub fn validate_int(& self, field__586: SafeIdentifier) -> Changeset {
        let return__308: Changeset;
        let mut t___10799: std::sync::Arc<String>;
        let mut t___10806: temper_core::List<ChangesetError>;
        let mut t___6200: TableDef;
        let mut t___6201: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>;
        let mut t___6202: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>;
        'fn__587: {
            if ! self.0.is_valid {
                return__308 = Changeset::new(self.clone());
                break 'fn__587;
            }
            t___10799 = field__586.sql_value();
            let val__588: std::sync::Arc<String> = temper_core::MappedTrait::get_or( & self.0.changes, t___10799.clone(), std::sync::Arc::new("".to_string()));
            if val__588.is_empty() {
                return__308 = Changeset::new(self.clone());
                break 'fn__587;
            }
            let parseOk__589: bool;
            'ok___10942: {
                'orelse___1893: {
                    match temper_core::string::to_int( & val__588, None) {
                        Ok(x) => x,
                        _ => break 'orelse___1893
                    };
                    parseOk__589 = true;
                    break 'ok___10942;
                }
                parseOk__589 = false;
            }
            if ! parseOk__589 {
                let eb__590: temper_core::ListBuilder<ChangesetError> = temper_core::ListedTrait::to_list_builder( & self.0.errors);
                temper_core::listed::add( & eb__590, ChangesetError::new(field__586.sql_value(), "must be an integer"), None);
                t___6200 = self.0.table_def.clone();
                t___6201 = self.0.params.clone();
                t___6202 = self.0.changes.clone();
                t___10806 = temper_core::ListedTrait::to_list( & eb__590);
                return__308 = Changeset::new(ChangesetImpl::new(t___6200.clone(), t___6201.clone(), t___6202.clone(), t___10806.clone(), false));
                break 'fn__587;
            }
            return__308 = Changeset::new(self.clone());
        }
        return return__308.clone();
    }
    pub fn validate_int64(& self, field__592: SafeIdentifier) -> Changeset {
        let return__309: Changeset;
        let mut t___10790: std::sync::Arc<String>;
        let mut t___10797: temper_core::List<ChangesetError>;
        let mut t___6187: TableDef;
        let mut t___6188: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>;
        let mut t___6189: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>;
        'fn__593: {
            if ! self.0.is_valid {
                return__309 = Changeset::new(self.clone());
                break 'fn__593;
            }
            t___10790 = field__592.sql_value();
            let val__594: std::sync::Arc<String> = temper_core::MappedTrait::get_or( & self.0.changes, t___10790.clone(), std::sync::Arc::new("".to_string()));
            if val__594.is_empty() {
                return__309 = Changeset::new(self.clone());
                break 'fn__593;
            }
            let parseOk__595: bool;
            'ok___10944: {
                'orelse___1894: {
                    match temper_core::string::to_int64( & val__594, None) {
                        Ok(x) => x,
                        _ => break 'orelse___1894
                    };
                    parseOk__595 = true;
                    break 'ok___10944;
                }
                parseOk__595 = false;
            }
            if ! parseOk__595 {
                let eb__596: temper_core::ListBuilder<ChangesetError> = temper_core::ListedTrait::to_list_builder( & self.0.errors);
                temper_core::listed::add( & eb__596, ChangesetError::new(field__592.sql_value(), "must be a 64-bit integer"), None);
                t___6187 = self.0.table_def.clone();
                t___6188 = self.0.params.clone();
                t___6189 = self.0.changes.clone();
                t___10797 = temper_core::ListedTrait::to_list( & eb__596);
                return__309 = Changeset::new(ChangesetImpl::new(t___6187.clone(), t___6188.clone(), t___6189.clone(), t___10797.clone(), false));
                break 'fn__593;
            }
            return__309 = Changeset::new(self.clone());
        }
        return return__309.clone();
    }
    pub fn validate_float(& self, field__598: SafeIdentifier) -> Changeset {
        let return__310: Changeset;
        let mut t___10781: std::sync::Arc<String>;
        let mut t___10788: temper_core::List<ChangesetError>;
        let mut t___6174: TableDef;
        let mut t___6175: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>;
        let mut t___6176: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>;
        'fn__599: {
            if ! self.0.is_valid {
                return__310 = Changeset::new(self.clone());
                break 'fn__599;
            }
            t___10781 = field__598.sql_value();
            let val__600: std::sync::Arc<String> = temper_core::MappedTrait::get_or( & self.0.changes, t___10781.clone(), std::sync::Arc::new("".to_string()));
            if val__600.is_empty() {
                return__310 = Changeset::new(self.clone());
                break 'fn__599;
            }
            let parseOk__601: bool;
            'ok___10946: {
                'orelse___1895: {
                    match temper_core::string::to_float64( & val__600) {
                        Ok(x) => x,
                        _ => break 'orelse___1895
                    };
                    parseOk__601 = true;
                    break 'ok___10946;
                }
                parseOk__601 = false;
            }
            if ! parseOk__601 {
                let eb__602: temper_core::ListBuilder<ChangesetError> = temper_core::ListedTrait::to_list_builder( & self.0.errors);
                temper_core::listed::add( & eb__602, ChangesetError::new(field__598.sql_value(), "must be a number"), None);
                t___6174 = self.0.table_def.clone();
                t___6175 = self.0.params.clone();
                t___6176 = self.0.changes.clone();
                t___10788 = temper_core::ListedTrait::to_list( & eb__602);
                return__310 = Changeset::new(ChangesetImpl::new(t___6174.clone(), t___6175.clone(), t___6176.clone(), t___10788.clone(), false));
                break 'fn__599;
            }
            return__310 = Changeset::new(self.clone());
        }
        return return__310.clone();
    }
    pub fn validate_bool(& self, field__604: SafeIdentifier) -> Changeset {
        let return__311: Changeset;
        let mut t___10772: std::sync::Arc<String>;
        let mut t___10779: temper_core::List<ChangesetError>;
        let mut t___6149: bool;
        let mut t___6150: bool;
        let mut t___6152: bool;
        let mut t___6153: bool;
        let mut t___6155: bool;
        let mut t___6161: TableDef;
        let mut t___6162: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>;
        let mut t___6163: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>;
        'fn__605: {
            if ! self.0.is_valid {
                return__311 = Changeset::new(self.clone());
                break 'fn__605;
            }
            t___10772 = field__604.sql_value();
            let val__606: std::sync::Arc<String> = temper_core::MappedTrait::get_or( & self.0.changes, t___10772.clone(), std::sync::Arc::new("".to_string()));
            if val__606.is_empty() {
                return__311 = Changeset::new(self.clone());
                break 'fn__605;
            }
            let isTrue__607: bool;
            if Some(val__606.as_str()) == Some("true") {
                isTrue__607 = true;
            } else {
                if Some(val__606.as_str()) == Some("1") {
                    t___6150 = true;
                } else {
                    if Some(val__606.as_str()) == Some("yes") {
                        t___6149 = true;
                    } else {
                        t___6149 = Some(val__606.as_str()) == Some("on");
                    }
                    t___6150 = t___6149;
                }
                isTrue__607 = t___6150;
            }
            let isFalse__608: bool;
            if Some(val__606.as_str()) == Some("false") {
                isFalse__608 = true;
            } else {
                if Some(val__606.as_str()) == Some("0") {
                    t___6153 = true;
                } else {
                    if Some(val__606.as_str()) == Some("no") {
                        t___6152 = true;
                    } else {
                        t___6152 = Some(val__606.as_str()) == Some("off");
                    }
                    t___6153 = t___6152;
                }
                isFalse__608 = t___6153;
            }
            if ! isTrue__607 {
                t___6155 = ! isFalse__608;
            } else {
                t___6155 = false;
            }
            if t___6155 {
                let eb__609: temper_core::ListBuilder<ChangesetError> = temper_core::ListedTrait::to_list_builder( & self.0.errors);
                temper_core::listed::add( & eb__609, ChangesetError::new(field__604.sql_value(), "must be a boolean (true/false/1/0/yes/no/on/off)"), None);
                t___6161 = self.0.table_def.clone();
                t___6162 = self.0.params.clone();
                t___6163 = self.0.changes.clone();
                t___10779 = temper_core::ListedTrait::to_list( & eb__609);
                return__311 = Changeset::new(ChangesetImpl::new(t___6161.clone(), t___6162.clone(), t___6163.clone(), t___10779.clone(), false));
                break 'fn__605;
            }
            return__311 = Changeset::new(self.clone());
        }
        return return__311.clone();
    }
    fn parse_bool_sql_part(& self, val__611: impl temper_core::ToArcString) -> temper_core::Result<SqlBoolean> {
        let val__611 = val__611.to_arc_string();
        let return__312: SqlBoolean;
        let mut t___6138: bool;
        let mut t___6139: bool;
        let mut t___6140: bool;
        let mut t___6142: bool;
        let mut t___6143: bool;
        let mut t___6144: bool;
        'fn__612: {
            if Some(val__611.as_str()) == Some("true") {
                t___6140 = true;
            } else {
                if Some(val__611.as_str()) == Some("1") {
                    t___6139 = true;
                } else {
                    if Some(val__611.as_str()) == Some("yes") {
                        t___6138 = true;
                    } else {
                        t___6138 = Some(val__611.as_str()) == Some("on");
                    }
                    t___6139 = t___6138;
                }
                t___6140 = t___6139;
            }
            if t___6140 {
                return__312 = SqlBoolean::new(true);
                break 'fn__612;
            }
            if Some(val__611.as_str()) == Some("false") {
                t___6144 = true;
            } else {
                if Some(val__611.as_str()) == Some("0") {
                    t___6143 = true;
                } else {
                    if Some(val__611.as_str()) == Some("no") {
                        t___6142 = true;
                    } else {
                        t___6142 = Some(val__611.as_str()) == Some("off");
                    }
                    t___6143 = t___6142;
                }
                t___6144 = t___6143;
            }
            if t___6144 {
                return__312 = SqlBoolean::new(false);
                break 'fn__612;
            }
            return Err(temper_core::Error::new());
        }
        return Ok(return__312.clone());
    }
    fn value_to_sql_part(& self, fieldDef__614: FieldDef, val__615: impl temper_core::ToArcString) -> temper_core::Result<SqlPart> {
        let val__615 = val__615.to_arc_string();
        let return__313: SqlPart;
        let mut t___6125: i32;
        let mut t___6128: i64;
        let mut t___6131: f64;
        let mut t___6136: temper_std::temporal::Date;
        'fn__616: {
            let ft__617: FieldType = fieldDef__614.field_type();
            if temper_core::is::<StringField>(ft__617.clone()) {
                return__313 = SqlPart::new(SqlString::new(val__615.clone()));
                break 'fn__616;
            }
            if temper_core::is::<IntField>(ft__617.clone()) {
                t___6125 = temper_core::string::to_int( & val__615, None) ? ;
                return__313 = SqlPart::new(SqlInt32::new(t___6125));
                break 'fn__616;
            }
            if temper_core::is::<Int64Field>(ft__617.clone()) {
                t___6128 = temper_core::string::to_int64( & val__615, None) ? ;
                return__313 = SqlPart::new(SqlInt64::new(t___6128));
                break 'fn__616;
            }
            if temper_core::is::<FloatField>(ft__617.clone()) {
                t___6131 = temper_core::string::to_float64( & val__615) ? ;
                return__313 = SqlPart::new(SqlFloat64::new(t___6131));
                break 'fn__616;
            }
            if temper_core::is::<BoolField>(ft__617.clone()) {
                return__313 = SqlPart::new(self.parse_bool_sql_part(val__615.clone()) ? );
                break 'fn__616;
            }
            if temper_core::is::<DateField>(ft__617.clone()) {
                t___6136 = temper_std::temporal::Date::from_iso_string(val__615.clone()) ? ;
                return__313 = SqlPart::new(SqlDate::new(t___6136.clone()));
                break 'fn__616;
            }
            return Err(temper_core::Error::new());
        }
        return Ok(return__313.clone());
    }
    pub fn to_insert_sql(& self) -> temper_core::Result<SqlFragment> {
        let mut t___10720: i32;
        let mut t___10725: std::sync::Arc<String>;
        let mut t___10726: bool;
        let mut t___10731: i32;
        let mut t___10733: std::sync::Arc<String>;
        let mut t___10737: std::sync::Arc<String>;
        let mut t___10752: i32;
        let mut t___6089: bool;
        let mut t___6097: FieldDef;
        let mut t___6102: SqlPart;
        if ! self.0.is_valid {
            return Err(temper_core::Error::new());
        }
        let mut i__620: i32 = 0;
        'loop___11021: loop {
            t___10720 = temper_core::ListedTrait::len( & self.0.table_def.fields());
            if ! (Some(i__620) < Some(t___10720)) {
                break;
            }
            let f__621: FieldDef = temper_core::ListedTrait::get( & self.0.table_def.fields(), i__620);
            if ! f__621.nullable() {
                t___10725 = f__621.name().sql_value();
                t___10726 = temper_core::MappedTrait::has( & self.0.changes, t___10725.clone());
                t___6089 = ! t___10726;
            } else {
                t___6089 = false;
            }
            if t___6089 {
                return Err(temper_core::Error::new());
            }
            i__620 = i__620.wrapping_add(1);
        }
        let pairs__622: temper_core::List<(std::sync::Arc<String>, std::sync::Arc<String>)> = temper_core::MappedTrait::to_list( & self.0.changes);
        if Some(temper_core::ListedTrait::len( & pairs__622)) == Some(0) {
            return Err(temper_core::Error::new());
        }
        let colNames__623: temper_core::ListBuilder<std::sync::Arc<String>> = temper_core::listed::new_builder();
        let valParts__624: temper_core::ListBuilder<SqlPart> = temper_core::listed::new_builder();
        let mut i__625: i32 = 0;
        'loop___11022: loop {
            t___10731 = temper_core::ListedTrait::len( & pairs__622);
            if ! (Some(i__625) < Some(t___10731)) {
                break;
            }
            let pair__626: (std::sync::Arc<String>, std::sync::Arc<String>) = temper_core::ListedTrait::get( & pairs__622, i__625);
            t___10733 = pair__626.key();
            t___6097 = self.0.table_def.field(t___10733.clone()) ? ;
            let fd__627: FieldDef = t___6097.clone();
            temper_core::listed::add( & colNames__623, fd__627.name().sql_value(), None);
            t___10737 = pair__626.value();
            t___6102 = self.value_to_sql_part(fd__627.clone(), t___10737.clone()) ? ;
            temper_core::listed::add( & valParts__624, t___6102.clone(), None);
            i__625 = i__625.wrapping_add(1);
        }
        let b__628: SqlBuilder = SqlBuilder::new();
        b__628.append_safe("INSERT INTO ");
        b__628.append_safe(self.0.table_def.table_name().sql_value());
        b__628.append_safe(" (");
        let mut t___10745: temper_core::List<std::sync::Arc<String>> = temper_core::ListedTrait::to_list( & colNames__623);
        #[derive(Clone)]
        struct ClosureGroup___3 {}
        impl ClosureGroup___3 {
            fn fn__10718(& self, c__629: impl temper_core::ToArcString) -> std::sync::Arc<String> {
                let c__629 = c__629.to_arc_string();
                return c__629.clone();
            }
        }
        let closure_group = ClosureGroup___3 {};
        let fn__10718 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | c__629: std::sync::Arc<String> | closure_group.fn__10718(c__629))
        };
        b__628.append_safe(temper_core::listed::join( & t___10745, std::sync::Arc::new(", ".to_string()), & ( * fn__10718.clone())));
        b__628.append_safe(") VALUES (");
        b__628.append_part(temper_core::ListedTrait::get( & valParts__624, 0));
        let mut j__630: i32 = 1;
        'loop___11023: loop {
            t___10752 = temper_core::ListedTrait::len( & valParts__624);
            if ! (Some(j__630) < Some(t___10752)) {
                break;
            }
            b__628.append_safe(", ");
            b__628.append_part(temper_core::ListedTrait::get( & valParts__624, j__630));
            j__630 = j__630.wrapping_add(1);
        }
        b__628.append_safe(")");
        return Ok(b__628.accumulated());
    }
    pub fn to_update_sql(& self, id__632: i32) -> temper_core::Result<SqlFragment> {
        let mut t___10705: i32;
        let mut t___10708: std::sync::Arc<String>;
        let mut t___10713: std::sync::Arc<String>;
        let mut t___6070: FieldDef;
        let mut t___6076: SqlPart;
        if ! self.0.is_valid {
            return Err(temper_core::Error::new());
        }
        let pairs__634: temper_core::List<(std::sync::Arc<String>, std::sync::Arc<String>)> = temper_core::MappedTrait::to_list( & self.0.changes);
        if Some(temper_core::ListedTrait::len( & pairs__634)) == Some(0) {
            return Err(temper_core::Error::new());
        }
        let b__635: SqlBuilder = SqlBuilder::new();
        b__635.append_safe("UPDATE ");
        b__635.append_safe(self.0.table_def.table_name().sql_value());
        b__635.append_safe(" SET ");
        let mut i__636: i32 = 0;
        'loop___11024: loop {
            t___10705 = temper_core::ListedTrait::len( & pairs__634);
            if ! (Some(i__636) < Some(t___10705)) {
                break;
            }
            if Some(i__636) > Some(0) {
                b__635.append_safe(", ");
            }
            let pair__637: (std::sync::Arc<String>, std::sync::Arc<String>) = temper_core::ListedTrait::get( & pairs__634, i__636);
            t___10708 = pair__637.key();
            t___6070 = self.0.table_def.field(t___10708.clone()) ? ;
            let fd__638: FieldDef = t___6070.clone();
            b__635.append_safe(fd__638.name().sql_value());
            b__635.append_safe(" = ");
            t___10713 = pair__637.value();
            t___6076 = self.value_to_sql_part(fd__638.clone(), t___10713.clone()) ? ;
            b__635.append_part(t___6076.clone());
            i__636 = i__636.wrapping_add(1);
        }
        b__635.append_safe(" WHERE id = ");
        b__635.append_int32(id__632);
        return Ok(b__635.accumulated());
    }
    pub fn new(_tableDef__640: TableDef, _params__641: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>, _changes__642: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>, _errors__643: impl temper_core::ToList<ChangesetError>, _isValid__644: bool) -> ChangesetImpl {
        let _errors__643 = _errors__643.to_list();
        let table_def;
        let params;
        let changes;
        let errors;
        let is_valid;
        table_def = _tableDef__640.clone();
        params = _params__641.clone();
        changes = _changes__642.clone();
        errors = _errors__643.clone();
        is_valid = _isValid__644;
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
    fn cast(& self, allowedFields__565: temper_core::List<SafeIdentifier>) -> Changeset {
        self.cast(allowedFields__565)
    }
    fn validate_required(& self, fields__571: temper_core::List<SafeIdentifier>) -> Changeset {
        self.validate_required(fields__571)
    }
    fn validate_length(& self, field__577: SafeIdentifier, min__578: i32, max__579: i32) -> Changeset {
        self.validate_length(field__577, min__578, max__579)
    }
    fn validate_int(& self, field__586: SafeIdentifier) -> Changeset {
        self.validate_int(field__586)
    }
    fn validate_int64(& self, field__592: SafeIdentifier) -> Changeset {
        self.validate_int64(field__592)
    }
    fn validate_float(& self, field__598: SafeIdentifier) -> Changeset {
        self.validate_float(field__598)
    }
    fn validate_bool(& self, field__604: SafeIdentifier) -> Changeset {
        self.validate_bool(field__604)
    }
    fn to_insert_sql(& self) -> temper_core::Result<SqlFragment> {
        self.to_insert_sql()
    }
    fn to_update_sql(& self, id__632: i32) -> temper_core::Result<SqlFragment> {
        self.to_update_sql(id__632)
    }
}
temper_core::impl_any_value_trait!(ChangesetImpl, [Changeset]);
pub enum JoinTypeEnum {
    InnerJoin(InnerJoin), LeftJoin(LeftJoin), RightJoin(RightJoin), FullJoin(FullJoin)
}
pub trait JoinTypeTrait: temper_core::AsAnyValue + temper_core::AnyValueTrait + std::marker::Send + std::marker::Sync {
    fn as_enum(& self) -> JoinTypeEnum;
    fn clone_boxed(& self) -> JoinType;
    fn keyword(& self) -> std::sync::Arc<String>;
}
#[derive(Clone)]
pub struct JoinType(std::sync::Arc<dyn JoinTypeTrait>);
impl JoinType {
    pub fn new(selfish: impl JoinTypeTrait + 'static) -> JoinType {
        JoinType(std::sync::Arc::new(selfish))
    }
}
impl JoinTypeTrait for JoinType {
    fn as_enum(& self) -> JoinTypeEnum {
        JoinTypeTrait::as_enum( & ( * self.0))
    }
    fn clone_boxed(& self) -> JoinType {
        JoinTypeTrait::clone_boxed( & ( * self.0))
    }
    fn keyword(& self) -> std::sync::Arc<String> {
        JoinTypeTrait::keyword( & ( * self.0))
    }
}
temper_core::impl_any_value_trait_for_interface!(JoinType);
impl std::ops::Deref for JoinType {
    type Target = dyn JoinTypeTrait;
    fn deref(& self) -> & Self::Target {
        & ( * self.0)
    }
}
struct InnerJoinStruct {}
#[derive(Clone)]
pub struct InnerJoin(std::sync::Arc<InnerJoinStruct>);
impl InnerJoin {
    pub fn keyword(& self) -> std::sync::Arc<String> {
        return std::sync::Arc::new("INNER JOIN".to_string());
    }
    pub fn new() -> InnerJoin {
        let selfish = InnerJoin(std::sync::Arc::new(InnerJoinStruct {}));
        return selfish;
    }
}
impl JoinTypeTrait for InnerJoin {
    fn as_enum(& self) -> JoinTypeEnum {
        JoinTypeEnum::InnerJoin(self.clone())
    }
    fn clone_boxed(& self) -> JoinType {
        JoinType::new(self.clone())
    }
    fn keyword(& self) -> std::sync::Arc<String> {
        self.keyword()
    }
}
temper_core::impl_any_value_trait!(InnerJoin, [JoinType]);
struct LeftJoinStruct {}
#[derive(Clone)]
pub struct LeftJoin(std::sync::Arc<LeftJoinStruct>);
impl LeftJoin {
    pub fn keyword(& self) -> std::sync::Arc<String> {
        return std::sync::Arc::new("LEFT JOIN".to_string());
    }
    pub fn new() -> LeftJoin {
        let selfish = LeftJoin(std::sync::Arc::new(LeftJoinStruct {}));
        return selfish;
    }
}
impl JoinTypeTrait for LeftJoin {
    fn as_enum(& self) -> JoinTypeEnum {
        JoinTypeEnum::LeftJoin(self.clone())
    }
    fn clone_boxed(& self) -> JoinType {
        JoinType::new(self.clone())
    }
    fn keyword(& self) -> std::sync::Arc<String> {
        self.keyword()
    }
}
temper_core::impl_any_value_trait!(LeftJoin, [JoinType]);
struct RightJoinStruct {}
#[derive(Clone)]
pub struct RightJoin(std::sync::Arc<RightJoinStruct>);
impl RightJoin {
    pub fn keyword(& self) -> std::sync::Arc<String> {
        return std::sync::Arc::new("RIGHT JOIN".to_string());
    }
    pub fn new() -> RightJoin {
        let selfish = RightJoin(std::sync::Arc::new(RightJoinStruct {}));
        return selfish;
    }
}
impl JoinTypeTrait for RightJoin {
    fn as_enum(& self) -> JoinTypeEnum {
        JoinTypeEnum::RightJoin(self.clone())
    }
    fn clone_boxed(& self) -> JoinType {
        JoinType::new(self.clone())
    }
    fn keyword(& self) -> std::sync::Arc<String> {
        self.keyword()
    }
}
temper_core::impl_any_value_trait!(RightJoin, [JoinType]);
struct FullJoinStruct {}
#[derive(Clone)]
pub struct FullJoin(std::sync::Arc<FullJoinStruct>);
impl FullJoin {
    pub fn keyword(& self) -> std::sync::Arc<String> {
        return std::sync::Arc::new("FULL OUTER JOIN".to_string());
    }
    pub fn new() -> FullJoin {
        let selfish = FullJoin(std::sync::Arc::new(FullJoinStruct {}));
        return selfish;
    }
}
impl JoinTypeTrait for FullJoin {
    fn as_enum(& self) -> JoinTypeEnum {
        JoinTypeEnum::FullJoin(self.clone())
    }
    fn clone_boxed(& self) -> JoinType {
        JoinType::new(self.clone())
    }
    fn keyword(& self) -> std::sync::Arc<String> {
        self.keyword()
    }
}
temper_core::impl_any_value_trait!(FullJoin, [JoinType]);
struct JoinClauseStruct {
    join_type: JoinType, table: SafeIdentifier, on_condition: SqlFragment
}
#[derive(Clone)]
pub struct JoinClause(std::sync::Arc<JoinClauseStruct>);
#[derive(Clone)]
pub struct JoinClauseBuilder {
    pub join_type: JoinType, pub table: SafeIdentifier, pub on_condition: SqlFragment
}
impl JoinClauseBuilder {
    pub fn build(self) -> JoinClause {
        JoinClause::new(self.join_type, self.table, self.on_condition)
    }
}
impl JoinClause {
    pub fn new(joinType__757: JoinType, table__758: SafeIdentifier, onCondition__759: SqlFragment) -> JoinClause {
        let join_type;
        let table;
        let on_condition;
        join_type = joinType__757.clone();
        table = table__758.clone();
        on_condition = onCondition__759.clone();
        let selfish = JoinClause(std::sync::Arc::new(JoinClauseStruct {
                    join_type, table, on_condition
        }));
        return selfish;
    }
    pub fn join_type(& self) -> JoinType {
        return self.0.join_type.clone();
    }
    pub fn table(& self) -> SafeIdentifier {
        return self.0.table.clone();
    }
    pub fn on_condition(& self) -> SqlFragment {
        return self.0.on_condition.clone();
    }
}
temper_core::impl_any_value_trait!(JoinClause, []);
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
    pub fn new(field__763: SafeIdentifier, ascending__764: bool) -> OrderClause {
        let field;
        let ascending;
        field = field__763.clone();
        ascending = ascending__764;
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
pub enum WhereClauseEnum {
    AndCondition(AndCondition), OrCondition(OrCondition)
}
pub trait WhereClauseTrait: temper_core::AsAnyValue + temper_core::AnyValueTrait + std::marker::Send + std::marker::Sync {
    fn as_enum(& self) -> WhereClauseEnum;
    fn clone_boxed(& self) -> WhereClause;
    fn condition(& self) -> SqlFragment;
    fn keyword(& self) -> std::sync::Arc<String>;
}
#[derive(Clone)]
pub struct WhereClause(std::sync::Arc<dyn WhereClauseTrait>);
impl WhereClause {
    pub fn new(selfish: impl WhereClauseTrait + 'static) -> WhereClause {
        WhereClause(std::sync::Arc::new(selfish))
    }
}
impl WhereClauseTrait for WhereClause {
    fn as_enum(& self) -> WhereClauseEnum {
        WhereClauseTrait::as_enum( & ( * self.0))
    }
    fn clone_boxed(& self) -> WhereClause {
        WhereClauseTrait::clone_boxed( & ( * self.0))
    }
    fn condition(& self) -> SqlFragment {
        WhereClauseTrait::condition( & ( * self.0))
    }
    fn keyword(& self) -> std::sync::Arc<String> {
        WhereClauseTrait::keyword( & ( * self.0))
    }
}
temper_core::impl_any_value_trait_for_interface!(WhereClause);
impl std::ops::Deref for WhereClause {
    type Target = dyn WhereClauseTrait;
    fn deref(& self) -> & Self::Target {
        & ( * self.0)
    }
}
struct AndConditionStruct {
    condition: SqlFragment
}
#[derive(Clone)]
pub struct AndCondition(std::sync::Arc<AndConditionStruct>);
impl AndCondition {
    pub fn condition(& self) -> SqlFragment {
        return self.0.condition.clone();
    }
    pub fn keyword(& self) -> std::sync::Arc<String> {
        return std::sync::Arc::new("AND".to_string());
    }
    pub fn new(_condition__775: SqlFragment) -> AndCondition {
        let condition;
        condition = _condition__775.clone();
        let selfish = AndCondition(std::sync::Arc::new(AndConditionStruct {
                    condition
        }));
        return selfish;
    }
}
impl WhereClauseTrait for AndCondition {
    fn as_enum(& self) -> WhereClauseEnum {
        WhereClauseEnum::AndCondition(self.clone())
    }
    fn clone_boxed(& self) -> WhereClause {
        WhereClause::new(self.clone())
    }
    fn condition(& self) -> SqlFragment {
        self.condition()
    }
    fn keyword(& self) -> std::sync::Arc<String> {
        self.keyword()
    }
}
temper_core::impl_any_value_trait!(AndCondition, [WhereClause]);
struct OrConditionStruct {
    condition: SqlFragment
}
#[derive(Clone)]
pub struct OrCondition(std::sync::Arc<OrConditionStruct>);
impl OrCondition {
    pub fn condition(& self) -> SqlFragment {
        return self.0.condition.clone();
    }
    pub fn keyword(& self) -> std::sync::Arc<String> {
        return std::sync::Arc::new("OR".to_string());
    }
    pub fn new(_condition__782: SqlFragment) -> OrCondition {
        let condition;
        condition = _condition__782.clone();
        let selfish = OrCondition(std::sync::Arc::new(OrConditionStruct {
                    condition
        }));
        return selfish;
    }
}
impl WhereClauseTrait for OrCondition {
    fn as_enum(& self) -> WhereClauseEnum {
        WhereClauseEnum::OrCondition(self.clone())
    }
    fn clone_boxed(& self) -> WhereClause {
        WhereClause::new(self.clone())
    }
    fn condition(& self) -> SqlFragment {
        self.condition()
    }
    fn keyword(& self) -> std::sync::Arc<String> {
        self.keyword()
    }
}
temper_core::impl_any_value_trait!(OrCondition, [WhereClause]);
struct QueryStruct {
    table_name: SafeIdentifier, conditions: temper_core::List<WhereClause>, selected_fields: temper_core::List<SafeIdentifier>, order_clauses: temper_core::List<OrderClause>, limit_val: Option<i32>, offset_val: Option<i32>, join_clauses: temper_core::List<JoinClause>, group_by_fields: temper_core::List<SafeIdentifier>, having_conditions: temper_core::List<WhereClause>, is_distinct: bool, select_exprs: temper_core::List<SqlFragment>
}
#[derive(Clone)]
pub struct Query(std::sync::Arc<QueryStruct>);
#[derive(Clone)]
pub struct QueryBuilder {
    pub table_name: SafeIdentifier, pub conditions: temper_core::List<WhereClause>, pub selected_fields: temper_core::List<SafeIdentifier>, pub order_clauses: temper_core::List<OrderClause>, pub limit_val: Option<i32>, pub offset_val: Option<i32>, pub join_clauses: temper_core::List<JoinClause>, pub group_by_fields: temper_core::List<SafeIdentifier>, pub having_conditions: temper_core::List<WhereClause>, pub is_distinct: bool, pub select_exprs: temper_core::List<SqlFragment>
}
impl QueryBuilder {
    pub fn build(self) -> Query {
        Query::new(self.table_name, self.conditions, self.selected_fields, self.order_clauses, self.limit_val, self.offset_val, self.join_clauses, self.group_by_fields, self.having_conditions, self.is_distinct, self.select_exprs)
    }
}
impl Query {
    pub fn r#where(& self, condition__795: SqlFragment) -> Query {
        let nb__797: temper_core::ListBuilder<WhereClause> = temper_core::ListedTrait::to_list_builder( & self.0.conditions);
        temper_core::listed::add( & nb__797, WhereClause::new(AndCondition::new(condition__795.clone())), None);
        return Query::new(self.0.table_name.clone(), temper_core::ListedTrait::to_list( & nb__797), self.0.selected_fields.clone(), self.0.order_clauses.clone(), self.0.limit_val, self.0.offset_val, self.0.join_clauses.clone(), self.0.group_by_fields.clone(), self.0.having_conditions.clone(), self.0.is_distinct, self.0.select_exprs.clone());
    }
    pub fn or_where(& self, condition__799: SqlFragment) -> Query {
        let nb__801: temper_core::ListBuilder<WhereClause> = temper_core::ListedTrait::to_list_builder( & self.0.conditions);
        temper_core::listed::add( & nb__801, WhereClause::new(OrCondition::new(condition__799.clone())), None);
        return Query::new(self.0.table_name.clone(), temper_core::ListedTrait::to_list( & nb__801), self.0.selected_fields.clone(), self.0.order_clauses.clone(), self.0.limit_val, self.0.offset_val, self.0.join_clauses.clone(), self.0.group_by_fields.clone(), self.0.having_conditions.clone(), self.0.is_distinct, self.0.select_exprs.clone());
    }
    pub fn where_null(& self, field__803: SafeIdentifier) -> Query {
        let b__805: SqlBuilder = SqlBuilder::new();
        b__805.append_safe(field__803.sql_value());
        b__805.append_safe(" IS NULL");
        let mut t___10304: SqlFragment = b__805.accumulated();
        return self.r#where(t___10304.clone());
    }
    pub fn where_not_null(& self, field__807: SafeIdentifier) -> Query {
        let b__809: SqlBuilder = SqlBuilder::new();
        b__809.append_safe(field__807.sql_value());
        b__809.append_safe(" IS NOT NULL");
        let mut t___10298: SqlFragment = b__809.accumulated();
        return self.r#where(t___10298.clone());
    }
    pub fn where_in(& self, field__811: SafeIdentifier, values__812: impl temper_core::ToList<SqlPart>) -> Query {
        let values__812 = values__812.to_list();
        let return__365: Query;
        let mut t___10279: SqlFragment;
        let mut t___10287: i32;
        let mut t___10292: SqlFragment;
        'fn__813: {
            if temper_core::ListedTrait::is_empty( & values__812) {
                let b__814: SqlBuilder = SqlBuilder::new();
                b__814.append_safe("1 = 0");
                t___10279 = b__814.accumulated();
                return__365 = self.r#where(t___10279.clone());
                break 'fn__813;
            }
            let b__815: SqlBuilder = SqlBuilder::new();
            b__815.append_safe(field__811.sql_value());
            b__815.append_safe(" IN (");
            b__815.append_part(temper_core::ListedTrait::get( & values__812, 0));
            let mut i__816: i32 = 1;
            'loop___11029: loop {
                t___10287 = temper_core::ListedTrait::len( & values__812);
                if ! (Some(i__816) < Some(t___10287)) {
                    break;
                }
                b__815.append_safe(", ");
                b__815.append_part(temper_core::ListedTrait::get( & values__812, i__816));
                i__816 = i__816.wrapping_add(1);
            }
            b__815.append_safe(")");
            t___10292 = b__815.accumulated();
            return__365 = self.r#where(t___10292.clone());
        }
        return return__365.clone();
    }
    pub fn where_in_subquery(& self, field__818: SafeIdentifier, sub__819: Query) -> Query {
        let b__821: SqlBuilder = SqlBuilder::new();
        b__821.append_safe(field__818.sql_value());
        b__821.append_safe(" IN (");
        b__821.append_fragment(sub__819.to_sql());
        b__821.append_safe(")");
        let mut t___10274: SqlFragment = b__821.accumulated();
        return self.r#where(t___10274.clone());
    }
    pub fn where_not(& self, condition__823: SqlFragment) -> Query {
        let b__825: SqlBuilder = SqlBuilder::new();
        b__825.append_safe("NOT (");
        b__825.append_fragment(condition__823.clone());
        b__825.append_safe(")");
        let mut t___10265: SqlFragment = b__825.accumulated();
        return self.r#where(t___10265.clone());
    }
    pub fn where_between(& self, field__827: SafeIdentifier, low__828: SqlPart, high__829: SqlPart) -> Query {
        let b__831: SqlBuilder = SqlBuilder::new();
        b__831.append_safe(field__827.sql_value());
        b__831.append_safe(" BETWEEN ");
        b__831.append_part(low__828.clone());
        b__831.append_safe(" AND ");
        b__831.append_part(high__829.clone());
        let mut t___10259: SqlFragment = b__831.accumulated();
        return self.r#where(t___10259.clone());
    }
    pub fn where_like(& self, field__833: SafeIdentifier, pattern__834: impl temper_core::ToArcString) -> Query {
        let pattern__834 = pattern__834.to_arc_string();
        let b__836: SqlBuilder = SqlBuilder::new();
        b__836.append_safe(field__833.sql_value());
        b__836.append_safe(" LIKE ");
        b__836.append_string(pattern__834.clone());
        let mut t___10250: SqlFragment = b__836.accumulated();
        return self.r#where(t___10250.clone());
    }
    pub fn where_i_like(& self, field__838: SafeIdentifier, pattern__839: impl temper_core::ToArcString) -> Query {
        let pattern__839 = pattern__839.to_arc_string();
        let b__841: SqlBuilder = SqlBuilder::new();
        b__841.append_safe(field__838.sql_value());
        b__841.append_safe(" ILIKE ");
        b__841.append_string(pattern__839.clone());
        let mut t___10243: SqlFragment = b__841.accumulated();
        return self.r#where(t___10243.clone());
    }
    pub fn select(& self, fields__843: impl temper_core::ToList<SafeIdentifier>) -> Query {
        let fields__843 = fields__843.to_list();
        return Query::new(self.0.table_name.clone(), self.0.conditions.clone(), fields__843.clone(), self.0.order_clauses.clone(), self.0.limit_val, self.0.offset_val, self.0.join_clauses.clone(), self.0.group_by_fields.clone(), self.0.having_conditions.clone(), self.0.is_distinct, self.0.select_exprs.clone());
    }
    pub fn select_expr(& self, exprs__846: impl temper_core::ToList<SqlFragment>) -> Query {
        let exprs__846 = exprs__846.to_list();
        return Query::new(self.0.table_name.clone(), self.0.conditions.clone(), self.0.selected_fields.clone(), self.0.order_clauses.clone(), self.0.limit_val, self.0.offset_val, self.0.join_clauses.clone(), self.0.group_by_fields.clone(), self.0.having_conditions.clone(), self.0.is_distinct, exprs__846.clone());
    }
    pub fn order_by(& self, field__849: SafeIdentifier, ascending__850: bool) -> Query {
        let nb__852: temper_core::ListBuilder<OrderClause> = temper_core::ListedTrait::to_list_builder( & self.0.order_clauses);
        temper_core::listed::add( & nb__852, OrderClause::new(field__849.clone(), ascending__850), None);
        return Query::new(self.0.table_name.clone(), self.0.conditions.clone(), self.0.selected_fields.clone(), temper_core::ListedTrait::to_list( & nb__852), self.0.limit_val, self.0.offset_val, self.0.join_clauses.clone(), self.0.group_by_fields.clone(), self.0.having_conditions.clone(), self.0.is_distinct, self.0.select_exprs.clone());
    }
    pub fn limit(& self, n__854: i32) -> temper_core::Result<Query> {
        if Some(n__854) < Some(0) {
            return Err(temper_core::Error::new());
        }
        return Ok(Query::new(self.0.table_name.clone(), self.0.conditions.clone(), self.0.selected_fields.clone(), self.0.order_clauses.clone(), Some(n__854), self.0.offset_val, self.0.join_clauses.clone(), self.0.group_by_fields.clone(), self.0.having_conditions.clone(), self.0.is_distinct, self.0.select_exprs.clone()));
    }
    pub fn offset(& self, n__857: i32) -> temper_core::Result<Query> {
        if Some(n__857) < Some(0) {
            return Err(temper_core::Error::new());
        }
        return Ok(Query::new(self.0.table_name.clone(), self.0.conditions.clone(), self.0.selected_fields.clone(), self.0.order_clauses.clone(), self.0.limit_val, Some(n__857), self.0.join_clauses.clone(), self.0.group_by_fields.clone(), self.0.having_conditions.clone(), self.0.is_distinct, self.0.select_exprs.clone()));
    }
    pub fn join(& self, joinType__860: JoinType, table__861: SafeIdentifier, onCondition__862: SqlFragment) -> Query {
        let nb__864: temper_core::ListBuilder<JoinClause> = temper_core::ListedTrait::to_list_builder( & self.0.join_clauses);
        temper_core::listed::add( & nb__864, JoinClause::new(joinType__860.clone(), table__861.clone(), onCondition__862.clone()), None);
        return Query::new(self.0.table_name.clone(), self.0.conditions.clone(), self.0.selected_fields.clone(), self.0.order_clauses.clone(), self.0.limit_val, self.0.offset_val, temper_core::ListedTrait::to_list( & nb__864), self.0.group_by_fields.clone(), self.0.having_conditions.clone(), self.0.is_distinct, self.0.select_exprs.clone());
    }
    pub fn inner_join(& self, table__866: SafeIdentifier, onCondition__867: SqlFragment) -> Query {
        let mut t___10213: InnerJoin = InnerJoin::new();
        return self.join(JoinType::new(t___10213.clone()), table__866.clone(), onCondition__867.clone());
    }
    pub fn left_join(& self, table__870: SafeIdentifier, onCondition__871: SqlFragment) -> Query {
        let mut t___10211: LeftJoin = LeftJoin::new();
        return self.join(JoinType::new(t___10211.clone()), table__870.clone(), onCondition__871.clone());
    }
    pub fn right_join(& self, table__874: SafeIdentifier, onCondition__875: SqlFragment) -> Query {
        let mut t___10209: RightJoin = RightJoin::new();
        return self.join(JoinType::new(t___10209.clone()), table__874.clone(), onCondition__875.clone());
    }
    pub fn full_join(& self, table__878: SafeIdentifier, onCondition__879: SqlFragment) -> Query {
        let mut t___10207: FullJoin = FullJoin::new();
        return self.join(JoinType::new(t___10207.clone()), table__878.clone(), onCondition__879.clone());
    }
    pub fn group_by(& self, field__882: SafeIdentifier) -> Query {
        let nb__884: temper_core::ListBuilder<SafeIdentifier> = temper_core::ListedTrait::to_list_builder( & self.0.group_by_fields);
        temper_core::listed::add( & nb__884, field__882.clone(), None);
        return Query::new(self.0.table_name.clone(), self.0.conditions.clone(), self.0.selected_fields.clone(), self.0.order_clauses.clone(), self.0.limit_val, self.0.offset_val, self.0.join_clauses.clone(), temper_core::ListedTrait::to_list( & nb__884), self.0.having_conditions.clone(), self.0.is_distinct, self.0.select_exprs.clone());
    }
    pub fn having(& self, condition__886: SqlFragment) -> Query {
        let nb__888: temper_core::ListBuilder<WhereClause> = temper_core::ListedTrait::to_list_builder( & self.0.having_conditions);
        temper_core::listed::add( & nb__888, WhereClause::new(AndCondition::new(condition__886.clone())), None);
        return Query::new(self.0.table_name.clone(), self.0.conditions.clone(), self.0.selected_fields.clone(), self.0.order_clauses.clone(), self.0.limit_val, self.0.offset_val, self.0.join_clauses.clone(), self.0.group_by_fields.clone(), temper_core::ListedTrait::to_list( & nb__888), self.0.is_distinct, self.0.select_exprs.clone());
    }
    pub fn or_having(& self, condition__890: SqlFragment) -> Query {
        let nb__892: temper_core::ListBuilder<WhereClause> = temper_core::ListedTrait::to_list_builder( & self.0.having_conditions);
        temper_core::listed::add( & nb__892, WhereClause::new(OrCondition::new(condition__890.clone())), None);
        return Query::new(self.0.table_name.clone(), self.0.conditions.clone(), self.0.selected_fields.clone(), self.0.order_clauses.clone(), self.0.limit_val, self.0.offset_val, self.0.join_clauses.clone(), self.0.group_by_fields.clone(), temper_core::ListedTrait::to_list( & nb__892), self.0.is_distinct, self.0.select_exprs.clone());
    }
    pub fn distinct(& self) -> Query {
        return Query::new(self.0.table_name.clone(), self.0.conditions.clone(), self.0.selected_fields.clone(), self.0.order_clauses.clone(), self.0.limit_val, self.0.offset_val, self.0.join_clauses.clone(), self.0.group_by_fields.clone(), self.0.having_conditions.clone(), true, self.0.select_exprs.clone());
    }
    pub fn to_sql(& self) -> SqlFragment {
        let mut t___10113: i32;
        let mut t___10132: i32;
        let mut t___10151: i32;
        let b__897: SqlBuilder = SqlBuilder::new();
        if self.0.is_distinct {
            b__897.append_safe("SELECT DISTINCT ");
        } else {
            b__897.append_safe("SELECT ");
        }
        if ! temper_core::ListedTrait::is_empty( & self.0.select_exprs) {
            b__897.append_fragment(temper_core::ListedTrait::get( & self.0.select_exprs, 0));
            let mut i__898: i32 = 1;
            'loop___11048: loop {
                t___10113 = temper_core::ListedTrait::len( & self.0.select_exprs);
                if ! (Some(i__898) < Some(t___10113)) {
                    break;
                }
                b__897.append_safe(", ");
                b__897.append_fragment(temper_core::ListedTrait::get( & self.0.select_exprs, i__898));
                i__898 = i__898.wrapping_add(1);
            }
        } else {
            if temper_core::ListedTrait::is_empty( & self.0.selected_fields) {
                b__897.append_safe("*");
            } else {
                #[derive(Clone)]
                struct ClosureGroup___4 {}
                impl ClosureGroup___4 {
                    fn fn__10106(& self, f__899: SafeIdentifier) -> std::sync::Arc<String> {
                        return f__899.sql_value();
                    }
                }
                let closure_group = ClosureGroup___4 {};
                let fn__10106 = {
                    let closure_group = closure_group.clone();
                    std::sync::Arc::new(move | f__899: SafeIdentifier | closure_group.fn__10106(f__899))
                };
                b__897.append_safe(temper_core::listed::join( & self.0.selected_fields, std::sync::Arc::new(", ".to_string()), & ( * fn__10106.clone())));
            }
        }
        b__897.append_safe(" FROM ");
        b__897.append_safe(self.0.table_name.sql_value());
        #[derive(Clone)]
        struct ClosureGroup___5 {
            b__897: SqlBuilder
        }
        impl ClosureGroup___5 {
            fn fn__10105(& self, jc__900: JoinClause) {
                self.b__897.append_safe(" ");
                let mut t___10093: std::sync::Arc<String> = jc__900.join_type().keyword();
                self.b__897.append_safe(t___10093.clone());
                self.b__897.append_safe(" ");
                let mut t___10097: std::sync::Arc<String> = jc__900.table().sql_value();
                self.b__897.append_safe(t___10097.clone());
                self.b__897.append_safe(" ON ");
                let mut t___10100: SqlFragment = jc__900.on_condition();
                self.b__897.append_fragment(t___10100.clone());
            }
        }
        let closure_group = ClosureGroup___5 {
            b__897: b__897.clone()
        };
        let fn__10105 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | jc__900: JoinClause | closure_group.fn__10105(jc__900))
        };
        temper_core::listed::list_for_each( & self.0.join_clauses, & ( * fn__10105.clone()));
        if ! temper_core::ListedTrait::is_empty( & self.0.conditions) {
            b__897.append_safe(" WHERE ");
            b__897.append_fragment(temper_core::ListedTrait::get( & self.0.conditions, 0).condition());
            let mut i__901: i32 = 1;
            'loop___11049: loop {
                t___10132 = temper_core::ListedTrait::len( & self.0.conditions);
                if ! (Some(i__901) < Some(t___10132)) {
                    break;
                }
                b__897.append_safe(" ");
                b__897.append_safe(temper_core::ListedTrait::get( & self.0.conditions, i__901).keyword());
                b__897.append_safe(" ");
                b__897.append_fragment(temper_core::ListedTrait::get( & self.0.conditions, i__901).condition());
                i__901 = i__901.wrapping_add(1);
            }
        }
        if ! temper_core::ListedTrait::is_empty( & self.0.group_by_fields) {
            b__897.append_safe(" GROUP BY ");
            #[derive(Clone)]
            struct ClosureGroup___6 {}
            impl ClosureGroup___6 {
                fn fn__10104(& self, f__902: SafeIdentifier) -> std::sync::Arc<String> {
                    return f__902.sql_value();
                }
            }
            let closure_group = ClosureGroup___6 {};
            let fn__10104 = {
                let closure_group = closure_group.clone();
                std::sync::Arc::new(move | f__902: SafeIdentifier | closure_group.fn__10104(f__902))
            };
            b__897.append_safe(temper_core::listed::join( & self.0.group_by_fields, std::sync::Arc::new(", ".to_string()), & ( * fn__10104.clone())));
        }
        if ! temper_core::ListedTrait::is_empty( & self.0.having_conditions) {
            b__897.append_safe(" HAVING ");
            b__897.append_fragment(temper_core::ListedTrait::get( & self.0.having_conditions, 0).condition());
            let mut i__903: i32 = 1;
            'loop___11050: loop {
                t___10151 = temper_core::ListedTrait::len( & self.0.having_conditions);
                if ! (Some(i__903) < Some(t___10151)) {
                    break;
                }
                b__897.append_safe(" ");
                b__897.append_safe(temper_core::ListedTrait::get( & self.0.having_conditions, i__903).keyword());
                b__897.append_safe(" ");
                b__897.append_fragment(temper_core::ListedTrait::get( & self.0.having_conditions, i__903).condition());
                i__903 = i__903.wrapping_add(1);
            }
        }
        if ! temper_core::ListedTrait::is_empty( & self.0.order_clauses) {
            b__897.append_safe(" ORDER BY ");
            let mut first__904: std::sync::Arc<std::sync::RwLock<bool>> = std::sync::Arc::new(std::sync::RwLock::new(true));
            #[derive(Clone)]
            struct ClosureGroup___7 {
                first__904: std::sync::Arc<std::sync::RwLock<bool>>, b__897: SqlBuilder
            }
            impl ClosureGroup___7 {
                fn fn__10103(& self, oc__905: OrderClause) {
                    let mut t___5515: std::sync::Arc<String>;
                    if ! temper_core::read_locked( & self.first__904) {
                        self.b__897.append_safe(", ");
                    }
                    {
                        * self.first__904.write().unwrap() = false;
                    }
                    let mut t___10086: std::sync::Arc<String> = oc__905.field().sql_value();
                    self.b__897.append_safe(t___10086.clone());
                    if oc__905.ascending() {
                        t___5515 = std::sync::Arc::new(" ASC".to_string());
                    } else {
                        t___5515 = std::sync::Arc::new(" DESC".to_string());
                    }
                    self.b__897.append_safe(t___5515.clone());
                }
            }
            let closure_group = ClosureGroup___7 {
                first__904: first__904.clone(), b__897: b__897.clone()
            };
            let fn__10103 = {
                let closure_group = closure_group.clone();
                std::sync::Arc::new(move | oc__905: OrderClause | closure_group.fn__10103(oc__905))
            };
            temper_core::listed::list_for_each( & self.0.order_clauses, & ( * fn__10103.clone()));
        }
        let lv__906: Option<i32> = self.0.limit_val;
        if ! lv__906.is_none() {
            let lv___1952: i32 = lv__906.unwrap();
            b__897.append_safe(" LIMIT ");
            b__897.append_int32(lv___1952);
        }
        let ov__907: Option<i32> = self.0.offset_val;
        if ! ov__907.is_none() {
            let ov___1953: i32 = ov__907.unwrap();
            b__897.append_safe(" OFFSET ");
            b__897.append_int32(ov___1953);
        }
        return b__897.accumulated();
    }
    pub fn count_sql(& self) -> SqlFragment {
        let mut t___10055: i32;
        let mut t___10074: i32;
        let b__910: SqlBuilder = SqlBuilder::new();
        b__910.append_safe("SELECT COUNT(*) FROM ");
        b__910.append_safe(self.0.table_name.sql_value());
        #[derive(Clone)]
        struct ClosureGroup___8 {
            b__910: SqlBuilder
        }
        impl ClosureGroup___8 {
            fn fn__10043(& self, jc__911: JoinClause) {
                self.b__910.append_safe(" ");
                let mut t___10033: std::sync::Arc<String> = jc__911.join_type().keyword();
                self.b__910.append_safe(t___10033.clone());
                self.b__910.append_safe(" ");
                let mut t___10037: std::sync::Arc<String> = jc__911.table().sql_value();
                self.b__910.append_safe(t___10037.clone());
                self.b__910.append_safe(" ON ");
                let mut t___10040: SqlFragment = jc__911.on_condition();
                self.b__910.append_fragment(t___10040.clone());
            }
        }
        let closure_group = ClosureGroup___8 {
            b__910: b__910.clone()
        };
        let fn__10043 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | jc__911: JoinClause | closure_group.fn__10043(jc__911))
        };
        temper_core::listed::list_for_each( & self.0.join_clauses, & ( * fn__10043.clone()));
        if ! temper_core::ListedTrait::is_empty( & self.0.conditions) {
            b__910.append_safe(" WHERE ");
            b__910.append_fragment(temper_core::ListedTrait::get( & self.0.conditions, 0).condition());
            let mut i__912: i32 = 1;
            'loop___11053: loop {
                t___10055 = temper_core::ListedTrait::len( & self.0.conditions);
                if ! (Some(i__912) < Some(t___10055)) {
                    break;
                }
                b__910.append_safe(" ");
                b__910.append_safe(temper_core::ListedTrait::get( & self.0.conditions, i__912).keyword());
                b__910.append_safe(" ");
                b__910.append_fragment(temper_core::ListedTrait::get( & self.0.conditions, i__912).condition());
                i__912 = i__912.wrapping_add(1);
            }
        }
        if ! temper_core::ListedTrait::is_empty( & self.0.group_by_fields) {
            b__910.append_safe(" GROUP BY ");
            #[derive(Clone)]
            struct ClosureGroup___9 {}
            impl ClosureGroup___9 {
                fn fn__10042(& self, f__913: SafeIdentifier) -> std::sync::Arc<String> {
                    return f__913.sql_value();
                }
            }
            let closure_group = ClosureGroup___9 {};
            let fn__10042 = {
                let closure_group = closure_group.clone();
                std::sync::Arc::new(move | f__913: SafeIdentifier | closure_group.fn__10042(f__913))
            };
            b__910.append_safe(temper_core::listed::join( & self.0.group_by_fields, std::sync::Arc::new(", ".to_string()), & ( * fn__10042.clone())));
        }
        if ! temper_core::ListedTrait::is_empty( & self.0.having_conditions) {
            b__910.append_safe(" HAVING ");
            b__910.append_fragment(temper_core::ListedTrait::get( & self.0.having_conditions, 0).condition());
            let mut i__914: i32 = 1;
            'loop___11054: loop {
                t___10074 = temper_core::ListedTrait::len( & self.0.having_conditions);
                if ! (Some(i__914) < Some(t___10074)) {
                    break;
                }
                b__910.append_safe(" ");
                b__910.append_safe(temper_core::ListedTrait::get( & self.0.having_conditions, i__914).keyword());
                b__910.append_safe(" ");
                b__910.append_fragment(temper_core::ListedTrait::get( & self.0.having_conditions, i__914).condition());
                i__914 = i__914.wrapping_add(1);
            }
        }
        return b__910.accumulated();
    }
    pub fn safe_to_sql(& self, defaultLimit__916: i32) -> temper_core::Result<SqlFragment> {
        let return__387: SqlFragment;
        let mut t___5464: Query;
        if Some(defaultLimit__916) < Some(0) {
            return Err(temper_core::Error::new());
        }
        if ! self.0.limit_val.is_none() {
            return__387 = self.to_sql();
        } else {
            t___5464 = self.limit(defaultLimit__916) ? ;
            return__387 = t___5464.to_sql();
        }
        return Ok(return__387.clone());
    }
    pub fn new(tableName__919: SafeIdentifier, conditions__920: impl temper_core::ToList<WhereClause>, selectedFields__921: impl temper_core::ToList<SafeIdentifier>, orderClauses__922: impl temper_core::ToList<OrderClause>, limitVal__923: Option<i32>, offsetVal__924: Option<i32>, joinClauses__925: impl temper_core::ToList<JoinClause>, groupByFields__926: impl temper_core::ToList<SafeIdentifier>, havingConditions__927: impl temper_core::ToList<WhereClause>, isDistinct__928: bool, selectExprs__929: impl temper_core::ToList<SqlFragment>) -> Query {
        let conditions__920 = conditions__920.to_list();
        let selectedFields__921 = selectedFields__921.to_list();
        let orderClauses__922 = orderClauses__922.to_list();
        let joinClauses__925 = joinClauses__925.to_list();
        let groupByFields__926 = groupByFields__926.to_list();
        let havingConditions__927 = havingConditions__927.to_list();
        let selectExprs__929 = selectExprs__929.to_list();
        let table_name;
        let conditions;
        let selected_fields;
        let order_clauses;
        let limit_val;
        let offset_val;
        let join_clauses;
        let group_by_fields;
        let having_conditions;
        let is_distinct;
        let select_exprs;
        table_name = tableName__919.clone();
        conditions = conditions__920.clone();
        selected_fields = selectedFields__921.clone();
        order_clauses = orderClauses__922.clone();
        limit_val = limitVal__923;
        offset_val = offsetVal__924;
        join_clauses = joinClauses__925.clone();
        group_by_fields = groupByFields__926.clone();
        having_conditions = havingConditions__927.clone();
        is_distinct = isDistinct__928;
        select_exprs = selectExprs__929.clone();
        let selfish = Query(std::sync::Arc::new(QueryStruct {
                    table_name, conditions, selected_fields, order_clauses, limit_val, offset_val, join_clauses, group_by_fields, having_conditions, is_distinct, select_exprs
        }));
        return selfish;
    }
    pub fn table_name(& self) -> SafeIdentifier {
        return self.0.table_name.clone();
    }
    pub fn conditions(& self) -> temper_core::List<WhereClause> {
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
    pub fn join_clauses(& self) -> temper_core::List<JoinClause> {
        return self.0.join_clauses.clone();
    }
    pub fn group_by_fields(& self) -> temper_core::List<SafeIdentifier> {
        return self.0.group_by_fields.clone();
    }
    pub fn having_conditions(& self) -> temper_core::List<WhereClause> {
        return self.0.having_conditions.clone();
    }
    pub fn is_distinct(& self) -> bool {
        return self.0.is_distinct;
    }
    pub fn select_exprs(& self) -> temper_core::List<SqlFragment> {
        return self.0.select_exprs.clone();
    }
}
temper_core::impl_any_value_trait!(Query, []);
struct SetClauseStruct {
    field: SafeIdentifier, value: SqlPart
}
#[derive(Clone)]
pub struct SetClause(std::sync::Arc<SetClauseStruct>);
#[derive(Clone)]
pub struct SetClauseBuilder {
    pub field: SafeIdentifier, pub value: SqlPart
}
impl SetClauseBuilder {
    pub fn build(self) -> SetClause {
        SetClause::new(self.field, self.value)
    }
}
impl SetClause {
    pub fn new(field__979: SafeIdentifier, value__980: SqlPart) -> SetClause {
        let field;
        let value;
        field = field__979.clone();
        value = value__980.clone();
        let selfish = SetClause(std::sync::Arc::new(SetClauseStruct {
                    field, value
        }));
        return selfish;
    }
    pub fn field(& self) -> SafeIdentifier {
        return self.0.field.clone();
    }
    pub fn value(& self) -> SqlPart {
        return self.0.value.clone();
    }
}
temper_core::impl_any_value_trait!(SetClause, []);
struct UpdateQueryStruct {
    table_name: SafeIdentifier, set_clauses: temper_core::List<SetClause>, conditions: temper_core::List<WhereClause>, limit_val: Option<i32>
}
#[derive(Clone)]
pub struct UpdateQuery(std::sync::Arc<UpdateQueryStruct>);
#[derive(Clone)]
pub struct UpdateQueryBuilder {
    pub table_name: SafeIdentifier, pub set_clauses: temper_core::List<SetClause>, pub conditions: temper_core::List<WhereClause>, pub limit_val: Option<i32>
}
impl UpdateQueryBuilder {
    pub fn build(self) -> UpdateQuery {
        UpdateQuery::new(self.table_name, self.set_clauses, self.conditions, self.limit_val)
    }
}
impl UpdateQuery {
    pub fn set(& self, field__986: SafeIdentifier, value__987: SqlPart) -> UpdateQuery {
        let nb__989: temper_core::ListBuilder<SetClause> = temper_core::ListedTrait::to_list_builder( & self.0.set_clauses);
        temper_core::listed::add( & nb__989, SetClause::new(field__986.clone(), value__987.clone()), None);
        return UpdateQuery::new(self.0.table_name.clone(), temper_core::ListedTrait::to_list( & nb__989), self.0.conditions.clone(), self.0.limit_val);
    }
    pub fn r#where(& self, condition__991: SqlFragment) -> UpdateQuery {
        let nb__993: temper_core::ListBuilder<WhereClause> = temper_core::ListedTrait::to_list_builder( & self.0.conditions);
        temper_core::listed::add( & nb__993, WhereClause::new(AndCondition::new(condition__991.clone())), None);
        return UpdateQuery::new(self.0.table_name.clone(), self.0.set_clauses.clone(), temper_core::ListedTrait::to_list( & nb__993), self.0.limit_val);
    }
    pub fn or_where(& self, condition__995: SqlFragment) -> UpdateQuery {
        let nb__997: temper_core::ListBuilder<WhereClause> = temper_core::ListedTrait::to_list_builder( & self.0.conditions);
        temper_core::listed::add( & nb__997, WhereClause::new(OrCondition::new(condition__995.clone())), None);
        return UpdateQuery::new(self.0.table_name.clone(), self.0.set_clauses.clone(), temper_core::ListedTrait::to_list( & nb__997), self.0.limit_val);
    }
    pub fn limit(& self, n__999: i32) -> temper_core::Result<UpdateQuery> {
        if Some(n__999) < Some(0) {
            return Err(temper_core::Error::new());
        }
        return Ok(UpdateQuery::new(self.0.table_name.clone(), self.0.set_clauses.clone(), self.0.conditions.clone(), Some(n__999)));
    }
    pub fn to_sql(& self) -> temper_core::Result<SqlFragment> {
        let mut t___9890: i32;
        let mut t___9904: i32;
        if temper_core::ListedTrait::is_empty( & self.0.conditions) {
            return Err(temper_core::Error::new());
        }
        if temper_core::ListedTrait::is_empty( & self.0.set_clauses) {
            return Err(temper_core::Error::new());
        }
        let b__1003: SqlBuilder = SqlBuilder::new();
        b__1003.append_safe("UPDATE ");
        b__1003.append_safe(self.0.table_name.sql_value());
        b__1003.append_safe(" SET ");
        b__1003.append_safe(temper_core::ListedTrait::get( & self.0.set_clauses, 0).field().sql_value());
        b__1003.append_safe(" = ");
        b__1003.append_part(temper_core::ListedTrait::get( & self.0.set_clauses, 0).value());
        let mut i__1004: i32 = 1;
        'loop___11062: loop {
            t___9890 = temper_core::ListedTrait::len( & self.0.set_clauses);
            if ! (Some(i__1004) < Some(t___9890)) {
                break;
            }
            b__1003.append_safe(", ");
            b__1003.append_safe(temper_core::ListedTrait::get( & self.0.set_clauses, i__1004).field().sql_value());
            b__1003.append_safe(" = ");
            b__1003.append_part(temper_core::ListedTrait::get( & self.0.set_clauses, i__1004).value());
            i__1004 = i__1004.wrapping_add(1);
        }
        b__1003.append_safe(" WHERE ");
        b__1003.append_fragment(temper_core::ListedTrait::get( & self.0.conditions, 0).condition());
        let mut i__1005: i32 = 1;
        'loop___11063: loop {
            t___9904 = temper_core::ListedTrait::len( & self.0.conditions);
            if ! (Some(i__1005) < Some(t___9904)) {
                break;
            }
            b__1003.append_safe(" ");
            b__1003.append_safe(temper_core::ListedTrait::get( & self.0.conditions, i__1005).keyword());
            b__1003.append_safe(" ");
            b__1003.append_fragment(temper_core::ListedTrait::get( & self.0.conditions, i__1005).condition());
            i__1005 = i__1005.wrapping_add(1);
        }
        let lv__1006: Option<i32> = self.0.limit_val;
        if ! lv__1006.is_none() {
            let lv___1954: i32 = lv__1006.unwrap();
            b__1003.append_safe(" LIMIT ");
            b__1003.append_int32(lv___1954);
        }
        return Ok(b__1003.accumulated());
    }
    pub fn new(tableName__1008: SafeIdentifier, setClauses__1009: impl temper_core::ToList<SetClause>, conditions__1010: impl temper_core::ToList<WhereClause>, limitVal__1011: Option<i32>) -> UpdateQuery {
        let setClauses__1009 = setClauses__1009.to_list();
        let conditions__1010 = conditions__1010.to_list();
        let table_name;
        let set_clauses;
        let conditions;
        let limit_val;
        table_name = tableName__1008.clone();
        set_clauses = setClauses__1009.clone();
        conditions = conditions__1010.clone();
        limit_val = limitVal__1011;
        let selfish = UpdateQuery(std::sync::Arc::new(UpdateQueryStruct {
                    table_name, set_clauses, conditions, limit_val
        }));
        return selfish;
    }
    pub fn table_name(& self) -> SafeIdentifier {
        return self.0.table_name.clone();
    }
    pub fn set_clauses(& self) -> temper_core::List<SetClause> {
        return self.0.set_clauses.clone();
    }
    pub fn conditions(& self) -> temper_core::List<WhereClause> {
        return self.0.conditions.clone();
    }
    pub fn limit_val(& self) -> Option<i32> {
        return self.0.limit_val;
    }
}
temper_core::impl_any_value_trait!(UpdateQuery, []);
struct DeleteQueryStruct {
    table_name: SafeIdentifier, conditions: temper_core::List<WhereClause>, limit_val: Option<i32>
}
#[derive(Clone)]
pub struct DeleteQuery(std::sync::Arc<DeleteQueryStruct>);
#[derive(Clone)]
pub struct DeleteQueryBuilder {
    pub table_name: SafeIdentifier, pub conditions: temper_core::List<WhereClause>, pub limit_val: Option<i32>
}
impl DeleteQueryBuilder {
    pub fn build(self) -> DeleteQuery {
        DeleteQuery::new(self.table_name, self.conditions, self.limit_val)
    }
}
impl DeleteQuery {
    pub fn r#where(& self, condition__1016: SqlFragment) -> DeleteQuery {
        let nb__1018: temper_core::ListBuilder<WhereClause> = temper_core::ListedTrait::to_list_builder( & self.0.conditions);
        temper_core::listed::add( & nb__1018, WhereClause::new(AndCondition::new(condition__1016.clone())), None);
        return DeleteQuery::new(self.0.table_name.clone(), temper_core::ListedTrait::to_list( & nb__1018), self.0.limit_val);
    }
    pub fn or_where(& self, condition__1020: SqlFragment) -> DeleteQuery {
        let nb__1022: temper_core::ListBuilder<WhereClause> = temper_core::ListedTrait::to_list_builder( & self.0.conditions);
        temper_core::listed::add( & nb__1022, WhereClause::new(OrCondition::new(condition__1020.clone())), None);
        return DeleteQuery::new(self.0.table_name.clone(), temper_core::ListedTrait::to_list( & nb__1022), self.0.limit_val);
    }
    pub fn limit(& self, n__1024: i32) -> temper_core::Result<DeleteQuery> {
        if Some(n__1024) < Some(0) {
            return Err(temper_core::Error::new());
        }
        return Ok(DeleteQuery::new(self.0.table_name.clone(), self.0.conditions.clone(), Some(n__1024)));
    }
    pub fn to_sql(& self) -> temper_core::Result<SqlFragment> {
        let mut t___9850: i32;
        if temper_core::ListedTrait::is_empty( & self.0.conditions) {
            return Err(temper_core::Error::new());
        }
        let b__1028: SqlBuilder = SqlBuilder::new();
        b__1028.append_safe("DELETE FROM ");
        b__1028.append_safe(self.0.table_name.sql_value());
        b__1028.append_safe(" WHERE ");
        b__1028.append_fragment(temper_core::ListedTrait::get( & self.0.conditions, 0).condition());
        let mut i__1029: i32 = 1;
        'loop___11069: loop {
            t___9850 = temper_core::ListedTrait::len( & self.0.conditions);
            if ! (Some(i__1029) < Some(t___9850)) {
                break;
            }
            b__1028.append_safe(" ");
            b__1028.append_safe(temper_core::ListedTrait::get( & self.0.conditions, i__1029).keyword());
            b__1028.append_safe(" ");
            b__1028.append_fragment(temper_core::ListedTrait::get( & self.0.conditions, i__1029).condition());
            i__1029 = i__1029.wrapping_add(1);
        }
        let lv__1030: Option<i32> = self.0.limit_val;
        if ! lv__1030.is_none() {
            let lv___1955: i32 = lv__1030.unwrap();
            b__1028.append_safe(" LIMIT ");
            b__1028.append_int32(lv___1955);
        }
        return Ok(b__1028.accumulated());
    }
    pub fn new(tableName__1032: SafeIdentifier, conditions__1033: impl temper_core::ToList<WhereClause>, limitVal__1034: Option<i32>) -> DeleteQuery {
        let conditions__1033 = conditions__1033.to_list();
        let table_name;
        let conditions;
        let limit_val;
        table_name = tableName__1032.clone();
        conditions = conditions__1033.clone();
        limit_val = limitVal__1034;
        let selfish = DeleteQuery(std::sync::Arc::new(DeleteQueryStruct {
                    table_name, conditions, limit_val
        }));
        return selfish;
    }
    pub fn table_name(& self) -> SafeIdentifier {
        return self.0.table_name.clone();
    }
    pub fn conditions(& self) -> temper_core::List<WhereClause> {
        return self.0.conditions.clone();
    }
    pub fn limit_val(& self) -> Option<i32> {
        return self.0.limit_val;
    }
}
temper_core::impl_any_value_trait!(DeleteQuery, []);
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
    pub fn new(_value__1253: impl temper_core::ToArcString) -> ValidatedIdentifier {
        let _value__1253 = _value__1253.to_arc_string();
        let value;
        value = _value__1253.clone();
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
    pub fn new(name__1271: SafeIdentifier, fieldType__1272: FieldType, nullable__1273: bool) -> FieldDef {
        let name;
        let field_type;
        let nullable;
        name = name__1271.clone();
        field_type = fieldType__1272.clone();
        nullable = nullable__1273;
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
    pub fn field(& self, name__1277: impl temper_core::ToArcString) -> temper_core::Result<FieldDef> {
        let name__1277 = name__1277.to_arc_string();
        let return__451: FieldDef;
        'fn__1278: {
            let this__6482: temper_core::List<FieldDef> = self.0.fields.clone();
            let n__6483: i32 = temper_core::ListedTrait::len( & this__6482);
            let mut i__6484: i32 = 0;
            'loop___11073: while Some(i__6484) < Some(n__6483) {
                let el__6485: FieldDef = temper_core::ListedTrait::get( & this__6482, i__6484);
                i__6484 = i__6484.wrapping_add(1);
                let f__1279: FieldDef = el__6485.clone();
                if Some(f__1279.name().sql_value().as_str()) == Some(name__1277.as_str()) {
                    return__451 = f__1279.clone();
                    break 'fn__1278;
                }
            }
            return Err(temper_core::Error::new());
        }
        return Ok(return__451.clone());
    }
    pub fn new(tableName__1281: SafeIdentifier, fields__1282: impl temper_core::ToList<FieldDef>) -> TableDef {
        let fields__1282 = fields__1282.to_list();
        let table_name;
        let fields;
        table_name = tableName__1281.clone();
        fields = fields__1282.clone();
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
    pub fn append_safe(& self, sqlSource__1304: impl temper_core::ToArcString) {
        let sqlSource__1304 = sqlSource__1304.to_arc_string();
        let mut t___10895: SqlSource = SqlSource::new(sqlSource__1304.clone());
        temper_core::listed::add( & self.0.buffer, SqlPart::new(t___10895.clone()), None);
    }
    pub fn append_fragment(& self, fragment__1307: SqlFragment) {
        let mut t___10893: temper_core::List<SqlPart> = fragment__1307.parts();
        temper_core::listed::add_all( & self.0.buffer, temper_core::ToListed::to_listed(t___10893.clone()), None);
    }
    pub fn append_part(& self, part__1310: SqlPart) {
        temper_core::listed::add( & self.0.buffer, part__1310.clone(), None);
    }
    pub fn append_part_list(& self, values__1313: impl temper_core::ToList<SqlPart>) {
        let values__1313 = values__1313.to_list();
        #[derive(Clone)]
        struct ClosureGroup___10 {
            this__244: SqlBuilder
        }
        impl ClosureGroup___10 {
            fn fn__10889(& self, x__1315: SqlPart) {
                self.this__244.append_part(x__1315.clone());
            }
        }
        let closure_group = ClosureGroup___10 {
            this__244: self.clone()
        };
        let fn__10889 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | x__1315: SqlPart | closure_group.fn__10889(x__1315))
        };
        self.append_list(temper_core::ToListed::to_listed(values__1313.clone()), fn__10889.clone());
    }
    pub fn append_boolean(& self, value__1317: bool) {
        let mut t___10886: SqlBoolean = SqlBoolean::new(value__1317);
        temper_core::listed::add( & self.0.buffer, SqlPart::new(t___10886.clone()), None);
    }
    pub fn append_boolean_list(& self, values__1320: impl temper_core::ToListed<bool>) {
        let values__1320 = values__1320.to_listed();
        #[derive(Clone)]
        struct ClosureGroup___11 {
            this__246: SqlBuilder
        }
        impl ClosureGroup___11 {
            fn fn__10883(& self, x__1322: bool) {
                self.this__246.append_boolean(x__1322);
            }
        }
        let closure_group = ClosureGroup___11 {
            this__246: self.clone()
        };
        let fn__10883 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | x__1322: bool | closure_group.fn__10883(x__1322))
        };
        self.append_list(values__1320.clone(), fn__10883.clone());
    }
    pub fn append_date(& self, value__1324: temper_std::temporal::Date) {
        let mut t___10880: SqlDate = SqlDate::new(value__1324.clone());
        temper_core::listed::add( & self.0.buffer, SqlPart::new(t___10880.clone()), None);
    }
    pub fn append_date_list(& self, values__1327: impl temper_core::ToListed<temper_std::temporal::Date>) {
        let values__1327 = values__1327.to_listed();
        #[derive(Clone)]
        struct ClosureGroup___12 {
            this__248: SqlBuilder
        }
        impl ClosureGroup___12 {
            fn fn__10877(& self, x__1329: temper_std::temporal::Date) {
                self.this__248.append_date(x__1329.clone());
            }
        }
        let closure_group = ClosureGroup___12 {
            this__248: self.clone()
        };
        let fn__10877 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | x__1329: temper_std::temporal::Date | closure_group.fn__10877(x__1329))
        };
        self.append_list(values__1327.clone(), fn__10877.clone());
    }
    pub fn append_float64(& self, value__1331: f64) {
        let mut t___10874: SqlFloat64 = SqlFloat64::new(value__1331);
        temper_core::listed::add( & self.0.buffer, SqlPart::new(t___10874.clone()), None);
    }
    pub fn append_float64_list(& self, values__1334: impl temper_core::ToListed<f64>) {
        let values__1334 = values__1334.to_listed();
        #[derive(Clone)]
        struct ClosureGroup___13 {
            this__250: SqlBuilder
        }
        impl ClosureGroup___13 {
            fn fn__10871(& self, x__1336: f64) {
                self.this__250.append_float64(x__1336);
            }
        }
        let closure_group = ClosureGroup___13 {
            this__250: self.clone()
        };
        let fn__10871 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | x__1336: f64 | closure_group.fn__10871(x__1336))
        };
        self.append_list(values__1334.clone(), fn__10871.clone());
    }
    pub fn append_int32(& self, value__1338: i32) {
        let mut t___10868: SqlInt32 = SqlInt32::new(value__1338);
        temper_core::listed::add( & self.0.buffer, SqlPart::new(t___10868.clone()), None);
    }
    pub fn append_int32_list(& self, values__1341: impl temper_core::ToListed<i32>) {
        let values__1341 = values__1341.to_listed();
        #[derive(Clone)]
        struct ClosureGroup___14 {
            this__252: SqlBuilder
        }
        impl ClosureGroup___14 {
            fn fn__10865(& self, x__1343: i32) {
                self.this__252.append_int32(x__1343);
            }
        }
        let closure_group = ClosureGroup___14 {
            this__252: self.clone()
        };
        let fn__10865 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | x__1343: i32 | closure_group.fn__10865(x__1343))
        };
        self.append_list(values__1341.clone(), fn__10865.clone());
    }
    pub fn append_int64(& self, value__1345: i64) {
        let mut t___10862: SqlInt64 = SqlInt64::new(value__1345);
        temper_core::listed::add( & self.0.buffer, SqlPart::new(t___10862.clone()), None);
    }
    pub fn append_int64_list(& self, values__1348: impl temper_core::ToListed<i64>) {
        let values__1348 = values__1348.to_listed();
        #[derive(Clone)]
        struct ClosureGroup___15 {
            this__254: SqlBuilder
        }
        impl ClosureGroup___15 {
            fn fn__10859(& self, x__1350: i64) {
                self.this__254.append_int64(x__1350);
            }
        }
        let closure_group = ClosureGroup___15 {
            this__254: self.clone()
        };
        let fn__10859 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | x__1350: i64 | closure_group.fn__10859(x__1350))
        };
        self.append_list(values__1348.clone(), fn__10859.clone());
    }
    pub fn append_string(& self, value__1352: impl temper_core::ToArcString) {
        let value__1352 = value__1352.to_arc_string();
        let mut t___10856: SqlString = SqlString::new(value__1352.clone());
        temper_core::listed::add( & self.0.buffer, SqlPart::new(t___10856.clone()), None);
    }
    pub fn append_string_list(& self, values__1355: impl temper_core::ToListed<std::sync::Arc<String>>) {
        let values__1355 = values__1355.to_listed();
        #[derive(Clone)]
        struct ClosureGroup___16 {
            this__256: SqlBuilder
        }
        impl ClosureGroup___16 {
            fn fn__10853(& self, x__1357: impl temper_core::ToArcString) {
                let x__1357 = x__1357.to_arc_string();
                self.this__256.append_string(x__1357.clone());
            }
        }
        let closure_group = ClosureGroup___16 {
            this__256: self.clone()
        };
        let fn__10853 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | x__1357: std::sync::Arc<String> | closure_group.fn__10853(x__1357))
        };
        self.append_list(values__1355.clone(), fn__10853.clone());
    }
    fn append_list<T>(& self, values__1359: impl temper_core::ToListed<T>, appendValue__1360: std::sync::Arc<dyn Fn (T) + std::marker::Send + std::marker::Sync>) where T: Clone + std::marker::Send + std::marker::Sync + 'static {
        let values__1359 = values__1359.to_listed();
        let mut t___10848: i32;
        let mut t___10850: T;
        let mut i__1362: i32 = 0;
        'loop___11074: loop {
            t___10848 = temper_core::ListedTrait::len( & ( * values__1359));
            if ! (Some(i__1362) < Some(t___10848)) {
                break;
            }
            if Some(i__1362) > Some(0) {
                self.append_safe(", ");
            }
            t___10850 = temper_core::ListedTrait::get( & ( * values__1359), i__1362);
            appendValue__1360(t___10850.clone());
            i__1362 = i__1362.wrapping_add(1);
        }
    }
    pub fn accumulated(& self) -> SqlFragment {
        return SqlFragment::new(temper_core::ListedTrait::to_list( & self.0.buffer));
    }
    pub fn new() -> SqlBuilder {
        let buffer;
        let mut t___10845: temper_core::ListBuilder<SqlPart> = temper_core::listed::new_builder();
        buffer = t___10845.clone();
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
        let mut t___10919: i32;
        let builder__1374: std::sync::Arc<std::sync::RwLock<String>> = std::sync::Arc::new(std::sync::RwLock::new(String::new()));
        let mut i__1375: i32 = 0;
        'loop___11075: loop {
            t___10919 = temper_core::ListedTrait::len( & self.0.parts);
            if ! (Some(i__1375) < Some(t___10919)) {
                break;
            }
            temper_core::ListedTrait::get( & self.0.parts, i__1375).format_to(builder__1374.clone());
            i__1375 = i__1375.wrapping_add(1);
        }
        return temper_core::string::builder::to_string( & builder__1374);
    }
    pub fn new(parts__1377: impl temper_core::ToList<SqlPart>) -> SqlFragment {
        let parts__1377 = parts__1377.to_list();
        let parts;
        parts = parts__1377.clone();
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
    fn format_to(& self, builder__1379: std::sync::Arc<std::sync::RwLock<String>>);
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
    pub fn format_to(& self, builder__1383: std::sync::Arc<std::sync::RwLock<String>>) {
        temper_core::string::builder::append( & builder__1383, self.0.source.clone());
    }
    pub fn new(source__1386: impl temper_core::ToArcString) -> SqlSource {
        let source__1386 = source__1386.to_arc_string();
        let source;
        source = source__1386.clone();
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
    fn format_to(& self, builder__1383: std::sync::Arc<std::sync::RwLock<String>>) {
        self.format_to(builder__1383)
    }
}
temper_core::impl_any_value_trait!(SqlSource, [SqlPart]);
struct SqlBooleanStruct {
    value: bool
}
#[derive(Clone)]
pub struct SqlBoolean(std::sync::Arc<SqlBooleanStruct>);
impl SqlBoolean {
    pub fn format_to(& self, builder__1389: std::sync::Arc<std::sync::RwLock<String>>) {
        let mut t___6281: std::sync::Arc<String>;
        if self.0.value {
            t___6281 = std::sync::Arc::new("TRUE".to_string());
        } else {
            t___6281 = std::sync::Arc::new("FALSE".to_string());
        }
        temper_core::string::builder::append( & builder__1389, t___6281.clone());
    }
    pub fn new(value__1392: bool) -> SqlBoolean {
        let value;
        value = value__1392;
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
    fn format_to(& self, builder__1389: std::sync::Arc<std::sync::RwLock<String>>) {
        self.format_to(builder__1389)
    }
}
temper_core::impl_any_value_trait!(SqlBoolean, [SqlPart]);
struct SqlDateStruct {
    value: temper_std::temporal::Date
}
#[derive(Clone)]
pub struct SqlDate(std::sync::Arc<SqlDateStruct>);
impl SqlDate {
    pub fn format_to(& self, builder__1395: std::sync::Arc<std::sync::RwLock<String>>) {
        temper_core::string::builder::append( & builder__1395, "'");
        let mut t___10900: std::sync::Arc<String> = self.0.value.to_string();
        #[derive(Clone)]
        struct ClosureGroup___17 {
            builder__1395: std::sync::Arc<std::sync::RwLock<String>>
        }
        impl ClosureGroup___17 {
            fn fn__10898(& self, c__1397: i32) {
                if Some(c__1397) == Some(39) {
                    temper_core::string::builder::append( & self.builder__1395, "''");
                } else {
                    'ok___10958: {
                        'orelse___1892: {
                            match temper_core::string::builder::append_code_point( & self.builder__1395, c__1397) {
                                Ok(x) => x,
                                _ => break 'orelse___1892
                            };
                            break 'ok___10958;
                        }
                        return panic!();
                    }
                }
            }
        }
        let closure_group = ClosureGroup___17 {
            builder__1395: builder__1395.clone()
        };
        let fn__10898 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | c__1397: i32 | closure_group.fn__10898(c__1397))
        };
        temper_core::string::for_each( & t___10900, & ( * fn__10898.clone()));
        temper_core::string::builder::append( & builder__1395, "'");
    }
    pub fn new(value__1399: temper_std::temporal::Date) -> SqlDate {
        let value;
        value = value__1399.clone();
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
    fn format_to(& self, builder__1395: std::sync::Arc<std::sync::RwLock<String>>) {
        self.format_to(builder__1395)
    }
}
temper_core::impl_any_value_trait!(SqlDate, [SqlPart]);
struct SqlFloat64Struct {
    value: f64
}
#[derive(Clone)]
pub struct SqlFloat64(std::sync::Arc<SqlFloat64Struct>);
impl SqlFloat64 {
    pub fn format_to(& self, builder__1402: std::sync::Arc<std::sync::RwLock<String>>) {
        let mut t___6270: bool;
        let mut t___6271: bool;
        let s__1404: std::sync::Arc<String> = temper_core::float64::to_string(self.0.value);
        if Some(s__1404.as_str()) == Some("NaN") {
            t___6271 = true;
        } else {
            if Some(s__1404.as_str()) == Some("Infinity") {
                t___6270 = true;
            } else {
                t___6270 = Some(s__1404.as_str()) == Some("-Infinity");
            }
            t___6271 = t___6270;
        }
        if t___6271 {
            temper_core::string::builder::append( & builder__1402, "NULL");
        } else {
            temper_core::string::builder::append( & builder__1402, s__1404.clone());
        }
    }
    pub fn new(value__1406: f64) -> SqlFloat64 {
        let value;
        value = value__1406;
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
    fn format_to(& self, builder__1402: std::sync::Arc<std::sync::RwLock<String>>) {
        self.format_to(builder__1402)
    }
}
temper_core::impl_any_value_trait!(SqlFloat64, [SqlPart]);
struct SqlInt32Struct {
    value: i32
}
#[derive(Clone)]
pub struct SqlInt32(std::sync::Arc<SqlInt32Struct>);
impl SqlInt32 {
    pub fn format_to(& self, builder__1409: std::sync::Arc<std::sync::RwLock<String>>) {
        let mut t___10909: std::sync::Arc<String> = temper_core::int_to_string(self.0.value, None);
        temper_core::string::builder::append( & builder__1409, t___10909.clone());
    }
    pub fn new(value__1412: i32) -> SqlInt32 {
        let value;
        value = value__1412;
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
    fn format_to(& self, builder__1409: std::sync::Arc<std::sync::RwLock<String>>) {
        self.format_to(builder__1409)
    }
}
temper_core::impl_any_value_trait!(SqlInt32, [SqlPart]);
struct SqlInt64Struct {
    value: i64
}
#[derive(Clone)]
pub struct SqlInt64(std::sync::Arc<SqlInt64Struct>);
impl SqlInt64 {
    pub fn format_to(& self, builder__1415: std::sync::Arc<std::sync::RwLock<String>>) {
        let mut t___10907: std::sync::Arc<String> = temper_core::int64_to_string(self.0.value, None);
        temper_core::string::builder::append( & builder__1415, t___10907.clone());
    }
    pub fn new(value__1418: i64) -> SqlInt64 {
        let value;
        value = value__1418;
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
    fn format_to(& self, builder__1415: std::sync::Arc<std::sync::RwLock<String>>) {
        self.format_to(builder__1415)
    }
}
temper_core::impl_any_value_trait!(SqlInt64, [SqlPart]);
struct SqlStringStruct {
    value: std::sync::Arc<String>
}
#[derive(Clone)]
pub struct SqlString(std::sync::Arc<SqlStringStruct>);
impl SqlString {
    pub fn format_to(& self, builder__1421: std::sync::Arc<std::sync::RwLock<String>>) {
        temper_core::string::builder::append( & builder__1421, "'");
        #[derive(Clone)]
        struct ClosureGroup___18 {
            builder__1421: std::sync::Arc<std::sync::RwLock<String>>
        }
        impl ClosureGroup___18 {
            fn fn__10912(& self, c__1423: i32) {
                if Some(c__1423) == Some(39) {
                    temper_core::string::builder::append( & self.builder__1421, "''");
                } else {
                    'ok___10963: {
                        'orelse___1891: {
                            match temper_core::string::builder::append_code_point( & self.builder__1421, c__1423) {
                                Ok(x) => x,
                                _ => break 'orelse___1891
                            };
                            break 'ok___10963;
                        }
                        return panic!();
                    }
                }
            }
        }
        let closure_group = ClosureGroup___18 {
            builder__1421: builder__1421.clone()
        };
        let fn__10912 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | c__1423: i32 | closure_group.fn__10912(c__1423))
        };
        temper_core::string::for_each( & self.0.value, & ( * fn__10912.clone()));
        temper_core::string::builder::append( & builder__1421, "'");
    }
    pub fn new(value__1425: impl temper_core::ToArcString) -> SqlString {
        let value__1425 = value__1425.to_arc_string();
        let value;
        value = value__1425.clone();
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
    fn format_to(& self, builder__1421: std::sync::Arc<std::sync::RwLock<String>>) {
        self.format_to(builder__1421)
    }
}
temper_core::impl_any_value_trait!(SqlString, [SqlPart]);
pub fn changeset(tableDef__645: TableDef, params__646: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>>) -> Changeset {
    let mut t___10695: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & []);
    return Changeset::new(ChangesetImpl::new(tableDef__645.clone(), params__646.clone(), t___10695.clone(), [], true));
}
fn isIdentStart__506(c__1254: i32) -> bool {
    let return__431: bool;
    let mut t___6044: bool;
    let mut t___6045: bool;
    if Some(c__1254) >= Some(97) {
        t___6044 = Some(c__1254) <= Some(122);
    } else {
        t___6044 = false;
    }
    if t___6044 {
        return__431 = true;
    } else {
        if Some(c__1254) >= Some(65) {
            t___6045 = Some(c__1254) <= Some(90);
        } else {
            t___6045 = false;
        }
        if t___6045 {
            return__431 = true;
        } else {
            return__431 = Some(c__1254) == Some(95);
        }
    }
    return return__431;
}
fn isIdentPart__507(c__1256: i32) -> bool {
    let return__432: bool;
    if isIdentStart__506(c__1256) {
        return__432 = true;
    } else {
        if Some(c__1256) >= Some(48) {
            return__432 = Some(c__1256) <= Some(57);
        } else {
            return__432 = false;
        }
    }
    return return__432;
}
pub fn safe_identifier(name__1258: impl temper_core::ToArcString) -> temper_core::Result<SafeIdentifier> {
    let name__1258 = name__1258.to_arc_string();
    let mut t___10693: usize;
    if name__1258.is_empty() {
        return Err(temper_core::Error::new());
    }
    let mut idx__1260: usize = 0usize;
    if ! isIdentStart__506(temper_core::string::get( & name__1258, idx__1260)) {
        return Err(temper_core::Error::new());
    }
    let mut t___10690: usize = temper_core::string::next( & name__1258, idx__1260);
    idx__1260 = t___10690;
    'loop___11076: loop {
        if ! temper_core::string::has_index( & name__1258, idx__1260) {
            break;
        }
        if ! isIdentPart__507(temper_core::string::get( & name__1258, idx__1260)) {
            return Err(temper_core::Error::new());
        }
        t___10693 = temper_core::string::next( & name__1258, idx__1260);
        idx__1260 = t___10693;
    }
    return Ok(SafeIdentifier::new(ValidatedIdentifier::new(name__1258.clone())));
}
fn csid__503(name__648: impl temper_core::ToArcString) -> SafeIdentifier {
    let name__648 = name__648.to_arc_string();
    let return__317: SafeIdentifier;
    let mut t___6032: SafeIdentifier;
    'ok___10968: {
        'orelse___1896: {
            t___6032 = match safe_identifier(name__648.clone()) {
                Ok(x) => x,
                _ => break 'orelse___1896
            };
            return__317 = t___6032.clone();
            break 'ok___10968;
        }
        return__317 = temper_core::cast::<SafeIdentifier>(panic!()).unwrap();
    }
    return return__317.clone();
}
fn userTable__504() -> TableDef {
    return TableDef::new(csid__503("users"), [FieldDef::new(csid__503("name"), FieldType::new(StringField::new()), false), FieldDef::new(csid__503("email"), FieldType::new(StringField::new()), false), FieldDef::new(csid__503("age"), FieldType::new(IntField::new()), true), FieldDef::new(csid__503("score"), FieldType::new(FloatField::new()), true), FieldDef::new(csid__503("active"), FieldType::new(BoolField::new()), true)]);
}
pub fn delete_sql(tableDef__735: TableDef, id__736: i32) -> SqlFragment {
    let b__738: SqlBuilder = SqlBuilder::new();
    b__738.append_safe("DELETE FROM ");
    b__738.append_safe(tableDef__735.table_name().sql_value());
    b__738.append_safe(" WHERE id = ");
    b__738.append_int32(id__736);
    return b__738.accumulated();
}
pub fn from(tableName__930: SafeIdentifier) -> Query {
    return Query::new(tableName__930.clone(), [], [], [], None, None, [], [], [], false, []);
}
pub fn col(table__932: SafeIdentifier, column__933: SafeIdentifier) -> SqlFragment {
    let b__935: SqlBuilder = SqlBuilder::new();
    b__935.append_safe(table__932.sql_value());
    b__935.append_safe(".");
    b__935.append_safe(column__933.sql_value());
    return b__935.accumulated();
}
pub fn count_all() -> SqlFragment {
    let b__937: SqlBuilder = SqlBuilder::new();
    b__937.append_safe("COUNT(*)");
    return b__937.accumulated();
}
pub fn count_col(field__938: SafeIdentifier) -> SqlFragment {
    let b__940: SqlBuilder = SqlBuilder::new();
    b__940.append_safe("COUNT(");
    b__940.append_safe(field__938.sql_value());
    b__940.append_safe(")");
    return b__940.accumulated();
}
pub fn sum_col(field__941: SafeIdentifier) -> SqlFragment {
    let b__943: SqlBuilder = SqlBuilder::new();
    b__943.append_safe("SUM(");
    b__943.append_safe(field__941.sql_value());
    b__943.append_safe(")");
    return b__943.accumulated();
}
pub fn avg_col(field__944: SafeIdentifier) -> SqlFragment {
    let b__946: SqlBuilder = SqlBuilder::new();
    b__946.append_safe("AVG(");
    b__946.append_safe(field__944.sql_value());
    b__946.append_safe(")");
    return b__946.accumulated();
}
pub fn min_col(field__947: SafeIdentifier) -> SqlFragment {
    let b__949: SqlBuilder = SqlBuilder::new();
    b__949.append_safe("MIN(");
    b__949.append_safe(field__947.sql_value());
    b__949.append_safe(")");
    return b__949.accumulated();
}
pub fn max_col(field__950: SafeIdentifier) -> SqlFragment {
    let b__952: SqlBuilder = SqlBuilder::new();
    b__952.append_safe("MAX(");
    b__952.append_safe(field__950.sql_value());
    b__952.append_safe(")");
    return b__952.accumulated();
}
pub fn union_sql(a__953: Query, b__954: Query) -> SqlFragment {
    let sb__956: SqlBuilder = SqlBuilder::new();
    sb__956.append_safe("(");
    sb__956.append_fragment(a__953.to_sql());
    sb__956.append_safe(") UNION (");
    sb__956.append_fragment(b__954.to_sql());
    sb__956.append_safe(")");
    return sb__956.accumulated();
}
pub fn union_all_sql(a__957: Query, b__958: Query) -> SqlFragment {
    let sb__960: SqlBuilder = SqlBuilder::new();
    sb__960.append_safe("(");
    sb__960.append_fragment(a__957.to_sql());
    sb__960.append_safe(") UNION ALL (");
    sb__960.append_fragment(b__958.to_sql());
    sb__960.append_safe(")");
    return sb__960.accumulated();
}
pub fn intersect_sql(a__961: Query, b__962: Query) -> SqlFragment {
    let sb__964: SqlBuilder = SqlBuilder::new();
    sb__964.append_safe("(");
    sb__964.append_fragment(a__961.to_sql());
    sb__964.append_safe(") INTERSECT (");
    sb__964.append_fragment(b__962.to_sql());
    sb__964.append_safe(")");
    return sb__964.accumulated();
}
pub fn except_sql(a__965: Query, b__966: Query) -> SqlFragment {
    let sb__968: SqlBuilder = SqlBuilder::new();
    sb__968.append_safe("(");
    sb__968.append_fragment(a__965.to_sql());
    sb__968.append_safe(") EXCEPT (");
    sb__968.append_fragment(b__966.to_sql());
    sb__968.append_safe(")");
    return sb__968.accumulated();
}
pub fn subquery(q__969: Query, alias__970: SafeIdentifier) -> SqlFragment {
    let b__972: SqlBuilder = SqlBuilder::new();
    b__972.append_safe("(");
    b__972.append_fragment(q__969.to_sql());
    b__972.append_safe(") AS ");
    b__972.append_safe(alias__970.sql_value());
    return b__972.accumulated();
}
pub fn exists_sql(q__973: Query) -> SqlFragment {
    let b__975: SqlBuilder = SqlBuilder::new();
    b__975.append_safe("EXISTS (");
    b__975.append_fragment(q__973.to_sql());
    b__975.append_safe(")");
    return b__975.accumulated();
}
pub fn update(tableName__1035: SafeIdentifier) -> UpdateQuery {
    return UpdateQuery::new(tableName__1035.clone(), [], [], None);
}
pub fn delete_from(tableName__1037: SafeIdentifier) -> DeleteQuery {
    return DeleteQuery::new(tableName__1037.clone(), [], None);
}
fn sid__505(name__1039: impl temper_core::ToArcString) -> SafeIdentifier {
    let name__1039 = name__1039.to_arc_string();
    let return__424: SafeIdentifier;
    let mut t___5241: SafeIdentifier;
    'ok___10979: {
        'orelse___1904: {
            t___5241 = match safe_identifier(name__1039.clone()) {
                Ok(x) => x,
                _ => break 'orelse___1904
            };
            return__424 = t___5241.clone();
            break 'ok___10979;
        }
        return__424 = temper_core::cast::<SafeIdentifier>(panic!()).unwrap();
    }
    return return__424.clone();
}
#[cfg(test)]
mod tests {
    #[test]
    fn castWhitelistsAllowedFields__1546() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___24 = temper_std::testing::Test::new();
        let params__652: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("name".to_string()), std::sync::Arc::new("Alice".to_string())), (std::sync::Arc::new("email".to_string()), std::sync::Arc::new("alice@example.com".to_string())), (std::sync::Arc::new("admin".to_string()), std::sync::Arc::new("true".to_string()))]);
        let mut t___10651: TableDef = userTable__504();
        let mut t___10652: SafeIdentifier = csid__503("name");
        let mut t___10653: SafeIdentifier = csid__503("email");
        let cs__653: Changeset = changeset(t___10651.clone(), params__652.clone()).cast(std::sync::Arc::new(vec![t___10652.clone(), t___10653.clone()]));
        let mut t___10656: bool = temper_core::MappedTrait::has( & cs__653.changes(), std::sync::Arc::new("name".to_string()));
        #[derive(Clone)]
        struct ClosureGroup___19 {}
        impl ClosureGroup___19 {
            fn fn__10646(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("name should be in changes".to_string());
            }
        }
        let closure_group = ClosureGroup___19 {};
        let fn__10646 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__10646())
        };
        test___24.assert(t___10656, fn__10646.clone());
        let mut t___10660: bool = temper_core::MappedTrait::has( & cs__653.changes(), std::sync::Arc::new("email".to_string()));
        #[derive(Clone)]
        struct ClosureGroup___20 {}
        impl ClosureGroup___20 {
            fn fn__10645(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("email should be in changes".to_string());
            }
        }
        let closure_group = ClosureGroup___20 {};
        let fn__10645 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__10645())
        };
        test___24.assert(t___10660, fn__10645.clone());
        let mut t___10666: bool = ! temper_core::MappedTrait::has( & cs__653.changes(), std::sync::Arc::new("admin".to_string()));
        #[derive(Clone)]
        struct ClosureGroup___21 {}
        impl ClosureGroup___21 {
            fn fn__10644(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("admin must be dropped (not in whitelist)".to_string());
            }
        }
        let closure_group = ClosureGroup___21 {};
        let fn__10644 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__10644())
        };
        test___24.assert(t___10666, fn__10644.clone());
        let mut t___10668: bool = cs__653.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___22 {}
        impl ClosureGroup___22 {
            fn fn__10643(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("should still be valid".to_string());
            }
        }
        let closure_group = ClosureGroup___22 {};
        let fn__10643 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__10643())
        };
        test___24.assert(t___10668, fn__10643.clone());
        test___24.soft_fail_to_hard()
    }
    #[test]
    fn castIsReplacingNotAdditiveSecondCallResetsWhitelist__1547() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___25 = temper_std::testing::Test::new();
        let params__655: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("name".to_string()), std::sync::Arc::new("Alice".to_string())), (std::sync::Arc::new("email".to_string()), std::sync::Arc::new("alice@example.com".to_string()))]);
        let mut t___10629: TableDef = userTable__504();
        let mut t___10630: SafeIdentifier = csid__503("name");
        let cs__656: Changeset = changeset(t___10629.clone(), params__655.clone()).cast(std::sync::Arc::new(vec![t___10630.clone()])).cast(std::sync::Arc::new(vec![csid__503("email")]));
        let mut t___10637: bool = ! temper_core::MappedTrait::has( & cs__656.changes(), std::sync::Arc::new("name".to_string()));
        #[derive(Clone)]
        struct ClosureGroup___23 {}
        impl ClosureGroup___23 {
            fn fn__10625(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("name must be excluded by second cast".to_string());
            }
        }
        let closure_group = ClosureGroup___23 {};
        let fn__10625 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__10625())
        };
        test___25.assert(t___10637, fn__10625.clone());
        let mut t___10640: bool = temper_core::MappedTrait::has( & cs__656.changes(), std::sync::Arc::new("email".to_string()));
        #[derive(Clone)]
        struct ClosureGroup___24 {}
        impl ClosureGroup___24 {
            fn fn__10624(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("email should be present".to_string());
            }
        }
        let closure_group = ClosureGroup___24 {};
        let fn__10624 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__10624())
        };
        test___25.assert(t___10640, fn__10624.clone());
        test___25.soft_fail_to_hard()
    }
    #[test]
    fn castIgnoresEmptyStringValues__1548() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___26 = temper_std::testing::Test::new();
        let params__658: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("name".to_string()), std::sync::Arc::new("".to_string())), (std::sync::Arc::new("email".to_string()), std::sync::Arc::new("bob@example.com".to_string()))]);
        let mut t___10611: TableDef = userTable__504();
        let mut t___10612: SafeIdentifier = csid__503("name");
        let mut t___10613: SafeIdentifier = csid__503("email");
        let cs__659: Changeset = changeset(t___10611.clone(), params__658.clone()).cast(std::sync::Arc::new(vec![t___10612.clone(), t___10613.clone()]));
        let mut t___10618: bool = ! temper_core::MappedTrait::has( & cs__659.changes(), std::sync::Arc::new("name".to_string()));
        #[derive(Clone)]
        struct ClosureGroup___25 {}
        impl ClosureGroup___25 {
            fn fn__10607(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("empty name should not be in changes".to_string());
            }
        }
        let closure_group = ClosureGroup___25 {};
        let fn__10607 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__10607())
        };
        test___26.assert(t___10618, fn__10607.clone());
        let mut t___10621: bool = temper_core::MappedTrait::has( & cs__659.changes(), std::sync::Arc::new("email".to_string()));
        #[derive(Clone)]
        struct ClosureGroup___26 {}
        impl ClosureGroup___26 {
            fn fn__10606(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("email should be in changes".to_string());
            }
        }
        let closure_group = ClosureGroup___26 {};
        let fn__10606 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__10606())
        };
        test___26.assert(t___10621, fn__10606.clone());
        test___26.soft_fail_to_hard()
    }
    #[test]
    fn validateRequiredPassesWhenFieldPresent__1549() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___27 = temper_std::testing::Test::new();
        let params__661: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("name".to_string()), std::sync::Arc::new("Alice".to_string()))]);
        let mut t___10593: TableDef = userTable__504();
        let mut t___10594: SafeIdentifier = csid__503("name");
        let cs__662: Changeset = changeset(t___10593.clone(), params__661.clone()).cast(std::sync::Arc::new(vec![t___10594.clone()])).validate_required(std::sync::Arc::new(vec![csid__503("name")]));
        let mut t___10598: bool = cs__662.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___27 {}
        impl ClosureGroup___27 {
            fn fn__10590(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("should be valid".to_string());
            }
        }
        let closure_group = ClosureGroup___27 {};
        let fn__10590 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__10590())
        };
        test___27.assert(t___10598, fn__10590.clone());
        let mut t___10604: bool = Some(temper_core::ListedTrait::len( & cs__662.errors())) == Some(0);
        #[derive(Clone)]
        struct ClosureGroup___28 {}
        impl ClosureGroup___28 {
            fn fn__10589(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("no errors expected".to_string());
            }
        }
        let closure_group = ClosureGroup___28 {};
        let fn__10589 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__10589())
        };
        test___27.assert(t___10604, fn__10589.clone());
        test___27.soft_fail_to_hard()
    }
    #[test]
    fn validateRequiredFailsWhenFieldMissing__1550() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___28 = temper_std::testing::Test::new();
        let params__664: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & []);
        let mut t___10569: TableDef = userTable__504();
        let mut t___10570: SafeIdentifier = csid__503("name");
        let cs__665: Changeset = changeset(t___10569.clone(), params__664.clone()).cast(std::sync::Arc::new(vec![t___10570.clone()])).validate_required(std::sync::Arc::new(vec![csid__503("name")]));
        let mut t___10576: bool = ! cs__665.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___29 {}
        impl ClosureGroup___29 {
            fn fn__10567(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("should be invalid".to_string());
            }
        }
        let closure_group = ClosureGroup___29 {};
        let fn__10567 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__10567())
        };
        test___28.assert(t___10576, fn__10567.clone());
        let mut t___10581: bool = Some(temper_core::ListedTrait::len( & cs__665.errors())) == Some(1);
        #[derive(Clone)]
        struct ClosureGroup___30 {}
        impl ClosureGroup___30 {
            fn fn__10566(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("should have one error".to_string());
            }
        }
        let closure_group = ClosureGroup___30 {};
        let fn__10566 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__10566())
        };
        test___28.assert(t___10581, fn__10566.clone());
        let mut t___10587: bool = Some(temper_core::ListedTrait::get( & cs__665.errors(), 0).field().as_str()) == Some("name");
        #[derive(Clone)]
        struct ClosureGroup___31 {}
        impl ClosureGroup___31 {
            fn fn__10565(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("error should name the field".to_string());
            }
        }
        let closure_group = ClosureGroup___31 {};
        let fn__10565 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__10565())
        };
        test___28.assert(t___10587, fn__10565.clone());
        test___28.soft_fail_to_hard()
    }
    #[test]
    fn validateLengthPassesWithinRange__1551() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___29 = temper_std::testing::Test::new();
        let params__667: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("name".to_string()), std::sync::Arc::new("Alice".to_string()))]);
        let mut t___10557: TableDef = userTable__504();
        let mut t___10558: SafeIdentifier = csid__503("name");
        let cs__668: Changeset = changeset(t___10557.clone(), params__667.clone()).cast(std::sync::Arc::new(vec![t___10558.clone()])).validate_length(csid__503("name"), 2, 50);
        let mut t___10562: bool = cs__668.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___32 {}
        impl ClosureGroup___32 {
            fn fn__10554(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("should be valid".to_string());
            }
        }
        let closure_group = ClosureGroup___32 {};
        let fn__10554 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__10554())
        };
        test___29.assert(t___10562, fn__10554.clone());
        test___29.soft_fail_to_hard()
    }
    #[test]
    fn validateLengthFailsWhenTooShort__1552() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___30 = temper_std::testing::Test::new();
        let params__670: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("name".to_string()), std::sync::Arc::new("A".to_string()))]);
        let mut t___10545: TableDef = userTable__504();
        let mut t___10546: SafeIdentifier = csid__503("name");
        let cs__671: Changeset = changeset(t___10545.clone(), params__670.clone()).cast(std::sync::Arc::new(vec![t___10546.clone()])).validate_length(csid__503("name"), 2, 50);
        let mut t___10552: bool = ! cs__671.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___33 {}
        impl ClosureGroup___33 {
            fn fn__10542(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("should be invalid".to_string());
            }
        }
        let closure_group = ClosureGroup___33 {};
        let fn__10542 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__10542())
        };
        test___30.assert(t___10552, fn__10542.clone());
        test___30.soft_fail_to_hard()
    }
    #[test]
    fn validateLengthFailsWhenTooLong__1553() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___31 = temper_std::testing::Test::new();
        let params__673: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("name".to_string()), std::sync::Arc::new("ABCDEFGHIJKLMNOPQRSTUVWXYZ".to_string()))]);
        let mut t___10533: TableDef = userTable__504();
        let mut t___10534: SafeIdentifier = csid__503("name");
        let cs__674: Changeset = changeset(t___10533.clone(), params__673.clone()).cast(std::sync::Arc::new(vec![t___10534.clone()])).validate_length(csid__503("name"), 2, 10);
        let mut t___10540: bool = ! cs__674.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___34 {}
        impl ClosureGroup___34 {
            fn fn__10530(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("should be invalid".to_string());
            }
        }
        let closure_group = ClosureGroup___34 {};
        let fn__10530 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__10530())
        };
        test___31.assert(t___10540, fn__10530.clone());
        test___31.soft_fail_to_hard()
    }
    #[test]
    fn validateIntPassesForValidInteger__1554() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___32 = temper_std::testing::Test::new();
        let params__676: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("age".to_string()), std::sync::Arc::new("30".to_string()))]);
        let mut t___10522: TableDef = userTable__504();
        let mut t___10523: SafeIdentifier = csid__503("age");
        let cs__677: Changeset = changeset(t___10522.clone(), params__676.clone()).cast(std::sync::Arc::new(vec![t___10523.clone()])).validate_int(csid__503("age"));
        let mut t___10527: bool = cs__677.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___35 {}
        impl ClosureGroup___35 {
            fn fn__10519(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("should be valid".to_string());
            }
        }
        let closure_group = ClosureGroup___35 {};
        let fn__10519 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__10519())
        };
        test___32.assert(t___10527, fn__10519.clone());
        test___32.soft_fail_to_hard()
    }
    #[test]
    fn validateIntFailsForNonInteger__1555() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___33 = temper_std::testing::Test::new();
        let params__679: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("age".to_string()), std::sync::Arc::new("not-a-number".to_string()))]);
        let mut t___10510: TableDef = userTable__504();
        let mut t___10511: SafeIdentifier = csid__503("age");
        let cs__680: Changeset = changeset(t___10510.clone(), params__679.clone()).cast(std::sync::Arc::new(vec![t___10511.clone()])).validate_int(csid__503("age"));
        let mut t___10517: bool = ! cs__680.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___36 {}
        impl ClosureGroup___36 {
            fn fn__10507(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("should be invalid".to_string());
            }
        }
        let closure_group = ClosureGroup___36 {};
        let fn__10507 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__10507())
        };
        test___33.assert(t___10517, fn__10507.clone());
        test___33.soft_fail_to_hard()
    }
    #[test]
    fn validateFloatPassesForValidFloat__1556() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___34 = temper_std::testing::Test::new();
        let params__682: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("score".to_string()), std::sync::Arc::new("9.5".to_string()))]);
        let mut t___10499: TableDef = userTable__504();
        let mut t___10500: SafeIdentifier = csid__503("score");
        let cs__683: Changeset = changeset(t___10499.clone(), params__682.clone()).cast(std::sync::Arc::new(vec![t___10500.clone()])).validate_float(csid__503("score"));
        let mut t___10504: bool = cs__683.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___37 {}
        impl ClosureGroup___37 {
            fn fn__10496(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("should be valid".to_string());
            }
        }
        let closure_group = ClosureGroup___37 {};
        let fn__10496 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__10496())
        };
        test___34.assert(t___10504, fn__10496.clone());
        test___34.soft_fail_to_hard()
    }
    #[test]
    fn validateInt64_passesForValid64_bitInteger__1557() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___35 = temper_std::testing::Test::new();
        let params__685: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("age".to_string()), std::sync::Arc::new("9999999999".to_string()))]);
        let mut t___10488: TableDef = userTable__504();
        let mut t___10489: SafeIdentifier = csid__503("age");
        let cs__686: Changeset = changeset(t___10488.clone(), params__685.clone()).cast(std::sync::Arc::new(vec![t___10489.clone()])).validate_int64(csid__503("age"));
        let mut t___10493: bool = cs__686.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___38 {}
        impl ClosureGroup___38 {
            fn fn__10485(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("should be valid".to_string());
            }
        }
        let closure_group = ClosureGroup___38 {};
        let fn__10485 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__10485())
        };
        test___35.assert(t___10493, fn__10485.clone());
        test___35.soft_fail_to_hard()
    }
    #[test]
    fn validateInt64_failsForNonInteger__1558() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___36 = temper_std::testing::Test::new();
        let params__688: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("age".to_string()), std::sync::Arc::new("not-a-number".to_string()))]);
        let mut t___10476: TableDef = userTable__504();
        let mut t___10477: SafeIdentifier = csid__503("age");
        let cs__689: Changeset = changeset(t___10476.clone(), params__688.clone()).cast(std::sync::Arc::new(vec![t___10477.clone()])).validate_int64(csid__503("age"));
        let mut t___10483: bool = ! cs__689.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___39 {}
        impl ClosureGroup___39 {
            fn fn__10473(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("should be invalid".to_string());
            }
        }
        let closure_group = ClosureGroup___39 {};
        let fn__10473 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__10473())
        };
        test___36.assert(t___10483, fn__10473.clone());
        test___36.soft_fail_to_hard()
    }
    #[test]
    fn validateBoolAcceptsTrue1_yesOn__1559() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___37 = temper_std::testing::Test::new();
        #[derive(Clone)]
        struct ClosureGroup___40 {
            test___37: temper_std::testing::Test
        }
        impl ClosureGroup___40 {
            fn fn__10470(& self, v__691: impl temper_core::ToArcString) {
                let v__691 = v__691.to_arc_string();
                let params__692: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("active".to_string()), v__691.clone())]);
                let mut t___10462: TableDef = userTable__504();
                let mut t___10463: SafeIdentifier = csid__503("active");
                let cs__693: Changeset = changeset(t___10462.clone(), params__692.clone()).cast(std::sync::Arc::new(vec![t___10463.clone()])).validate_bool(csid__503("active"));
                let mut t___10467: bool = cs__693.is_valid();
                #[derive(Clone)]
                struct ClosureGroup___41 {
                    v__691: std::sync::Arc<String>
                }
                impl ClosureGroup___41 {
                    fn fn__10459(& self) -> std::sync::Arc<String> {
                        return std::sync::Arc::new(format!("should accept: {}", self.v__691.clone()));
                    }
                }
                let closure_group = ClosureGroup___41 {
                    v__691: v__691.clone()
                };
                let fn__10459 = {
                    let closure_group = closure_group.clone();
                    std::sync::Arc::new(move | | closure_group.fn__10459())
                };
                self.test___37.assert(t___10467, fn__10459.clone());
            }
        }
        let closure_group = ClosureGroup___40 {
            test___37: test___37.clone()
        };
        let fn__10470 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | v__691: std::sync::Arc<String> | closure_group.fn__10470(v__691))
        };
        temper_core::listed::list_for_each( & std::sync::Arc::new(vec![std::sync::Arc::new("true".to_string()), std::sync::Arc::new("1".to_string()), std::sync::Arc::new("yes".to_string()), std::sync::Arc::new("on".to_string())]), & ( * fn__10470.clone()));
        test___37.soft_fail_to_hard()
    }
    #[test]
    fn validateBoolAcceptsFalse0_noOff__1560() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___38 = temper_std::testing::Test::new();
        #[derive(Clone)]
        struct ClosureGroup___42 {
            test___38: temper_std::testing::Test
        }
        impl ClosureGroup___42 {
            fn fn__10456(& self, v__695: impl temper_core::ToArcString) {
                let v__695 = v__695.to_arc_string();
                let params__696: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("active".to_string()), v__695.clone())]);
                let mut t___10448: TableDef = userTable__504();
                let mut t___10449: SafeIdentifier = csid__503("active");
                let cs__697: Changeset = changeset(t___10448.clone(), params__696.clone()).cast(std::sync::Arc::new(vec![t___10449.clone()])).validate_bool(csid__503("active"));
                let mut t___10453: bool = cs__697.is_valid();
                #[derive(Clone)]
                struct ClosureGroup___43 {
                    v__695: std::sync::Arc<String>
                }
                impl ClosureGroup___43 {
                    fn fn__10445(& self) -> std::sync::Arc<String> {
                        return std::sync::Arc::new(format!("should accept: {}", self.v__695.clone()));
                    }
                }
                let closure_group = ClosureGroup___43 {
                    v__695: v__695.clone()
                };
                let fn__10445 = {
                    let closure_group = closure_group.clone();
                    std::sync::Arc::new(move | | closure_group.fn__10445())
                };
                self.test___38.assert(t___10453, fn__10445.clone());
            }
        }
        let closure_group = ClosureGroup___42 {
            test___38: test___38.clone()
        };
        let fn__10456 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | v__695: std::sync::Arc<String> | closure_group.fn__10456(v__695))
        };
        temper_core::listed::list_for_each( & std::sync::Arc::new(vec![std::sync::Arc::new("false".to_string()), std::sync::Arc::new("0".to_string()), std::sync::Arc::new("no".to_string()), std::sync::Arc::new("off".to_string())]), & ( * fn__10456.clone()));
        test___38.soft_fail_to_hard()
    }
    #[test]
    fn validateBoolRejectsAmbiguousValues__1561() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___39 = temper_std::testing::Test::new();
        #[derive(Clone)]
        struct ClosureGroup___44 {
            test___39: temper_std::testing::Test
        }
        impl ClosureGroup___44 {
            fn fn__10442(& self, v__699: impl temper_core::ToArcString) {
                let v__699 = v__699.to_arc_string();
                let params__700: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("active".to_string()), v__699.clone())]);
                let mut t___10433: TableDef = userTable__504();
                let mut t___10434: SafeIdentifier = csid__503("active");
                let cs__701: Changeset = changeset(t___10433.clone(), params__700.clone()).cast(std::sync::Arc::new(vec![t___10434.clone()])).validate_bool(csid__503("active"));
                let mut t___10440: bool = ! cs__701.is_valid();
                #[derive(Clone)]
                struct ClosureGroup___45 {
                    v__699: std::sync::Arc<String>
                }
                impl ClosureGroup___45 {
                    fn fn__10430(& self) -> std::sync::Arc<String> {
                        return std::sync::Arc::new(format!("should reject ambiguous: {}", self.v__699.clone()));
                    }
                }
                let closure_group = ClosureGroup___45 {
                    v__699: v__699.clone()
                };
                let fn__10430 = {
                    let closure_group = closure_group.clone();
                    std::sync::Arc::new(move | | closure_group.fn__10430())
                };
                self.test___39.assert(t___10440, fn__10430.clone());
            }
        }
        let closure_group = ClosureGroup___44 {
            test___39: test___39.clone()
        };
        let fn__10442 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | v__699: std::sync::Arc<String> | closure_group.fn__10442(v__699))
        };
        temper_core::listed::list_for_each( & std::sync::Arc::new(vec![std::sync::Arc::new("TRUE".to_string()), std::sync::Arc::new("Yes".to_string()), std::sync::Arc::new("maybe".to_string()), std::sync::Arc::new("2".to_string()), std::sync::Arc::new("enabled".to_string())]), & ( * fn__10442.clone()));
        test___39.soft_fail_to_hard()
    }
    #[test]
    fn toInsertSqlEscapesBobbyTables__1562() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___40 = temper_std::testing::Test::new();
        let mut t___5833: SqlFragment;
        let params__703: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("name".to_string()), std::sync::Arc::new("Robert'); DROP TABLE users;--".to_string())), (std::sync::Arc::new("email".to_string()), std::sync::Arc::new("bobby@evil.com".to_string()))]);
        let mut t___10418: TableDef = userTable__504();
        let mut t___10419: SafeIdentifier = csid__503("name");
        let mut t___10420: SafeIdentifier = csid__503("email");
        let cs__704: Changeset = changeset(t___10418.clone(), params__703.clone()).cast(std::sync::Arc::new(vec![t___10419.clone(), t___10420.clone()])).validate_required(std::sync::Arc::new(vec![csid__503("name"), csid__503("email")]));
        let sqlFrag__705: SqlFragment;
        'ok___10970: {
            'orelse___1897: {
                t___5833 = match cs__704.to_insert_sql() {
                    Ok(x) => x,
                    _ => break 'orelse___1897
                };
                sqlFrag__705 = t___5833.clone();
                break 'ok___10970;
            }
            sqlFrag__705 = panic!();
        }
        let s__706: std::sync::Arc<String> = sqlFrag__705.to_string();
        let mut t___10427: bool = temper_core::string::index_of( & s__706, "''", None).is_some();
        #[derive(Clone)]
        struct ClosureGroup___46 {
            s__706: std::sync::Arc<String>
        }
        impl ClosureGroup___46 {
            fn fn__10414(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("single quote must be doubled: {}", self.s__706.clone()));
            }
        }
        let closure_group = ClosureGroup___46 {
            s__706: s__706.clone()
        };
        let fn__10414 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__10414())
        };
        test___40.assert(t___10427, fn__10414.clone());
        test___40.soft_fail_to_hard()
    }
    #[test]
    fn toInsertSqlProducesCorrectSqlForStringField__1563() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___41 = temper_std::testing::Test::new();
        let mut t___5812: SqlFragment;
        let params__708: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("name".to_string()), std::sync::Arc::new("Alice".to_string())), (std::sync::Arc::new("email".to_string()), std::sync::Arc::new("a@example.com".to_string()))]);
        let mut t___10398: TableDef = userTable__504();
        let mut t___10399: SafeIdentifier = csid__503("name");
        let mut t___10400: SafeIdentifier = csid__503("email");
        let cs__709: Changeset = changeset(t___10398.clone(), params__708.clone()).cast(std::sync::Arc::new(vec![t___10399.clone(), t___10400.clone()])).validate_required(std::sync::Arc::new(vec![csid__503("name"), csid__503("email")]));
        let sqlFrag__710: SqlFragment;
        'ok___10973: {
            'orelse___1898: {
                t___5812 = match cs__709.to_insert_sql() {
                    Ok(x) => x,
                    _ => break 'orelse___1898
                };
                sqlFrag__710 = t___5812.clone();
                break 'ok___10973;
            }
            sqlFrag__710 = panic!();
        }
        let s__711: std::sync::Arc<String> = sqlFrag__710.to_string();
        let mut t___10407: bool = temper_core::string::index_of( & s__711, "INSERT INTO users", None).is_some();
        #[derive(Clone)]
        struct ClosureGroup___47 {
            s__711: std::sync::Arc<String>
        }
        impl ClosureGroup___47 {
            fn fn__10394(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("has INSERT INTO: {}", self.s__711.clone()));
            }
        }
        let closure_group = ClosureGroup___47 {
            s__711: s__711.clone()
        };
        let fn__10394 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__10394())
        };
        test___41.assert(t___10407, fn__10394.clone());
        let mut t___10411: bool = temper_core::string::index_of( & s__711, "'Alice'", None).is_some();
        #[derive(Clone)]
        struct ClosureGroup___48 {
            s__711: std::sync::Arc<String>
        }
        impl ClosureGroup___48 {
            fn fn__10393(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("has quoted name: {}", self.s__711.clone()));
            }
        }
        let closure_group = ClosureGroup___48 {
            s__711: s__711.clone()
        };
        let fn__10393 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__10393())
        };
        test___41.assert(t___10411, fn__10393.clone());
        test___41.soft_fail_to_hard()
    }
    #[test]
    fn toInsertSqlProducesCorrectSqlForIntField__1564() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___42 = temper_std::testing::Test::new();
        let mut t___5795: SqlFragment;
        let params__713: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("name".to_string()), std::sync::Arc::new("Bob".to_string())), (std::sync::Arc::new("email".to_string()), std::sync::Arc::new("b@example.com".to_string())), (std::sync::Arc::new("age".to_string()), std::sync::Arc::new("25".to_string()))]);
        let mut t___10380: TableDef = userTable__504();
        let mut t___10381: SafeIdentifier = csid__503("name");
        let mut t___10382: SafeIdentifier = csid__503("email");
        let mut t___10383: SafeIdentifier = csid__503("age");
        let cs__714: Changeset = changeset(t___10380.clone(), params__713.clone()).cast(std::sync::Arc::new(vec![t___10381.clone(), t___10382.clone(), t___10383.clone()])).validate_required(std::sync::Arc::new(vec![csid__503("name"), csid__503("email")]));
        let sqlFrag__715: SqlFragment;
        'ok___10974: {
            'orelse___1899: {
                t___5795 = match cs__714.to_insert_sql() {
                    Ok(x) => x,
                    _ => break 'orelse___1899
                };
                sqlFrag__715 = t___5795.clone();
                break 'ok___10974;
            }
            sqlFrag__715 = panic!();
        }
        let s__716: std::sync::Arc<String> = sqlFrag__715.to_string();
        let mut t___10390: bool = temper_core::string::index_of( & s__716, "25", None).is_some();
        #[derive(Clone)]
        struct ClosureGroup___49 {
            s__716: std::sync::Arc<String>
        }
        impl ClosureGroup___49 {
            fn fn__10375(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("age rendered unquoted: {}", self.s__716.clone()));
            }
        }
        let closure_group = ClosureGroup___49 {
            s__716: s__716.clone()
        };
        let fn__10375 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__10375())
        };
        test___42.assert(t___10390, fn__10375.clone());
        test___42.soft_fail_to_hard()
    }
    #[test]
    fn toInsertSqlBubblesOnInvalidChangeset__1565() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___43 = temper_std::testing::Test::new();
        let params__718: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & []);
        let mut t___10368: TableDef = userTable__504();
        let mut t___10369: SafeIdentifier = csid__503("name");
        let cs__719: Changeset = changeset(t___10368.clone(), params__718.clone()).cast(std::sync::Arc::new(vec![t___10369.clone()])).validate_required(std::sync::Arc::new(vec![csid__503("name")]));
        let didBubble__720: bool;
        'ok___10975: {
            'orelse___1900: {
                match cs__719.to_insert_sql() {
                    Ok(x) => x,
                    _ => break 'orelse___1900
                };
                didBubble__720 = false;
                break 'ok___10975;
            }
            didBubble__720 = true;
        }
        #[derive(Clone)]
        struct ClosureGroup___50 {}
        impl ClosureGroup___50 {
            fn fn__10366(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("invalid changeset should bubble".to_string());
            }
        }
        let closure_group = ClosureGroup___50 {};
        let fn__10366 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__10366())
        };
        test___43.assert(didBubble__720, fn__10366.clone());
        test___43.soft_fail_to_hard()
    }
    #[test]
    fn toInsertSqlEnforcesNonNullableFieldsIndependentlyOfIsValid__1566() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___44 = temper_std::testing::Test::new();
        let strictTable__722: TableDef = TableDef::new(csid__503("posts"), [FieldDef::new(csid__503("title"), FieldType::new(StringField::new()), false), FieldDef::new(csid__503("body"), FieldType::new(StringField::new()), true)]);
        let params__723: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("body".to_string()), std::sync::Arc::new("hello".to_string()))]);
        let mut t___10359: SafeIdentifier = csid__503("body");
        let cs__724: Changeset = changeset(strictTable__722.clone(), params__723.clone()).cast(std::sync::Arc::new(vec![t___10359.clone()]));
        let mut t___10361: bool = cs__724.is_valid();
        #[derive(Clone)]
        struct ClosureGroup___51 {}
        impl ClosureGroup___51 {
            fn fn__10348(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("changeset should appear valid (no explicit validation run)".to_string());
            }
        }
        let closure_group = ClosureGroup___51 {};
        let fn__10348 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__10348())
        };
        test___44.assert(t___10361, fn__10348.clone());
        let didBubble__725: bool;
        'ok___10976: {
            'orelse___1901: {
                match cs__724.to_insert_sql() {
                    Ok(x) => x,
                    _ => break 'orelse___1901
                };
                didBubble__725 = false;
                break 'ok___10976;
            }
            didBubble__725 = true;
        }
        #[derive(Clone)]
        struct ClosureGroup___52 {}
        impl ClosureGroup___52 {
            fn fn__10347(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("toInsertSql should enforce nullable regardless of isValid".to_string());
            }
        }
        let closure_group = ClosureGroup___52 {};
        let fn__10347 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__10347())
        };
        test___44.assert(didBubble__725, fn__10347.clone());
        test___44.soft_fail_to_hard()
    }
    #[test]
    fn toUpdateSqlProducesCorrectSql__1567() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___45 = temper_std::testing::Test::new();
        let mut t___5755: SqlFragment;
        let params__727: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & [(std::sync::Arc::new("name".to_string()), std::sync::Arc::new("Bob".to_string()))]);
        let mut t___10338: TableDef = userTable__504();
        let mut t___10339: SafeIdentifier = csid__503("name");
        let cs__728: Changeset = changeset(t___10338.clone(), params__727.clone()).cast(std::sync::Arc::new(vec![t___10339.clone()])).validate_required(std::sync::Arc::new(vec![csid__503("name")]));
        let sqlFrag__729: SqlFragment;
        'ok___10977: {
            'orelse___1902: {
                t___5755 = match cs__728.to_update_sql(42) {
                    Ok(x) => x,
                    _ => break 'orelse___1902
                };
                sqlFrag__729 = t___5755.clone();
                break 'ok___10977;
            }
            sqlFrag__729 = panic!();
        }
        let s__730: std::sync::Arc<String> = sqlFrag__729.to_string();
        let mut t___10345: bool = Some(s__730.as_str()) == Some("UPDATE users SET name = 'Bob' WHERE id = 42");
        #[derive(Clone)]
        struct ClosureGroup___53 {
            s__730: std::sync::Arc<String>
        }
        impl ClosureGroup___53 {
            fn fn__10335(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("got: {}", self.s__730.clone()));
            }
        }
        let closure_group = ClosureGroup___53 {
            s__730: s__730.clone()
        };
        let fn__10335 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__10335())
        };
        test___45.assert(t___10345, fn__10335.clone());
        test___45.soft_fail_to_hard()
    }
    #[test]
    fn toUpdateSqlBubblesOnInvalidChangeset__1568() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___46 = temper_std::testing::Test::new();
        let params__732: temper_core::Map<std::sync::Arc<String>, std::sync::Arc<String>> = temper_core::Map::new( & []);
        let mut t___10328: TableDef = userTable__504();
        let mut t___10329: SafeIdentifier = csid__503("name");
        let cs__733: Changeset = changeset(t___10328.clone(), params__732.clone()).cast(std::sync::Arc::new(vec![t___10329.clone()])).validate_required(std::sync::Arc::new(vec![csid__503("name")]));
        let didBubble__734: bool;
        'ok___10978: {
            'orelse___1903: {
                match cs__733.to_update_sql(1) {
                    Ok(x) => x,
                    _ => break 'orelse___1903
                };
                didBubble__734 = false;
                break 'ok___10978;
            }
            didBubble__734 = true;
        }
        #[derive(Clone)]
        struct ClosureGroup___54 {}
        impl ClosureGroup___54 {
            fn fn__10326(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("invalid changeset should bubble".to_string());
            }
        }
        let closure_group = ClosureGroup___54 {};
        let fn__10326 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__10326())
        };
        test___46.assert(didBubble__734, fn__10326.clone());
        test___46.soft_fail_to_hard()
    }
    #[test]
    fn bareFromProducesSelect__1644() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___47 = temper_std::testing::Test::new();
        let q__1042: Query = from(sid__505("users"));
        let mut t___9837: bool = Some(q__1042.to_sql().to_string().as_str()) == Some("SELECT * FROM users");
        #[derive(Clone)]
        struct ClosureGroup___55 {}
        impl ClosureGroup___55 {
            fn fn__9832(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("bare query".to_string());
            }
        }
        let closure_group = ClosureGroup___55 {};
        let fn__9832 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__9832())
        };
        test___47.assert(t___9837, fn__9832.clone());
        test___47.soft_fail_to_hard()
    }
    #[test]
    fn selectRestrictsColumns__1645() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___48 = temper_std::testing::Test::new();
        let mut t___9823: SafeIdentifier = sid__505("users");
        let mut t___9824: SafeIdentifier = sid__505("id");
        let mut t___9825: SafeIdentifier = sid__505("name");
        let q__1044: Query = from(t___9823.clone()).select([t___9824.clone(), t___9825.clone()]);
        let mut t___9830: bool = Some(q__1044.to_sql().to_string().as_str()) == Some("SELECT id, name FROM users");
        #[derive(Clone)]
        struct ClosureGroup___56 {}
        impl ClosureGroup___56 {
            fn fn__9822(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("select columns".to_string());
            }
        }
        let closure_group = ClosureGroup___56 {};
        let fn__9822 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__9822())
        };
        test___48.assert(t___9830, fn__9822.clone());
        test___48.soft_fail_to_hard()
    }
    #[test]
    fn whereAddsConditionWithIntValue__1646() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___49 = temper_std::testing::Test::new();
        let mut t___9811: SafeIdentifier = sid__505("users");
        let mut t___9812: SqlBuilder = SqlBuilder::new();
        t___9812.append_safe("age > ");
        t___9812.append_int32(18);
        let mut t___9815: SqlFragment = t___9812.accumulated();
        let q__1046: Query = from(t___9811.clone()).r#where(t___9815.clone());
        let mut t___9820: bool = Some(q__1046.to_sql().to_string().as_str()) == Some("SELECT * FROM users WHERE age > 18");
        #[derive(Clone)]
        struct ClosureGroup___57 {}
        impl ClosureGroup___57 {
            fn fn__9810(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("where int".to_string());
            }
        }
        let closure_group = ClosureGroup___57 {};
        let fn__9810 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__9810())
        };
        test___49.assert(t___9820, fn__9810.clone());
        test___49.soft_fail_to_hard()
    }
    #[test]
    fn whereAddsConditionWithBoolValue__1648() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___50 = temper_std::testing::Test::new();
        let mut t___9799: SafeIdentifier = sid__505("users");
        let mut t___9800: SqlBuilder = SqlBuilder::new();
        t___9800.append_safe("active = ");
        t___9800.append_boolean(true);
        let mut t___9803: SqlFragment = t___9800.accumulated();
        let q__1048: Query = from(t___9799.clone()).r#where(t___9803.clone());
        let mut t___9808: bool = Some(q__1048.to_sql().to_string().as_str()) == Some("SELECT * FROM users WHERE active = TRUE");
        #[derive(Clone)]
        struct ClosureGroup___58 {}
        impl ClosureGroup___58 {
            fn fn__9798(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("where bool".to_string());
            }
        }
        let closure_group = ClosureGroup___58 {};
        let fn__9798 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__9798())
        };
        test___50.assert(t___9808, fn__9798.clone());
        test___50.soft_fail_to_hard()
    }
    #[test]
    fn chainedWhereUsesAnd__1650() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___51 = temper_std::testing::Test::new();
        let mut t___9782: SafeIdentifier = sid__505("users");
        let mut t___9783: SqlBuilder = SqlBuilder::new();
        t___9783.append_safe("age > ");
        t___9783.append_int32(18);
        let mut t___9786: SqlFragment = t___9783.accumulated();
        let mut t___9787: Query = from(t___9782.clone()).r#where(t___9786.clone());
        let mut t___9788: SqlBuilder = SqlBuilder::new();
        t___9788.append_safe("active = ");
        t___9788.append_boolean(true);
        let q__1050: Query = t___9787.r#where(t___9788.accumulated());
        let mut t___9796: bool = Some(q__1050.to_sql().to_string().as_str()) == Some("SELECT * FROM users WHERE age > 18 AND active = TRUE");
        #[derive(Clone)]
        struct ClosureGroup___59 {}
        impl ClosureGroup___59 {
            fn fn__9781(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("chained where".to_string());
            }
        }
        let closure_group = ClosureGroup___59 {};
        let fn__9781 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__9781())
        };
        test___51.assert(t___9796, fn__9781.clone());
        test___51.soft_fail_to_hard()
    }
    #[test]
    fn orderByAsc__1653() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___52 = temper_std::testing::Test::new();
        let mut t___9773: SafeIdentifier = sid__505("users");
        let mut t___9774: SafeIdentifier = sid__505("name");
        let q__1052: Query = from(t___9773.clone()).order_by(t___9774.clone(), true);
        let mut t___9779: bool = Some(q__1052.to_sql().to_string().as_str()) == Some("SELECT * FROM users ORDER BY name ASC");
        #[derive(Clone)]
        struct ClosureGroup___60 {}
        impl ClosureGroup___60 {
            fn fn__9772(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("order asc".to_string());
            }
        }
        let closure_group = ClosureGroup___60 {};
        let fn__9772 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__9772())
        };
        test___52.assert(t___9779, fn__9772.clone());
        test___52.soft_fail_to_hard()
    }
    #[test]
    fn orderByDesc__1654() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___53 = temper_std::testing::Test::new();
        let mut t___9764: SafeIdentifier = sid__505("users");
        let mut t___9765: SafeIdentifier = sid__505("created_at");
        let q__1054: Query = from(t___9764.clone()).order_by(t___9765.clone(), false);
        let mut t___9770: bool = Some(q__1054.to_sql().to_string().as_str()) == Some("SELECT * FROM users ORDER BY created_at DESC");
        #[derive(Clone)]
        struct ClosureGroup___61 {}
        impl ClosureGroup___61 {
            fn fn__9763(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("order desc".to_string());
            }
        }
        let closure_group = ClosureGroup___61 {};
        let fn__9763 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__9763())
        };
        test___53.assert(t___9770, fn__9763.clone());
        test___53.soft_fail_to_hard()
    }
    #[test]
    fn limitAndOffset__1655() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___54 = temper_std::testing::Test::new();
        let mut t___5175: Query;
        let mut t___5176: Query;
        let q__1056: Query;
        'ok___10980: {
            'orelse___1905: {
                t___5175 = match from(sid__505("users")).limit(10) {
                    Ok(x) => x,
                    _ => break 'orelse___1905
                };
                t___5176 = match t___5175.offset(20) {
                    Ok(x) => x,
                    _ => break 'orelse___1905
                };
                q__1056 = t___5176.clone();
                break 'ok___10980;
            }
            q__1056 = panic!();
        }
        let mut t___9761: bool = Some(q__1056.to_sql().to_string().as_str()) == Some("SELECT * FROM users LIMIT 10 OFFSET 20");
        #[derive(Clone)]
        struct ClosureGroup___62 {}
        impl ClosureGroup___62 {
            fn fn__9756(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("limit/offset".to_string());
            }
        }
        let closure_group = ClosureGroup___62 {};
        let fn__9756 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__9756())
        };
        test___54.assert(t___9761, fn__9756.clone());
        test___54.soft_fail_to_hard()
    }
    #[test]
    fn limitBubblesOnNegative__1656() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___55 = temper_std::testing::Test::new();
        let didBubble__1058: bool;
        'ok___10981: {
            'orelse___1906: {
                match from(sid__505("users")).limit(-1) {
                    Ok(x) => x,
                    _ => break 'orelse___1906
                };
                didBubble__1058 = false;
                break 'ok___10981;
            }
            didBubble__1058 = true;
        }
        #[derive(Clone)]
        struct ClosureGroup___63 {}
        impl ClosureGroup___63 {
            fn fn__9752(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("negative limit should bubble".to_string());
            }
        }
        let closure_group = ClosureGroup___63 {};
        let fn__9752 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__9752())
        };
        test___55.assert(didBubble__1058, fn__9752.clone());
        test___55.soft_fail_to_hard()
    }
    #[test]
    fn offsetBubblesOnNegative__1657() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___56 = temper_std::testing::Test::new();
        let didBubble__1060: bool;
        'ok___10982: {
            'orelse___1907: {
                match from(sid__505("users")).offset(-1) {
                    Ok(x) => x,
                    _ => break 'orelse___1907
                };
                didBubble__1060 = false;
                break 'ok___10982;
            }
            didBubble__1060 = true;
        }
        #[derive(Clone)]
        struct ClosureGroup___64 {}
        impl ClosureGroup___64 {
            fn fn__9748(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("negative offset should bubble".to_string());
            }
        }
        let closure_group = ClosureGroup___64 {};
        let fn__9748 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__9748())
        };
        test___56.assert(didBubble__1060, fn__9748.clone());
        test___56.soft_fail_to_hard()
    }
    #[test]
    fn complexComposedQuery__1658() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___57 = temper_std::testing::Test::new();
        let mut t___9726: SafeIdentifier;
        let mut t___9727: SafeIdentifier;
        let mut t___9728: SafeIdentifier;
        let mut t___9729: SafeIdentifier;
        let mut t___9730: Query;
        let mut t___9731: SqlBuilder;
        let mut t___9735: Query;
        let mut t___9736: SqlBuilder;
        let mut t___5161: Query;
        let mut t___5162: Query;
        let minAge__1062: i32 = 21;
        let q__1063: Query;
        'ok___10983: {
            'orelse___1908: {
                t___9726 = sid__505("users");
                t___9727 = sid__505("id");
                t___9728 = sid__505("name");
                t___9729 = sid__505("email");
                t___9730 = from(t___9726.clone()).select([t___9727.clone(), t___9728.clone(), t___9729.clone()]);
                t___9731 = SqlBuilder::new();
                t___9731.append_safe("age >= ");
                t___9731.append_int32(21);
                t___9735 = t___9730.r#where(t___9731.accumulated());
                t___9736 = SqlBuilder::new();
                t___9736.append_safe("active = ");
                t___9736.append_boolean(true);
                t___5161 = match t___9735.r#where(t___9736.accumulated()).order_by(sid__505("name"), true).limit(25) {
                    Ok(x) => x,
                    _ => break 'orelse___1908
                };
                t___5162 = match t___5161.offset(0) {
                    Ok(x) => x,
                    _ => break 'orelse___1908
                };
                q__1063 = t___5162.clone();
                break 'ok___10983;
            }
            q__1063 = panic!();
        }
        let mut t___9746: bool = Some(q__1063.to_sql().to_string().as_str()) == Some("SELECT id, name, email FROM users WHERE age >= 21 AND active = TRUE ORDER BY name ASC LIMIT 25 OFFSET 0");
        #[derive(Clone)]
        struct ClosureGroup___65 {}
        impl ClosureGroup___65 {
            fn fn__9725(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("complex query".to_string());
            }
        }
        let closure_group = ClosureGroup___65 {};
        let fn__9725 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__9725())
        };
        test___57.assert(t___9746, fn__9725.clone());
        test___57.soft_fail_to_hard()
    }
    #[test]
    fn safeToSqlAppliesDefaultLimitWhenNoneSet__1661() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___58 = temper_std::testing::Test::new();
        let mut t___5138: SqlFragment;
        let mut t___5139: SqlFragment;
        let q__1065: Query = from(sid__505("users"));
        'ok___10984: {
            'orelse___1909: {
                t___5138 = match q__1065.safe_to_sql(100) {
                    Ok(x) => x,
                    _ => break 'orelse___1909
                };
                t___5139 = t___5138.clone();
                break 'ok___10984;
            }
            t___5139 = panic!();
        }
        let s__1066: std::sync::Arc<String> = t___5139.to_string();
        let mut t___9723: bool = Some(s__1066.as_str()) == Some("SELECT * FROM users LIMIT 100");
        #[derive(Clone)]
        struct ClosureGroup___66 {
            s__1066: std::sync::Arc<String>
        }
        impl ClosureGroup___66 {
            fn fn__9719(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("should have limit: {}", self.s__1066.clone()));
            }
        }
        let closure_group = ClosureGroup___66 {
            s__1066: s__1066.clone()
        };
        let fn__9719 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__9719())
        };
        test___58.assert(t___9723, fn__9719.clone());
        test___58.soft_fail_to_hard()
    }
    #[test]
    fn safeToSqlRespectsExplicitLimit__1662() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___59 = temper_std::testing::Test::new();
        let mut t___5130: Query;
        let mut t___5133: SqlFragment;
        let mut t___5134: SqlFragment;
        let q__1068: Query;
        'ok___10985: {
            'orelse___1910: {
                t___5130 = match from(sid__505("users")).limit(5) {
                    Ok(x) => x,
                    _ => break 'orelse___1910
                };
                q__1068 = t___5130.clone();
                break 'ok___10985;
            }
            q__1068 = panic!();
        }
        'ok___10986: {
            'orelse___1911: {
                t___5133 = match q__1068.safe_to_sql(100) {
                    Ok(x) => x,
                    _ => break 'orelse___1911
                };
                t___5134 = t___5133.clone();
                break 'ok___10986;
            }
            t___5134 = panic!();
        }
        let s__1069: std::sync::Arc<String> = t___5134.to_string();
        let mut t___9717: bool = Some(s__1069.as_str()) == Some("SELECT * FROM users LIMIT 5");
        #[derive(Clone)]
        struct ClosureGroup___67 {
            s__1069: std::sync::Arc<String>
        }
        impl ClosureGroup___67 {
            fn fn__9713(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("explicit limit preserved: {}", self.s__1069.clone()));
            }
        }
        let closure_group = ClosureGroup___67 {
            s__1069: s__1069.clone()
        };
        let fn__9713 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__9713())
        };
        test___59.assert(t___9717, fn__9713.clone());
        test___59.soft_fail_to_hard()
    }
    #[test]
    fn safeToSqlBubblesOnNegativeDefaultLimit__1663() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___60 = temper_std::testing::Test::new();
        let didBubble__1071: bool;
        'ok___10987: {
            'orelse___1912: {
                match from(sid__505("users")).safe_to_sql(-1) {
                    Ok(x) => x,
                    _ => break 'orelse___1912
                };
                didBubble__1071 = false;
                break 'ok___10987;
            }
            didBubble__1071 = true;
        }
        #[derive(Clone)]
        struct ClosureGroup___68 {}
        impl ClosureGroup___68 {
            fn fn__9709(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("negative defaultLimit should bubble".to_string());
            }
        }
        let closure_group = ClosureGroup___68 {};
        let fn__9709 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__9709())
        };
        test___60.assert(didBubble__1071, fn__9709.clone());
        test___60.soft_fail_to_hard()
    }
    #[test]
    fn whereWithInjectionAttemptInStringValueIsEscaped__1664() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___61 = temper_std::testing::Test::new();
        let evil__1073: std::sync::Arc<String> = std::sync::Arc::new("'; DROP TABLE users; --".to_string());
        let mut t___9693: SafeIdentifier = sid__505("users");
        let mut t___9694: SqlBuilder = SqlBuilder::new();
        t___9694.append_safe("name = ");
        t___9694.append_string("'; DROP TABLE users; --");
        let mut t___9697: SqlFragment = t___9694.accumulated();
        let q__1074: Query = from(t___9693.clone()).r#where(t___9697.clone());
        let s__1075: std::sync::Arc<String> = q__1074.to_sql().to_string();
        let mut t___9702: bool = temper_core::string::index_of( & s__1075, "''", None).is_some();
        #[derive(Clone)]
        struct ClosureGroup___69 {
            s__1075: std::sync::Arc<String>
        }
        impl ClosureGroup___69 {
            fn fn__9692(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("quotes must be doubled: {}", self.s__1075.clone()));
            }
        }
        let closure_group = ClosureGroup___69 {
            s__1075: s__1075.clone()
        };
        let fn__9692 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__9692())
        };
        test___61.assert(t___9702, fn__9692.clone());
        let mut t___9706: bool = temper_core::string::index_of( & s__1075, "SELECT * FROM users WHERE name =", None).is_some();
        #[derive(Clone)]
        struct ClosureGroup___70 {
            s__1075: std::sync::Arc<String>
        }
        impl ClosureGroup___70 {
            fn fn__9691(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("structure intact: {}", self.s__1075.clone()));
            }
        }
        let closure_group = ClosureGroup___70 {
            s__1075: s__1075.clone()
        };
        let fn__9691 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__9691())
        };
        test___61.assert(t___9706, fn__9691.clone());
        test___61.soft_fail_to_hard()
    }
    #[test]
    fn safeIdentifierRejectsUserSuppliedTableNameWithMetacharacters__1666() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___62 = temper_std::testing::Test::new();
        let attack__1077: std::sync::Arc<String> = std::sync::Arc::new("users; DROP TABLE users; --".to_string());
        let didBubble__1078: bool;
        'ok___10988: {
            'orelse___1913: {
                match safe_identifier("users; DROP TABLE users; --") {
                    Ok(x) => x,
                    _ => break 'orelse___1913
                };
                didBubble__1078 = false;
                break 'ok___10988;
            }
            didBubble__1078 = true;
        }
        #[derive(Clone)]
        struct ClosureGroup___71 {}
        impl ClosureGroup___71 {
            fn fn__9688(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("metacharacter-containing name must be rejected at construction".to_string());
            }
        }
        let closure_group = ClosureGroup___71 {};
        let fn__9688 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__9688())
        };
        test___62.assert(didBubble__1078, fn__9688.clone());
        test___62.soft_fail_to_hard()
    }
    #[test]
    fn innerJoinProducesInnerJoin__1667() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___63 = temper_std::testing::Test::new();
        let mut t___9677: SafeIdentifier = sid__505("users");
        let mut t___9678: SafeIdentifier = sid__505("orders");
        let mut t___9679: SqlBuilder = SqlBuilder::new();
        t___9679.append_safe("users.id = orders.user_id");
        let mut t___9681: SqlFragment = t___9679.accumulated();
        let q__1080: Query = from(t___9677.clone()).inner_join(t___9678.clone(), t___9681.clone());
        let mut t___9686: bool = Some(q__1080.to_sql().to_string().as_str()) == Some("SELECT * FROM users INNER JOIN orders ON users.id = orders.user_id");
        #[derive(Clone)]
        struct ClosureGroup___72 {}
        impl ClosureGroup___72 {
            fn fn__9676(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("inner join".to_string());
            }
        }
        let closure_group = ClosureGroup___72 {};
        let fn__9676 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__9676())
        };
        test___63.assert(t___9686, fn__9676.clone());
        test___63.soft_fail_to_hard()
    }
    #[test]
    fn leftJoinProducesLeftJoin__1669() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___64 = temper_std::testing::Test::new();
        let mut t___9665: SafeIdentifier = sid__505("users");
        let mut t___9666: SafeIdentifier = sid__505("profiles");
        let mut t___9667: SqlBuilder = SqlBuilder::new();
        t___9667.append_safe("users.id = profiles.user_id");
        let mut t___9669: SqlFragment = t___9667.accumulated();
        let q__1082: Query = from(t___9665.clone()).left_join(t___9666.clone(), t___9669.clone());
        let mut t___9674: bool = Some(q__1082.to_sql().to_string().as_str()) == Some("SELECT * FROM users LEFT JOIN profiles ON users.id = profiles.user_id");
        #[derive(Clone)]
        struct ClosureGroup___73 {}
        impl ClosureGroup___73 {
            fn fn__9664(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("left join".to_string());
            }
        }
        let closure_group = ClosureGroup___73 {};
        let fn__9664 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__9664())
        };
        test___64.assert(t___9674, fn__9664.clone());
        test___64.soft_fail_to_hard()
    }
    #[test]
    fn rightJoinProducesRightJoin__1671() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___65 = temper_std::testing::Test::new();
        let mut t___9653: SafeIdentifier = sid__505("orders");
        let mut t___9654: SafeIdentifier = sid__505("users");
        let mut t___9655: SqlBuilder = SqlBuilder::new();
        t___9655.append_safe("orders.user_id = users.id");
        let mut t___9657: SqlFragment = t___9655.accumulated();
        let q__1084: Query = from(t___9653.clone()).right_join(t___9654.clone(), t___9657.clone());
        let mut t___9662: bool = Some(q__1084.to_sql().to_string().as_str()) == Some("SELECT * FROM orders RIGHT JOIN users ON orders.user_id = users.id");
        #[derive(Clone)]
        struct ClosureGroup___74 {}
        impl ClosureGroup___74 {
            fn fn__9652(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("right join".to_string());
            }
        }
        let closure_group = ClosureGroup___74 {};
        let fn__9652 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__9652())
        };
        test___65.assert(t___9662, fn__9652.clone());
        test___65.soft_fail_to_hard()
    }
    #[test]
    fn fullJoinProducesFullOuterJoin__1673() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___66 = temper_std::testing::Test::new();
        let mut t___9641: SafeIdentifier = sid__505("users");
        let mut t___9642: SafeIdentifier = sid__505("orders");
        let mut t___9643: SqlBuilder = SqlBuilder::new();
        t___9643.append_safe("users.id = orders.user_id");
        let mut t___9645: SqlFragment = t___9643.accumulated();
        let q__1086: Query = from(t___9641.clone()).full_join(t___9642.clone(), t___9645.clone());
        let mut t___9650: bool = Some(q__1086.to_sql().to_string().as_str()) == Some("SELECT * FROM users FULL OUTER JOIN orders ON users.id = orders.user_id");
        #[derive(Clone)]
        struct ClosureGroup___75 {}
        impl ClosureGroup___75 {
            fn fn__9640(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("full join".to_string());
            }
        }
        let closure_group = ClosureGroup___75 {};
        let fn__9640 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__9640())
        };
        test___66.assert(t___9650, fn__9640.clone());
        test___66.soft_fail_to_hard()
    }
    #[test]
    fn chainedJoins__1675() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___67 = temper_std::testing::Test::new();
        let mut t___9624: SafeIdentifier = sid__505("users");
        let mut t___9625: SafeIdentifier = sid__505("orders");
        let mut t___9626: SqlBuilder = SqlBuilder::new();
        t___9626.append_safe("users.id = orders.user_id");
        let mut t___9628: SqlFragment = t___9626.accumulated();
        let mut t___9629: Query = from(t___9624.clone()).inner_join(t___9625.clone(), t___9628.clone());
        let mut t___9630: SafeIdentifier = sid__505("profiles");
        let mut t___9631: SqlBuilder = SqlBuilder::new();
        t___9631.append_safe("users.id = profiles.user_id");
        let q__1088: Query = t___9629.left_join(t___9630.clone(), t___9631.accumulated());
        let mut t___9638: bool = Some(q__1088.to_sql().to_string().as_str()) == Some("SELECT * FROM users INNER JOIN orders ON users.id = orders.user_id LEFT JOIN profiles ON users.id = profiles.user_id");
        #[derive(Clone)]
        struct ClosureGroup___76 {}
        impl ClosureGroup___76 {
            fn fn__9623(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("chained joins".to_string());
            }
        }
        let closure_group = ClosureGroup___76 {};
        let fn__9623 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__9623())
        };
        test___67.assert(t___9638, fn__9623.clone());
        test___67.soft_fail_to_hard()
    }
    #[test]
    fn joinWithWhereAndOrderBy__1678() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___68 = temper_std::testing::Test::new();
        let mut t___9605: SafeIdentifier;
        let mut t___9606: SafeIdentifier;
        let mut t___9607: SqlBuilder;
        let mut t___9609: SqlFragment;
        let mut t___9610: Query;
        let mut t___9611: SqlBuilder;
        let mut t___5045: Query;
        let q__1090: Query;
        'ok___10989: {
            'orelse___1914: {
                t___9605 = sid__505("users");
                t___9606 = sid__505("orders");
                t___9607 = SqlBuilder::new();
                t___9607.append_safe("users.id = orders.user_id");
                t___9609 = t___9607.accumulated();
                t___9610 = from(t___9605.clone()).inner_join(t___9606.clone(), t___9609.clone());
                t___9611 = SqlBuilder::new();
                t___9611.append_safe("orders.total > ");
                t___9611.append_int32(100);
                t___5045 = match t___9610.r#where(t___9611.accumulated()).order_by(sid__505("name"), true).limit(10) {
                    Ok(x) => x,
                    _ => break 'orelse___1914
                };
                q__1090 = t___5045.clone();
                break 'ok___10989;
            }
            q__1090 = panic!();
        }
        let mut t___9621: bool = Some(q__1090.to_sql().to_string().as_str()) == Some("SELECT * FROM users INNER JOIN orders ON users.id = orders.user_id WHERE orders.total > 100 ORDER BY name ASC LIMIT 10");
        #[derive(Clone)]
        struct ClosureGroup___77 {}
        impl ClosureGroup___77 {
            fn fn__9604(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("join with where/order/limit".to_string());
            }
        }
        let closure_group = ClosureGroup___77 {};
        let fn__9604 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__9604())
        };
        test___68.assert(t___9621, fn__9604.clone());
        test___68.soft_fail_to_hard()
    }
    #[test]
    fn colHelperProducesQualifiedReference__1681() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___69 = temper_std::testing::Test::new();
        let c__1092: SqlFragment = col(sid__505("users"), sid__505("id"));
        let mut t___9602: bool = Some(c__1092.to_string().as_str()) == Some("users.id");
        #[derive(Clone)]
        struct ClosureGroup___78 {}
        impl ClosureGroup___78 {
            fn fn__9596(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("col helper".to_string());
            }
        }
        let closure_group = ClosureGroup___78 {};
        let fn__9596 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__9596())
        };
        test___69.assert(t___9602, fn__9596.clone());
        test___69.soft_fail_to_hard()
    }
    #[test]
    fn joinWithColHelper__1682() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___70 = temper_std::testing::Test::new();
        let onCond__1094: SqlFragment = col(sid__505("users"), sid__505("id"));
        let b__1095: SqlBuilder = SqlBuilder::new();
        b__1095.append_fragment(onCond__1094.clone());
        b__1095.append_safe(" = ");
        b__1095.append_fragment(col(sid__505("orders"), sid__505("user_id")));
        let mut t___9587: SafeIdentifier = sid__505("users");
        let mut t___9588: SafeIdentifier = sid__505("orders");
        let mut t___9589: SqlFragment = b__1095.accumulated();
        let q__1096: Query = from(t___9587.clone()).inner_join(t___9588.clone(), t___9589.clone());
        let mut t___9594: bool = Some(q__1096.to_sql().to_string().as_str()) == Some("SELECT * FROM users INNER JOIN orders ON users.id = orders.user_id");
        #[derive(Clone)]
        struct ClosureGroup___79 {}
        impl ClosureGroup___79 {
            fn fn__9576(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("join with col".to_string());
            }
        }
        let closure_group = ClosureGroup___79 {};
        let fn__9576 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__9576())
        };
        test___70.assert(t___9594, fn__9576.clone());
        test___70.soft_fail_to_hard()
    }
    #[test]
    fn orWhereBasic__1683() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___71 = temper_std::testing::Test::new();
        let mut t___9565: SafeIdentifier = sid__505("users");
        let mut t___9566: SqlBuilder = SqlBuilder::new();
        t___9566.append_safe("status = ");
        t___9566.append_string("active");
        let mut t___9569: SqlFragment = t___9566.accumulated();
        let q__1098: Query = from(t___9565.clone()).or_where(t___9569.clone());
        let mut t___9574: bool = Some(q__1098.to_sql().to_string().as_str()) == Some("SELECT * FROM users WHERE status = 'active'");
        #[derive(Clone)]
        struct ClosureGroup___80 {}
        impl ClosureGroup___80 {
            fn fn__9564(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("orWhere basic".to_string());
            }
        }
        let closure_group = ClosureGroup___80 {};
        let fn__9564 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__9564())
        };
        test___71.assert(t___9574, fn__9564.clone());
        test___71.soft_fail_to_hard()
    }
    #[test]
    fn whereThenOrWhere__1685() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___72 = temper_std::testing::Test::new();
        let mut t___9548: SafeIdentifier = sid__505("users");
        let mut t___9549: SqlBuilder = SqlBuilder::new();
        t___9549.append_safe("age > ");
        t___9549.append_int32(18);
        let mut t___9552: SqlFragment = t___9549.accumulated();
        let mut t___9553: Query = from(t___9548.clone()).r#where(t___9552.clone());
        let mut t___9554: SqlBuilder = SqlBuilder::new();
        t___9554.append_safe("vip = ");
        t___9554.append_boolean(true);
        let q__1100: Query = t___9553.or_where(t___9554.accumulated());
        let mut t___9562: bool = Some(q__1100.to_sql().to_string().as_str()) == Some("SELECT * FROM users WHERE age > 18 OR vip = TRUE");
        #[derive(Clone)]
        struct ClosureGroup___81 {}
        impl ClosureGroup___81 {
            fn fn__9547(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("where then orWhere".to_string());
            }
        }
        let closure_group = ClosureGroup___81 {};
        let fn__9547 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__9547())
        };
        test___72.assert(t___9562, fn__9547.clone());
        test___72.soft_fail_to_hard()
    }
    #[test]
    fn multipleOrWhere__1688() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___73 = temper_std::testing::Test::new();
        let mut t___9526: SafeIdentifier = sid__505("users");
        let mut t___9527: SqlBuilder = SqlBuilder::new();
        t___9527.append_safe("active = ");
        t___9527.append_boolean(true);
        let mut t___9530: SqlFragment = t___9527.accumulated();
        let mut t___9531: Query = from(t___9526.clone()).r#where(t___9530.clone());
        let mut t___9532: SqlBuilder = SqlBuilder::new();
        t___9532.append_safe("role = ");
        t___9532.append_string("admin");
        let mut t___9536: Query = t___9531.or_where(t___9532.accumulated());
        let mut t___9537: SqlBuilder = SqlBuilder::new();
        t___9537.append_safe("role = ");
        t___9537.append_string("moderator");
        let q__1102: Query = t___9536.or_where(t___9537.accumulated());
        let mut t___9545: bool = Some(q__1102.to_sql().to_string().as_str()) == Some("SELECT * FROM users WHERE active = TRUE OR role = 'admin' OR role = 'moderator'");
        #[derive(Clone)]
        struct ClosureGroup___82 {}
        impl ClosureGroup___82 {
            fn fn__9525(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("multiple orWhere".to_string());
            }
        }
        let closure_group = ClosureGroup___82 {};
        let fn__9525 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__9525())
        };
        test___73.assert(t___9545, fn__9525.clone());
        test___73.soft_fail_to_hard()
    }
    #[test]
    fn mixedWhereAndOrWhere__1692() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___74 = temper_std::testing::Test::new();
        let mut t___9504: SafeIdentifier = sid__505("users");
        let mut t___9505: SqlBuilder = SqlBuilder::new();
        t___9505.append_safe("age > ");
        t___9505.append_int32(18);
        let mut t___9508: SqlFragment = t___9505.accumulated();
        let mut t___9509: Query = from(t___9504.clone()).r#where(t___9508.clone());
        let mut t___9510: SqlBuilder = SqlBuilder::new();
        t___9510.append_safe("active = ");
        t___9510.append_boolean(true);
        let mut t___9514: Query = t___9509.r#where(t___9510.accumulated());
        let mut t___9515: SqlBuilder = SqlBuilder::new();
        t___9515.append_safe("vip = ");
        t___9515.append_boolean(true);
        let q__1104: Query = t___9514.or_where(t___9515.accumulated());
        let mut t___9523: bool = Some(q__1104.to_sql().to_string().as_str()) == Some("SELECT * FROM users WHERE age > 18 AND active = TRUE OR vip = TRUE");
        #[derive(Clone)]
        struct ClosureGroup___83 {}
        impl ClosureGroup___83 {
            fn fn__9503(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("mixed where and orWhere".to_string());
            }
        }
        let closure_group = ClosureGroup___83 {};
        let fn__9503 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__9503())
        };
        test___74.assert(t___9523, fn__9503.clone());
        test___74.soft_fail_to_hard()
    }
    #[test]
    fn whereNull__1696() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___75 = temper_std::testing::Test::new();
        let mut t___9495: SafeIdentifier = sid__505("users");
        let mut t___9496: SafeIdentifier = sid__505("deleted_at");
        let q__1106: Query = from(t___9495.clone()).where_null(t___9496.clone());
        let mut t___9501: bool = Some(q__1106.to_sql().to_string().as_str()) == Some("SELECT * FROM users WHERE deleted_at IS NULL");
        #[derive(Clone)]
        struct ClosureGroup___84 {}
        impl ClosureGroup___84 {
            fn fn__9494(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("whereNull".to_string());
            }
        }
        let closure_group = ClosureGroup___84 {};
        let fn__9494 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__9494())
        };
        test___75.assert(t___9501, fn__9494.clone());
        test___75.soft_fail_to_hard()
    }
    #[test]
    fn whereNotNull__1697() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___76 = temper_std::testing::Test::new();
        let mut t___9486: SafeIdentifier = sid__505("users");
        let mut t___9487: SafeIdentifier = sid__505("email");
        let q__1108: Query = from(t___9486.clone()).where_not_null(t___9487.clone());
        let mut t___9492: bool = Some(q__1108.to_sql().to_string().as_str()) == Some("SELECT * FROM users WHERE email IS NOT NULL");
        #[derive(Clone)]
        struct ClosureGroup___85 {}
        impl ClosureGroup___85 {
            fn fn__9485(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("whereNotNull".to_string());
            }
        }
        let closure_group = ClosureGroup___85 {};
        let fn__9485 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__9485())
        };
        test___76.assert(t___9492, fn__9485.clone());
        test___76.soft_fail_to_hard()
    }
    #[test]
    fn whereNullChainedWithWhere__1698() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___77 = temper_std::testing::Test::new();
        let mut t___9472: SafeIdentifier = sid__505("users");
        let mut t___9473: SqlBuilder = SqlBuilder::new();
        t___9473.append_safe("active = ");
        t___9473.append_boolean(true);
        let mut t___9476: SqlFragment = t___9473.accumulated();
        let q__1110: Query = from(t___9472.clone()).r#where(t___9476.clone()).where_null(sid__505("deleted_at"));
        let mut t___9483: bool = Some(q__1110.to_sql().to_string().as_str()) == Some("SELECT * FROM users WHERE active = TRUE AND deleted_at IS NULL");
        #[derive(Clone)]
        struct ClosureGroup___86 {}
        impl ClosureGroup___86 {
            fn fn__9471(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("whereNull chained".to_string());
            }
        }
        let closure_group = ClosureGroup___86 {};
        let fn__9471 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__9471())
        };
        test___77.assert(t___9483, fn__9471.clone());
        test___77.soft_fail_to_hard()
    }
    #[test]
    fn whereNotNullChainedWithOrWhere__1700() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___78 = temper_std::testing::Test::new();
        let mut t___9458: SafeIdentifier = sid__505("users");
        let mut t___9459: SafeIdentifier = sid__505("deleted_at");
        let mut t___9460: Query = from(t___9458.clone()).where_null(t___9459.clone());
        let mut t___9461: SqlBuilder = SqlBuilder::new();
        t___9461.append_safe("role = ");
        t___9461.append_string("admin");
        let q__1112: Query = t___9460.or_where(t___9461.accumulated());
        let mut t___9469: bool = Some(q__1112.to_sql().to_string().as_str()) == Some("SELECT * FROM users WHERE deleted_at IS NULL OR role = 'admin'");
        #[derive(Clone)]
        struct ClosureGroup___87 {}
        impl ClosureGroup___87 {
            fn fn__9457(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("whereNotNull with orWhere".to_string());
            }
        }
        let closure_group = ClosureGroup___87 {};
        let fn__9457 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__9457())
        };
        test___78.assert(t___9469, fn__9457.clone());
        test___78.soft_fail_to_hard()
    }
    #[test]
    fn whereInWithIntValues__1702() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___79 = temper_std::testing::Test::new();
        let mut t___9446: SafeIdentifier = sid__505("users");
        let mut t___9447: SafeIdentifier = sid__505("id");
        let mut t___9448: SqlInt32 = SqlInt32::new(1);
        let mut t___9449: SqlInt32 = SqlInt32::new(2);
        let mut t___9450: SqlInt32 = SqlInt32::new(3);
        let q__1114: Query = from(t___9446.clone()).where_in(t___9447.clone(), [SqlPart::new(t___9448.clone()), SqlPart::new(t___9449.clone()), SqlPart::new(t___9450.clone())]);
        let mut t___9455: bool = Some(q__1114.to_sql().to_string().as_str()) == Some("SELECT * FROM users WHERE id IN (1, 2, 3)");
        #[derive(Clone)]
        struct ClosureGroup___88 {}
        impl ClosureGroup___88 {
            fn fn__9445(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("whereIn ints".to_string());
            }
        }
        let closure_group = ClosureGroup___88 {};
        let fn__9445 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__9445())
        };
        test___79.assert(t___9455, fn__9445.clone());
        test___79.soft_fail_to_hard()
    }
    #[test]
    fn whereInWithStringValuesEscaping__1703() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___80 = temper_std::testing::Test::new();
        let mut t___9435: SafeIdentifier = sid__505("users");
        let mut t___9436: SafeIdentifier = sid__505("name");
        let mut t___9437: SqlString = SqlString::new("Alice");
        let mut t___9438: SqlString = SqlString::new("Bob's");
        let q__1116: Query = from(t___9435.clone()).where_in(t___9436.clone(), [SqlPart::new(t___9437.clone()), SqlPart::new(t___9438.clone())]);
        let mut t___9443: bool = Some(q__1116.to_sql().to_string().as_str()) == Some("SELECT * FROM users WHERE name IN ('Alice', 'Bob''s')");
        #[derive(Clone)]
        struct ClosureGroup___89 {}
        impl ClosureGroup___89 {
            fn fn__9434(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("whereIn strings".to_string());
            }
        }
        let closure_group = ClosureGroup___89 {};
        let fn__9434 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__9434())
        };
        test___80.assert(t___9443, fn__9434.clone());
        test___80.soft_fail_to_hard()
    }
    #[test]
    fn whereInWithEmptyListProduces1_0__1704() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___81 = temper_std::testing::Test::new();
        let mut t___9426: SafeIdentifier = sid__505("users");
        let mut t___9427: SafeIdentifier = sid__505("id");
        let q__1118: Query = from(t___9426.clone()).where_in(t___9427.clone(), []);
        let mut t___9432: bool = Some(q__1118.to_sql().to_string().as_str()) == Some("SELECT * FROM users WHERE 1 = 0");
        #[derive(Clone)]
        struct ClosureGroup___90 {}
        impl ClosureGroup___90 {
            fn fn__9425(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("whereIn empty".to_string());
            }
        }
        let closure_group = ClosureGroup___90 {};
        let fn__9425 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__9425())
        };
        test___81.assert(t___9432, fn__9425.clone());
        test___81.soft_fail_to_hard()
    }
    #[test]
    fn whereInChained__1705() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___82 = temper_std::testing::Test::new();
        let mut t___9410: SafeIdentifier = sid__505("users");
        let mut t___9411: SqlBuilder = SqlBuilder::new();
        t___9411.append_safe("active = ");
        t___9411.append_boolean(true);
        let mut t___9414: SqlFragment = t___9411.accumulated();
        let q__1120: Query = from(t___9410.clone()).r#where(t___9414.clone()).where_in(sid__505("role"), [SqlPart::new(SqlString::new("admin")), SqlPart::new(SqlString::new("user"))]);
        let mut t___9423: bool = Some(q__1120.to_sql().to_string().as_str()) == Some("SELECT * FROM users WHERE active = TRUE AND role IN ('admin', 'user')");
        #[derive(Clone)]
        struct ClosureGroup___91 {}
        impl ClosureGroup___91 {
            fn fn__9409(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("whereIn chained".to_string());
            }
        }
        let closure_group = ClosureGroup___91 {};
        let fn__9409 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__9409())
        };
        test___82.assert(t___9423, fn__9409.clone());
        test___82.soft_fail_to_hard()
    }
    #[test]
    fn whereInSingleElement__1707() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___83 = temper_std::testing::Test::new();
        let mut t___9400: SafeIdentifier = sid__505("users");
        let mut t___9401: SafeIdentifier = sid__505("id");
        let mut t___9402: SqlInt32 = SqlInt32::new(42);
        let q__1122: Query = from(t___9400.clone()).where_in(t___9401.clone(), [SqlPart::new(t___9402.clone())]);
        let mut t___9407: bool = Some(q__1122.to_sql().to_string().as_str()) == Some("SELECT * FROM users WHERE id IN (42)");
        #[derive(Clone)]
        struct ClosureGroup___92 {}
        impl ClosureGroup___92 {
            fn fn__9399(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("whereIn single".to_string());
            }
        }
        let closure_group = ClosureGroup___92 {};
        let fn__9399 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__9399())
        };
        test___83.assert(t___9407, fn__9399.clone());
        test___83.soft_fail_to_hard()
    }
    #[test]
    fn whereNotBasic__1708() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___84 = temper_std::testing::Test::new();
        let mut t___9388: SafeIdentifier = sid__505("users");
        let mut t___9389: SqlBuilder = SqlBuilder::new();
        t___9389.append_safe("active = ");
        t___9389.append_boolean(true);
        let mut t___9392: SqlFragment = t___9389.accumulated();
        let q__1124: Query = from(t___9388.clone()).where_not(t___9392.clone());
        let mut t___9397: bool = Some(q__1124.to_sql().to_string().as_str()) == Some("SELECT * FROM users WHERE NOT (active = TRUE)");
        #[derive(Clone)]
        struct ClosureGroup___93 {}
        impl ClosureGroup___93 {
            fn fn__9387(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("whereNot".to_string());
            }
        }
        let closure_group = ClosureGroup___93 {};
        let fn__9387 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__9387())
        };
        test___84.assert(t___9397, fn__9387.clone());
        test___84.soft_fail_to_hard()
    }
    #[test]
    fn whereNotChained__1710() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___85 = temper_std::testing::Test::new();
        let mut t___9371: SafeIdentifier = sid__505("users");
        let mut t___9372: SqlBuilder = SqlBuilder::new();
        t___9372.append_safe("age > ");
        t___9372.append_int32(18);
        let mut t___9375: SqlFragment = t___9372.accumulated();
        let mut t___9376: Query = from(t___9371.clone()).r#where(t___9375.clone());
        let mut t___9377: SqlBuilder = SqlBuilder::new();
        t___9377.append_safe("banned = ");
        t___9377.append_boolean(true);
        let q__1126: Query = t___9376.where_not(t___9377.accumulated());
        let mut t___9385: bool = Some(q__1126.to_sql().to_string().as_str()) == Some("SELECT * FROM users WHERE age > 18 AND NOT (banned = TRUE)");
        #[derive(Clone)]
        struct ClosureGroup___94 {}
        impl ClosureGroup___94 {
            fn fn__9370(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("whereNot chained".to_string());
            }
        }
        let closure_group = ClosureGroup___94 {};
        let fn__9370 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__9370())
        };
        test___85.assert(t___9385, fn__9370.clone());
        test___85.soft_fail_to_hard()
    }
    #[test]
    fn whereBetweenIntegers__1713() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___86 = temper_std::testing::Test::new();
        let mut t___9360: SafeIdentifier = sid__505("users");
        let mut t___9361: SafeIdentifier = sid__505("age");
        let mut t___9362: SqlInt32 = SqlInt32::new(18);
        let mut t___9363: SqlInt32 = SqlInt32::new(65);
        let q__1128: Query = from(t___9360.clone()).where_between(t___9361.clone(), SqlPart::new(t___9362.clone()), SqlPart::new(t___9363.clone()));
        let mut t___9368: bool = Some(q__1128.to_sql().to_string().as_str()) == Some("SELECT * FROM users WHERE age BETWEEN 18 AND 65");
        #[derive(Clone)]
        struct ClosureGroup___95 {}
        impl ClosureGroup___95 {
            fn fn__9359(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("whereBetween ints".to_string());
            }
        }
        let closure_group = ClosureGroup___95 {};
        let fn__9359 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__9359())
        };
        test___86.assert(t___9368, fn__9359.clone());
        test___86.soft_fail_to_hard()
    }
    #[test]
    fn whereBetweenChained__1714() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___87 = temper_std::testing::Test::new();
        let mut t___9344: SafeIdentifier = sid__505("users");
        let mut t___9345: SqlBuilder = SqlBuilder::new();
        t___9345.append_safe("active = ");
        t___9345.append_boolean(true);
        let mut t___9348: SqlFragment = t___9345.accumulated();
        let q__1130: Query = from(t___9344.clone()).r#where(t___9348.clone()).where_between(sid__505("age"), SqlPart::new(SqlInt32::new(21)), SqlPart::new(SqlInt32::new(30)));
        let mut t___9357: bool = Some(q__1130.to_sql().to_string().as_str()) == Some("SELECT * FROM users WHERE active = TRUE AND age BETWEEN 21 AND 30");
        #[derive(Clone)]
        struct ClosureGroup___96 {}
        impl ClosureGroup___96 {
            fn fn__9343(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("whereBetween chained".to_string());
            }
        }
        let closure_group = ClosureGroup___96 {};
        let fn__9343 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__9343())
        };
        test___87.assert(t___9357, fn__9343.clone());
        test___87.soft_fail_to_hard()
    }
    #[test]
    fn whereLikeBasic__1716() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___88 = temper_std::testing::Test::new();
        let mut t___9335: SafeIdentifier = sid__505("users");
        let mut t___9336: SafeIdentifier = sid__505("name");
        let q__1132: Query = from(t___9335.clone()).where_like(t___9336.clone(), "John%");
        let mut t___9341: bool = Some(q__1132.to_sql().to_string().as_str()) == Some("SELECT * FROM users WHERE name LIKE 'John%'");
        #[derive(Clone)]
        struct ClosureGroup___97 {}
        impl ClosureGroup___97 {
            fn fn__9334(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("whereLike".to_string());
            }
        }
        let closure_group = ClosureGroup___97 {};
        let fn__9334 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__9334())
        };
        test___88.assert(t___9341, fn__9334.clone());
        test___88.soft_fail_to_hard()
    }
    #[test]
    fn whereIlikeBasic__1717() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___89 = temper_std::testing::Test::new();
        let mut t___9326: SafeIdentifier = sid__505("users");
        let mut t___9327: SafeIdentifier = sid__505("email");
        let q__1134: Query = from(t___9326.clone()).where_i_like(t___9327.clone(), "%@gmail.com");
        let mut t___9332: bool = Some(q__1134.to_sql().to_string().as_str()) == Some("SELECT * FROM users WHERE email ILIKE '%@gmail.com'");
        #[derive(Clone)]
        struct ClosureGroup___98 {}
        impl ClosureGroup___98 {
            fn fn__9325(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("whereILike".to_string());
            }
        }
        let closure_group = ClosureGroup___98 {};
        let fn__9325 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__9325())
        };
        test___89.assert(t___9332, fn__9325.clone());
        test___89.soft_fail_to_hard()
    }
    #[test]
    fn whereLikeWithInjectionAttempt__1718() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___90 = temper_std::testing::Test::new();
        let mut t___9312: SafeIdentifier = sid__505("users");
        let mut t___9313: SafeIdentifier = sid__505("name");
        let q__1136: Query = from(t___9312.clone()).where_like(t___9313.clone(), "'; DROP TABLE users; --");
        let s__1137: std::sync::Arc<String> = q__1136.to_sql().to_string();
        let mut t___9318: bool = temper_core::string::index_of( & s__1137, "''", None).is_some();
        #[derive(Clone)]
        struct ClosureGroup___99 {
            s__1137: std::sync::Arc<String>
        }
        impl ClosureGroup___99 {
            fn fn__9311(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("like injection escaped: {}", self.s__1137.clone()));
            }
        }
        let closure_group = ClosureGroup___99 {
            s__1137: s__1137.clone()
        };
        let fn__9311 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__9311())
        };
        test___90.assert(t___9318, fn__9311.clone());
        let mut t___9322: bool = temper_core::string::index_of( & s__1137, "LIKE", None).is_some();
        #[derive(Clone)]
        struct ClosureGroup___100 {
            s__1137: std::sync::Arc<String>
        }
        impl ClosureGroup___100 {
            fn fn__9310(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("like structure intact: {}", self.s__1137.clone()));
            }
        }
        let closure_group = ClosureGroup___100 {
            s__1137: s__1137.clone()
        };
        let fn__9310 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__9310())
        };
        test___90.assert(t___9322, fn__9310.clone());
        test___90.soft_fail_to_hard()
    }
    #[test]
    fn whereLikeWildcardPatterns__1719() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___91 = temper_std::testing::Test::new();
        let mut t___9302: SafeIdentifier = sid__505("users");
        let mut t___9303: SafeIdentifier = sid__505("name");
        let q__1139: Query = from(t___9302.clone()).where_like(t___9303.clone(), "%son%");
        let mut t___9308: bool = Some(q__1139.to_sql().to_string().as_str()) == Some("SELECT * FROM users WHERE name LIKE '%son%'");
        #[derive(Clone)]
        struct ClosureGroup___101 {}
        impl ClosureGroup___101 {
            fn fn__9301(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("whereLike wildcard".to_string());
            }
        }
        let closure_group = ClosureGroup___101 {};
        let fn__9301 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__9301())
        };
        test___91.assert(t___9308, fn__9301.clone());
        test___91.soft_fail_to_hard()
    }
    #[test]
    fn countAllProducesCount__1720() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___92 = temper_std::testing::Test::new();
        let f__1141: SqlFragment = count_all();
        let mut t___9299: bool = Some(f__1141.to_string().as_str()) == Some("COUNT(*)");
        #[derive(Clone)]
        struct ClosureGroup___102 {}
        impl ClosureGroup___102 {
            fn fn__9295(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("countAll".to_string());
            }
        }
        let closure_group = ClosureGroup___102 {};
        let fn__9295 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__9295())
        };
        test___92.assert(t___9299, fn__9295.clone());
        test___92.soft_fail_to_hard()
    }
    #[test]
    fn countColProducesCountField__1721() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___93 = temper_std::testing::Test::new();
        let f__1143: SqlFragment = count_col(sid__505("id"));
        let mut t___9293: bool = Some(f__1143.to_string().as_str()) == Some("COUNT(id)");
        #[derive(Clone)]
        struct ClosureGroup___103 {}
        impl ClosureGroup___103 {
            fn fn__9288(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("countCol".to_string());
            }
        }
        let closure_group = ClosureGroup___103 {};
        let fn__9288 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__9288())
        };
        test___93.assert(t___9293, fn__9288.clone());
        test___93.soft_fail_to_hard()
    }
    #[test]
    fn sumColProducesSumField__1722() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___94 = temper_std::testing::Test::new();
        let f__1145: SqlFragment = sum_col(sid__505("amount"));
        let mut t___9286: bool = Some(f__1145.to_string().as_str()) == Some("SUM(amount)");
        #[derive(Clone)]
        struct ClosureGroup___104 {}
        impl ClosureGroup___104 {
            fn fn__9281(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("sumCol".to_string());
            }
        }
        let closure_group = ClosureGroup___104 {};
        let fn__9281 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__9281())
        };
        test___94.assert(t___9286, fn__9281.clone());
        test___94.soft_fail_to_hard()
    }
    #[test]
    fn avgColProducesAvgField__1723() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___95 = temper_std::testing::Test::new();
        let f__1147: SqlFragment = avg_col(sid__505("price"));
        let mut t___9279: bool = Some(f__1147.to_string().as_str()) == Some("AVG(price)");
        #[derive(Clone)]
        struct ClosureGroup___105 {}
        impl ClosureGroup___105 {
            fn fn__9274(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("avgCol".to_string());
            }
        }
        let closure_group = ClosureGroup___105 {};
        let fn__9274 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__9274())
        };
        test___95.assert(t___9279, fn__9274.clone());
        test___95.soft_fail_to_hard()
    }
    #[test]
    fn minColProducesMinField__1724() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___96 = temper_std::testing::Test::new();
        let f__1149: SqlFragment = min_col(sid__505("created_at"));
        let mut t___9272: bool = Some(f__1149.to_string().as_str()) == Some("MIN(created_at)");
        #[derive(Clone)]
        struct ClosureGroup___106 {}
        impl ClosureGroup___106 {
            fn fn__9267(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("minCol".to_string());
            }
        }
        let closure_group = ClosureGroup___106 {};
        let fn__9267 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__9267())
        };
        test___96.assert(t___9272, fn__9267.clone());
        test___96.soft_fail_to_hard()
    }
    #[test]
    fn maxColProducesMaxField__1725() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___97 = temper_std::testing::Test::new();
        let f__1151: SqlFragment = max_col(sid__505("score"));
        let mut t___9265: bool = Some(f__1151.to_string().as_str()) == Some("MAX(score)");
        #[derive(Clone)]
        struct ClosureGroup___107 {}
        impl ClosureGroup___107 {
            fn fn__9260(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("maxCol".to_string());
            }
        }
        let closure_group = ClosureGroup___107 {};
        let fn__9260 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__9260())
        };
        test___97.assert(t___9265, fn__9260.clone());
        test___97.soft_fail_to_hard()
    }
    #[test]
    fn selectExprWithAggregate__1726() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___98 = temper_std::testing::Test::new();
        let mut t___9252: SafeIdentifier = sid__505("orders");
        let mut t___9253: SqlFragment = count_all();
        let q__1153: Query = from(t___9252.clone()).select_expr([t___9253.clone()]);
        let mut t___9258: bool = Some(q__1153.to_sql().to_string().as_str()) == Some("SELECT COUNT(*) FROM orders");
        #[derive(Clone)]
        struct ClosureGroup___108 {}
        impl ClosureGroup___108 {
            fn fn__9251(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("selectExpr count".to_string());
            }
        }
        let closure_group = ClosureGroup___108 {};
        let fn__9251 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__9251())
        };
        test___98.assert(t___9258, fn__9251.clone());
        test___98.soft_fail_to_hard()
    }
    #[test]
    fn selectExprWithMultipleExpressions__1727() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___99 = temper_std::testing::Test::new();
        let nameFrag__1155: SqlFragment = col(sid__505("users"), sid__505("name"));
        let mut t___9243: SafeIdentifier = sid__505("users");
        let mut t___9244: SqlFragment = count_all();
        let q__1156: Query = from(t___9243.clone()).select_expr([nameFrag__1155.clone(), t___9244.clone()]);
        let mut t___9249: bool = Some(q__1156.to_sql().to_string().as_str()) == Some("SELECT users.name, COUNT(*) FROM users");
        #[derive(Clone)]
        struct ClosureGroup___109 {}
        impl ClosureGroup___109 {
            fn fn__9239(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("selectExpr multi".to_string());
            }
        }
        let closure_group = ClosureGroup___109 {};
        let fn__9239 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__9239())
        };
        test___99.assert(t___9249, fn__9239.clone());
        test___99.soft_fail_to_hard()
    }
    #[test]
    fn selectExprOverridesSelectedFields__1728() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___100 = temper_std::testing::Test::new();
        let mut t___9228: SafeIdentifier = sid__505("users");
        let mut t___9229: SafeIdentifier = sid__505("id");
        let mut t___9230: SafeIdentifier = sid__505("name");
        let q__1158: Query = from(t___9228.clone()).select([t___9229.clone(), t___9230.clone()]).select_expr([count_all()]);
        let mut t___9237: bool = Some(q__1158.to_sql().to_string().as_str()) == Some("SELECT COUNT(*) FROM users");
        #[derive(Clone)]
        struct ClosureGroup___110 {}
        impl ClosureGroup___110 {
            fn fn__9227(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("selectExpr overrides select".to_string());
            }
        }
        let closure_group = ClosureGroup___110 {};
        let fn__9227 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__9227())
        };
        test___100.assert(t___9237, fn__9227.clone());
        test___100.soft_fail_to_hard()
    }
    #[test]
    fn groupBySingleField__1729() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___101 = temper_std::testing::Test::new();
        let mut t___9214: SafeIdentifier = sid__505("orders");
        let mut t___9217: SqlFragment = col(sid__505("orders"), sid__505("status"));
        let mut t___9218: SqlFragment = count_all();
        let q__1160: Query = from(t___9214.clone()).select_expr([t___9217.clone(), t___9218.clone()]).group_by(sid__505("status"));
        let mut t___9225: bool = Some(q__1160.to_sql().to_string().as_str()) == Some("SELECT orders.status, COUNT(*) FROM orders GROUP BY status");
        #[derive(Clone)]
        struct ClosureGroup___111 {}
        impl ClosureGroup___111 {
            fn fn__9213(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("groupBy single".to_string());
            }
        }
        let closure_group = ClosureGroup___111 {};
        let fn__9213 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__9213())
        };
        test___101.assert(t___9225, fn__9213.clone());
        test___101.soft_fail_to_hard()
    }
    #[test]
    fn groupByMultipleFields__1730() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___102 = temper_std::testing::Test::new();
        let mut t___9203: SafeIdentifier = sid__505("orders");
        let mut t___9204: SafeIdentifier = sid__505("status");
        let q__1162: Query = from(t___9203.clone()).group_by(t___9204.clone()).group_by(sid__505("category"));
        let mut t___9211: bool = Some(q__1162.to_sql().to_string().as_str()) == Some("SELECT * FROM orders GROUP BY status, category");
        #[derive(Clone)]
        struct ClosureGroup___112 {}
        impl ClosureGroup___112 {
            fn fn__9202(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("groupBy multiple".to_string());
            }
        }
        let closure_group = ClosureGroup___112 {};
        let fn__9202 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__9202())
        };
        test___102.assert(t___9211, fn__9202.clone());
        test___102.soft_fail_to_hard()
    }
    #[test]
    fn havingBasic__1731() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___103 = temper_std::testing::Test::new();
        let mut t___9184: SafeIdentifier = sid__505("orders");
        let mut t___9187: SqlFragment = col(sid__505("orders"), sid__505("status"));
        let mut t___9188: SqlFragment = count_all();
        let mut t___9191: Query = from(t___9184.clone()).select_expr([t___9187.clone(), t___9188.clone()]).group_by(sid__505("status"));
        let mut t___9192: SqlBuilder = SqlBuilder::new();
        t___9192.append_safe("COUNT(*) > ");
        t___9192.append_int32(5);
        let q__1164: Query = t___9191.having(t___9192.accumulated());
        let mut t___9200: bool = Some(q__1164.to_sql().to_string().as_str()) == Some("SELECT orders.status, COUNT(*) FROM orders GROUP BY status HAVING COUNT(*) > 5");
        #[derive(Clone)]
        struct ClosureGroup___113 {}
        impl ClosureGroup___113 {
            fn fn__9183(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("having basic".to_string());
            }
        }
        let closure_group = ClosureGroup___113 {};
        let fn__9183 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__9183())
        };
        test___103.assert(t___9200, fn__9183.clone());
        test___103.soft_fail_to_hard()
    }
    #[test]
    fn orHaving__1733() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___104 = temper_std::testing::Test::new();
        let mut t___9165: SafeIdentifier = sid__505("orders");
        let mut t___9166: SafeIdentifier = sid__505("status");
        let mut t___9167: Query = from(t___9165.clone()).group_by(t___9166.clone());
        let mut t___9168: SqlBuilder = SqlBuilder::new();
        t___9168.append_safe("COUNT(*) > ");
        t___9168.append_int32(5);
        let mut t___9172: Query = t___9167.having(t___9168.accumulated());
        let mut t___9173: SqlBuilder = SqlBuilder::new();
        t___9173.append_safe("SUM(total) > ");
        t___9173.append_int32(1000);
        let q__1166: Query = t___9172.or_having(t___9173.accumulated());
        let mut t___9181: bool = Some(q__1166.to_sql().to_string().as_str()) == Some("SELECT * FROM orders GROUP BY status HAVING COUNT(*) > 5 OR SUM(total) > 1000");
        #[derive(Clone)]
        struct ClosureGroup___114 {}
        impl ClosureGroup___114 {
            fn fn__9164(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("orHaving".to_string());
            }
        }
        let closure_group = ClosureGroup___114 {};
        let fn__9164 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__9164())
        };
        test___104.assert(t___9181, fn__9164.clone());
        test___104.soft_fail_to_hard()
    }
    #[test]
    fn distinctBasic__1736() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___105 = temper_std::testing::Test::new();
        let mut t___9155: SafeIdentifier = sid__505("users");
        let mut t___9156: SafeIdentifier = sid__505("name");
        let q__1168: Query = from(t___9155.clone()).select([t___9156.clone()]).distinct();
        let mut t___9162: bool = Some(q__1168.to_sql().to_string().as_str()) == Some("SELECT DISTINCT name FROM users");
        #[derive(Clone)]
        struct ClosureGroup___115 {}
        impl ClosureGroup___115 {
            fn fn__9154(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("distinct".to_string());
            }
        }
        let closure_group = ClosureGroup___115 {};
        let fn__9154 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__9154())
        };
        test___105.assert(t___9162, fn__9154.clone());
        test___105.soft_fail_to_hard()
    }
    #[test]
    fn distinctWithWhere__1737() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___106 = temper_std::testing::Test::new();
        let mut t___9140: SafeIdentifier = sid__505("users");
        let mut t___9141: SafeIdentifier = sid__505("email");
        let mut t___9142: Query = from(t___9140.clone()).select([t___9141.clone()]);
        let mut t___9143: SqlBuilder = SqlBuilder::new();
        t___9143.append_safe("active = ");
        t___9143.append_boolean(true);
        let q__1170: Query = t___9142.r#where(t___9143.accumulated()).distinct();
        let mut t___9152: bool = Some(q__1170.to_sql().to_string().as_str()) == Some("SELECT DISTINCT email FROM users WHERE active = TRUE");
        #[derive(Clone)]
        struct ClosureGroup___116 {}
        impl ClosureGroup___116 {
            fn fn__9139(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("distinct with where".to_string());
            }
        }
        let closure_group = ClosureGroup___116 {};
        let fn__9139 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__9139())
        };
        test___106.assert(t___9152, fn__9139.clone());
        test___106.soft_fail_to_hard()
    }
    #[test]
    fn countSqlBare__1739() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___107 = temper_std::testing::Test::new();
        let q__1172: Query = from(sid__505("users"));
        let mut t___9137: bool = Some(q__1172.count_sql().to_string().as_str()) == Some("SELECT COUNT(*) FROM users");
        #[derive(Clone)]
        struct ClosureGroup___117 {}
        impl ClosureGroup___117 {
            fn fn__9132(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("countSql bare".to_string());
            }
        }
        let closure_group = ClosureGroup___117 {};
        let fn__9132 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__9132())
        };
        test___107.assert(t___9137, fn__9132.clone());
        test___107.soft_fail_to_hard()
    }
    #[test]
    fn countSqlWithWhere__1740() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___108 = temper_std::testing::Test::new();
        let mut t___9121: SafeIdentifier = sid__505("users");
        let mut t___9122: SqlBuilder = SqlBuilder::new();
        t___9122.append_safe("active = ");
        t___9122.append_boolean(true);
        let mut t___9125: SqlFragment = t___9122.accumulated();
        let q__1174: Query = from(t___9121.clone()).r#where(t___9125.clone());
        let mut t___9130: bool = Some(q__1174.count_sql().to_string().as_str()) == Some("SELECT COUNT(*) FROM users WHERE active = TRUE");
        #[derive(Clone)]
        struct ClosureGroup___118 {}
        impl ClosureGroup___118 {
            fn fn__9120(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("countSql with where".to_string());
            }
        }
        let closure_group = ClosureGroup___118 {};
        let fn__9120 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__9120())
        };
        test___108.assert(t___9130, fn__9120.clone());
        test___108.soft_fail_to_hard()
    }
    #[test]
    fn countSqlWithJoin__1742() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___109 = temper_std::testing::Test::new();
        let mut t___9104: SafeIdentifier = sid__505("users");
        let mut t___9105: SafeIdentifier = sid__505("orders");
        let mut t___9106: SqlBuilder = SqlBuilder::new();
        t___9106.append_safe("users.id = orders.user_id");
        let mut t___9108: SqlFragment = t___9106.accumulated();
        let mut t___9109: Query = from(t___9104.clone()).inner_join(t___9105.clone(), t___9108.clone());
        let mut t___9110: SqlBuilder = SqlBuilder::new();
        t___9110.append_safe("orders.total > ");
        t___9110.append_int32(100);
        let q__1176: Query = t___9109.r#where(t___9110.accumulated());
        let mut t___9118: bool = Some(q__1176.count_sql().to_string().as_str()) == Some("SELECT COUNT(*) FROM users INNER JOIN orders ON users.id = orders.user_id WHERE orders.total > 100");
        #[derive(Clone)]
        struct ClosureGroup___119 {}
        impl ClosureGroup___119 {
            fn fn__9103(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("countSql with join".to_string());
            }
        }
        let closure_group = ClosureGroup___119 {};
        let fn__9103 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__9103())
        };
        test___109.assert(t___9118, fn__9103.clone());
        test___109.soft_fail_to_hard()
    }
    #[test]
    fn countSqlDropsOrderByLimitOffset__1745() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___110 = temper_std::testing::Test::new();
        let mut t___9090: SafeIdentifier;
        let mut t___9091: SqlBuilder;
        let mut t___9094: SqlFragment;
        let mut t___4621: Query;
        let mut t___4622: Query;
        let q__1178: Query;
        'ok___10990: {
            'orelse___1915: {
                t___9090 = sid__505("users");
                t___9091 = SqlBuilder::new();
                t___9091.append_safe("active = ");
                t___9091.append_boolean(true);
                t___9094 = t___9091.accumulated();
                t___4621 = match from(t___9090.clone()).r#where(t___9094.clone()).order_by(sid__505("name"), true).limit(10) {
                    Ok(x) => x,
                    _ => break 'orelse___1915
                };
                t___4622 = match t___4621.offset(20) {
                    Ok(x) => x,
                    _ => break 'orelse___1915
                };
                q__1178 = t___4622.clone();
                break 'ok___10990;
            }
            q__1178 = panic!();
        }
        let s__1179: std::sync::Arc<String> = q__1178.count_sql().to_string();
        let mut t___9101: bool = Some(s__1179.as_str()) == Some("SELECT COUNT(*) FROM users WHERE active = TRUE");
        #[derive(Clone)]
        struct ClosureGroup___120 {
            s__1179: std::sync::Arc<String>
        }
        impl ClosureGroup___120 {
            fn fn__9089(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("countSql drops extras: {}", self.s__1179.clone()));
            }
        }
        let closure_group = ClosureGroup___120 {
            s__1179: s__1179.clone()
        };
        let fn__9089 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__9089())
        };
        test___110.assert(t___9101, fn__9089.clone());
        test___110.soft_fail_to_hard()
    }
    #[test]
    fn fullAggregationQuery__1747() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___111 = temper_std::testing::Test::new();
        let mut t___9057: SafeIdentifier = sid__505("orders");
        let mut t___9060: SqlFragment = col(sid__505("orders"), sid__505("status"));
        let mut t___9061: SqlFragment = count_all();
        let mut t___9063: SqlFragment = sum_col(sid__505("total"));
        let mut t___9064: Query = from(t___9057.clone()).select_expr([t___9060.clone(), t___9061.clone(), t___9063.clone()]);
        let mut t___9065: SafeIdentifier = sid__505("users");
        let mut t___9066: SqlBuilder = SqlBuilder::new();
        t___9066.append_safe("orders.user_id = users.id");
        let mut t___9069: Query = t___9064.inner_join(t___9065.clone(), t___9066.accumulated());
        let mut t___9070: SqlBuilder = SqlBuilder::new();
        t___9070.append_safe("users.active = ");
        t___9070.append_boolean(true);
        let mut t___9076: Query = t___9069.r#where(t___9070.accumulated()).group_by(sid__505("status"));
        let mut t___9077: SqlBuilder = SqlBuilder::new();
        t___9077.append_safe("COUNT(*) > ");
        t___9077.append_int32(3);
        let q__1181: Query = t___9076.having(t___9077.accumulated()).order_by(sid__505("status"), true);
        let expected__1182: std::sync::Arc<String> = std::sync::Arc::new("SELECT orders.status, COUNT(*), SUM(total) FROM orders INNER JOIN users ON orders.user_id = users.id WHERE users.active = TRUE GROUP BY status HAVING COUNT(*) > 3 ORDER BY status ASC".to_string());
        let mut t___9087: bool = Some(q__1181.to_sql().to_string().as_str()) == Some("SELECT orders.status, COUNT(*), SUM(total) FROM orders INNER JOIN users ON orders.user_id = users.id WHERE users.active = TRUE GROUP BY status HAVING COUNT(*) > 3 ORDER BY status ASC");
        #[derive(Clone)]
        struct ClosureGroup___121 {}
        impl ClosureGroup___121 {
            fn fn__9056(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("full aggregation".to_string());
            }
        }
        let closure_group = ClosureGroup___121 {};
        let fn__9056 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__9056())
        };
        test___111.assert(t___9087, fn__9056.clone());
        test___111.soft_fail_to_hard()
    }
    #[test]
    fn unionSql__1751() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___112 = temper_std::testing::Test::new();
        let mut t___9039: SafeIdentifier = sid__505("users");
        let mut t___9040: SqlBuilder = SqlBuilder::new();
        t___9040.append_safe("role = ");
        t___9040.append_string("admin");
        let mut t___9043: SqlFragment = t___9040.accumulated();
        let a__1184: Query = from(t___9039.clone()).r#where(t___9043.clone());
        let mut t___9045: SafeIdentifier = sid__505("users");
        let mut t___9046: SqlBuilder = SqlBuilder::new();
        t___9046.append_safe("role = ");
        t___9046.append_string("moderator");
        let mut t___9049: SqlFragment = t___9046.accumulated();
        let b__1185: Query = from(t___9045.clone()).r#where(t___9049.clone());
        let s__1186: std::sync::Arc<String> = union_sql(a__1184.clone(), b__1185.clone()).to_string();
        let mut t___9054: bool = Some(s__1186.as_str()) == Some("(SELECT * FROM users WHERE role = 'admin') UNION (SELECT * FROM users WHERE role = 'moderator')");
        #[derive(Clone)]
        struct ClosureGroup___122 {
            s__1186: std::sync::Arc<String>
        }
        impl ClosureGroup___122 {
            fn fn__9038(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("unionSql: {}", self.s__1186.clone()));
            }
        }
        let closure_group = ClosureGroup___122 {
            s__1186: s__1186.clone()
        };
        let fn__9038 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__9038())
        };
        test___112.assert(t___9054, fn__9038.clone());
        test___112.soft_fail_to_hard()
    }
    #[test]
    fn unionAllSql__1754() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___113 = temper_std::testing::Test::new();
        let mut t___9027: SafeIdentifier = sid__505("users");
        let mut t___9028: SafeIdentifier = sid__505("name");
        let a__1188: Query = from(t___9027.clone()).select([t___9028.clone()]);
        let mut t___9030: SafeIdentifier = sid__505("contacts");
        let mut t___9031: SafeIdentifier = sid__505("name");
        let b__1189: Query = from(t___9030.clone()).select([t___9031.clone()]);
        let s__1190: std::sync::Arc<String> = union_all_sql(a__1188.clone(), b__1189.clone()).to_string();
        let mut t___9036: bool = Some(s__1190.as_str()) == Some("(SELECT name FROM users) UNION ALL (SELECT name FROM contacts)");
        #[derive(Clone)]
        struct ClosureGroup___123 {
            s__1190: std::sync::Arc<String>
        }
        impl ClosureGroup___123 {
            fn fn__9026(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("unionAllSql: {}", self.s__1190.clone()));
            }
        }
        let closure_group = ClosureGroup___123 {
            s__1190: s__1190.clone()
        };
        let fn__9026 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__9026())
        };
        test___113.assert(t___9036, fn__9026.clone());
        test___113.soft_fail_to_hard()
    }
    #[test]
    fn intersectSql__1755() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___114 = temper_std::testing::Test::new();
        let mut t___9015: SafeIdentifier = sid__505("users");
        let mut t___9016: SafeIdentifier = sid__505("email");
        let a__1192: Query = from(t___9015.clone()).select([t___9016.clone()]);
        let mut t___9018: SafeIdentifier = sid__505("subscribers");
        let mut t___9019: SafeIdentifier = sid__505("email");
        let b__1193: Query = from(t___9018.clone()).select([t___9019.clone()]);
        let s__1194: std::sync::Arc<String> = intersect_sql(a__1192.clone(), b__1193.clone()).to_string();
        let mut t___9024: bool = Some(s__1194.as_str()) == Some("(SELECT email FROM users) INTERSECT (SELECT email FROM subscribers)");
        #[derive(Clone)]
        struct ClosureGroup___124 {
            s__1194: std::sync::Arc<String>
        }
        impl ClosureGroup___124 {
            fn fn__9014(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("intersectSql: {}", self.s__1194.clone()));
            }
        }
        let closure_group = ClosureGroup___124 {
            s__1194: s__1194.clone()
        };
        let fn__9014 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__9014())
        };
        test___114.assert(t___9024, fn__9014.clone());
        test___114.soft_fail_to_hard()
    }
    #[test]
    fn exceptSql__1756() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___115 = temper_std::testing::Test::new();
        let mut t___9003: SafeIdentifier = sid__505("users");
        let mut t___9004: SafeIdentifier = sid__505("id");
        let a__1196: Query = from(t___9003.clone()).select([t___9004.clone()]);
        let mut t___9006: SafeIdentifier = sid__505("banned");
        let mut t___9007: SafeIdentifier = sid__505("id");
        let b__1197: Query = from(t___9006.clone()).select([t___9007.clone()]);
        let s__1198: std::sync::Arc<String> = except_sql(a__1196.clone(), b__1197.clone()).to_string();
        let mut t___9012: bool = Some(s__1198.as_str()) == Some("(SELECT id FROM users) EXCEPT (SELECT id FROM banned)");
        #[derive(Clone)]
        struct ClosureGroup___125 {
            s__1198: std::sync::Arc<String>
        }
        impl ClosureGroup___125 {
            fn fn__9002(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("exceptSql: {}", self.s__1198.clone()));
            }
        }
        let closure_group = ClosureGroup___125 {
            s__1198: s__1198.clone()
        };
        let fn__9002 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__9002())
        };
        test___115.assert(t___9012, fn__9002.clone());
        test___115.soft_fail_to_hard()
    }
    #[test]
    fn subqueryWithAlias__1757() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___116 = temper_std::testing::Test::new();
        let mut t___8988: SafeIdentifier = sid__505("orders");
        let mut t___8989: SafeIdentifier = sid__505("user_id");
        let mut t___8990: Query = from(t___8988.clone()).select([t___8989.clone()]);
        let mut t___8991: SqlBuilder = SqlBuilder::new();
        t___8991.append_safe("total > ");
        t___8991.append_int32(100);
        let inner__1200: Query = t___8990.r#where(t___8991.accumulated());
        let s__1201: std::sync::Arc<String> = subquery(inner__1200.clone(), sid__505("big_orders")).to_string();
        let mut t___9000: bool = Some(s__1201.as_str()) == Some("(SELECT user_id FROM orders WHERE total > 100) AS big_orders");
        #[derive(Clone)]
        struct ClosureGroup___126 {
            s__1201: std::sync::Arc<String>
        }
        impl ClosureGroup___126 {
            fn fn__8987(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("subquery: {}", self.s__1201.clone()));
            }
        }
        let closure_group = ClosureGroup___126 {
            s__1201: s__1201.clone()
        };
        let fn__8987 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__8987())
        };
        test___116.assert(t___9000, fn__8987.clone());
        test___116.soft_fail_to_hard()
    }
    #[test]
    fn existsSql__1759() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___117 = temper_std::testing::Test::new();
        let mut t___8977: SafeIdentifier = sid__505("orders");
        let mut t___8978: SqlBuilder = SqlBuilder::new();
        t___8978.append_safe("orders.user_id = users.id");
        let mut t___8980: SqlFragment = t___8978.accumulated();
        let inner__1203: Query = from(t___8977.clone()).r#where(t___8980.clone());
        let s__1204: std::sync::Arc<String> = exists_sql(inner__1203.clone()).to_string();
        let mut t___8985: bool = Some(s__1204.as_str()) == Some("EXISTS (SELECT * FROM orders WHERE orders.user_id = users.id)");
        #[derive(Clone)]
        struct ClosureGroup___127 {
            s__1204: std::sync::Arc<String>
        }
        impl ClosureGroup___127 {
            fn fn__8976(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("existsSql: {}", self.s__1204.clone()));
            }
        }
        let closure_group = ClosureGroup___127 {
            s__1204: s__1204.clone()
        };
        let fn__8976 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__8976())
        };
        test___117.assert(t___8985, fn__8976.clone());
        test___117.soft_fail_to_hard()
    }
    #[test]
    fn whereInSubquery__1761() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___118 = temper_std::testing::Test::new();
        let mut t___8960: SafeIdentifier = sid__505("orders");
        let mut t___8961: SafeIdentifier = sid__505("user_id");
        let mut t___8962: Query = from(t___8960.clone()).select([t___8961.clone()]);
        let mut t___8963: SqlBuilder = SqlBuilder::new();
        t___8963.append_safe("total > ");
        t___8963.append_int32(1000);
        let sub__1206: Query = t___8962.r#where(t___8963.accumulated());
        let mut t___8968: SafeIdentifier = sid__505("users");
        let mut t___8969: SafeIdentifier = sid__505("id");
        let q__1207: Query = from(t___8968.clone()).where_in_subquery(t___8969.clone(), sub__1206.clone());
        let s__1208: std::sync::Arc<String> = q__1207.to_sql().to_string();
        let mut t___8974: bool = Some(s__1208.as_str()) == Some("SELECT * FROM users WHERE id IN (SELECT user_id FROM orders WHERE total > 1000)");
        #[derive(Clone)]
        struct ClosureGroup___128 {
            s__1208: std::sync::Arc<String>
        }
        impl ClosureGroup___128 {
            fn fn__8959(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("whereInSubquery: {}", self.s__1208.clone()));
            }
        }
        let closure_group = ClosureGroup___128 {
            s__1208: s__1208.clone()
        };
        let fn__8959 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__8959())
        };
        test___118.assert(t___8974, fn__8959.clone());
        test___118.soft_fail_to_hard()
    }
    #[test]
    fn setOperationWithWhereOnEachSide__1763() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___119 = temper_std::testing::Test::new();
        let mut t___8937: SafeIdentifier = sid__505("users");
        let mut t___8938: SqlBuilder = SqlBuilder::new();
        t___8938.append_safe("age > ");
        t___8938.append_int32(18);
        let mut t___8941: SqlFragment = t___8938.accumulated();
        let mut t___8942: Query = from(t___8937.clone()).r#where(t___8941.clone());
        let mut t___8943: SqlBuilder = SqlBuilder::new();
        t___8943.append_safe("active = ");
        t___8943.append_boolean(true);
        let a__1210: Query = t___8942.r#where(t___8943.accumulated());
        let mut t___8948: SafeIdentifier = sid__505("users");
        let mut t___8949: SqlBuilder = SqlBuilder::new();
        t___8949.append_safe("role = ");
        t___8949.append_string("vip");
        let mut t___8952: SqlFragment = t___8949.accumulated();
        let b__1211: Query = from(t___8948.clone()).r#where(t___8952.clone());
        let s__1212: std::sync::Arc<String> = union_sql(a__1210.clone(), b__1211.clone()).to_string();
        let mut t___8957: bool = Some(s__1212.as_str()) == Some("(SELECT * FROM users WHERE age > 18 AND active = TRUE) UNION (SELECT * FROM users WHERE role = 'vip')");
        #[derive(Clone)]
        struct ClosureGroup___129 {
            s__1212: std::sync::Arc<String>
        }
        impl ClosureGroup___129 {
            fn fn__8936(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("union with where: {}", self.s__1212.clone()));
            }
        }
        let closure_group = ClosureGroup___129 {
            s__1212: s__1212.clone()
        };
        let fn__8936 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__8936())
        };
        test___119.assert(t___8957, fn__8936.clone());
        test___119.soft_fail_to_hard()
    }
    #[test]
    fn whereInSubqueryChainedWithWhere__1767() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___120 = temper_std::testing::Test::new();
        let mut t___8920: SafeIdentifier = sid__505("orders");
        let mut t___8921: SafeIdentifier = sid__505("user_id");
        let sub__1214: Query = from(t___8920.clone()).select([t___8921.clone()]);
        let mut t___8923: SafeIdentifier = sid__505("users");
        let mut t___8924: SqlBuilder = SqlBuilder::new();
        t___8924.append_safe("active = ");
        t___8924.append_boolean(true);
        let mut t___8927: SqlFragment = t___8924.accumulated();
        let q__1215: Query = from(t___8923.clone()).r#where(t___8927.clone()).where_in_subquery(sid__505("id"), sub__1214.clone());
        let s__1216: std::sync::Arc<String> = q__1215.to_sql().to_string();
        let mut t___8934: bool = Some(s__1216.as_str()) == Some("SELECT * FROM users WHERE active = TRUE AND id IN (SELECT user_id FROM orders)");
        #[derive(Clone)]
        struct ClosureGroup___130 {
            s__1216: std::sync::Arc<String>
        }
        impl ClosureGroup___130 {
            fn fn__8919(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("whereInSubquery chained: {}", self.s__1216.clone()));
            }
        }
        let closure_group = ClosureGroup___130 {
            s__1216: s__1216.clone()
        };
        let fn__8919 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__8919())
        };
        test___120.assert(t___8934, fn__8919.clone());
        test___120.soft_fail_to_hard()
    }
    #[test]
    fn existsSqlUsedInWhere__1769() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___121 = temper_std::testing::Test::new();
        let mut t___8906: SafeIdentifier = sid__505("orders");
        let mut t___8907: SqlBuilder = SqlBuilder::new();
        t___8907.append_safe("orders.user_id = users.id");
        let mut t___8909: SqlFragment = t___8907.accumulated();
        let sub__1218: Query = from(t___8906.clone()).r#where(t___8909.clone());
        let mut t___8911: SafeIdentifier = sid__505("users");
        let mut t___8912: SqlFragment = exists_sql(sub__1218.clone());
        let q__1219: Query = from(t___8911.clone()).r#where(t___8912.clone());
        let s__1220: std::sync::Arc<String> = q__1219.to_sql().to_string();
        let mut t___8917: bool = Some(s__1220.as_str()) == Some("SELECT * FROM users WHERE EXISTS (SELECT * FROM orders WHERE orders.user_id = users.id)");
        #[derive(Clone)]
        struct ClosureGroup___131 {
            s__1220: std::sync::Arc<String>
        }
        impl ClosureGroup___131 {
            fn fn__8905(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("exists in where: {}", self.s__1220.clone()));
            }
        }
        let closure_group = ClosureGroup___131 {
            s__1220: s__1220.clone()
        };
        let fn__8905 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__8905())
        };
        test___121.assert(t___8917, fn__8905.clone());
        test___121.soft_fail_to_hard()
    }
    #[test]
    fn updateQueryBasic__1771() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___122 = temper_std::testing::Test::new();
        let mut t___8892: SafeIdentifier;
        let mut t___8893: SafeIdentifier;
        let mut t___8894: SqlString;
        let mut t___8895: UpdateQuery;
        let mut t___8896: SqlBuilder;
        let mut t___4443: SqlFragment;
        let q__1222: SqlFragment;
        'ok___10991: {
            'orelse___1916: {
                t___8892 = sid__505("users");
                t___8893 = sid__505("name");
                t___8894 = SqlString::new("Alice");
                t___8895 = update(t___8892.clone()).set(t___8893.clone(), SqlPart::new(t___8894.clone()));
                t___8896 = SqlBuilder::new();
                t___8896.append_safe("id = ");
                t___8896.append_int32(1);
                t___4443 = match t___8895.r#where(t___8896.accumulated()).to_sql() {
                    Ok(x) => x,
                    _ => break 'orelse___1916
                };
                q__1222 = t___4443.clone();
                break 'ok___10991;
            }
            q__1222 = panic!();
        }
        let mut t___8903: bool = Some(q__1222.to_string().as_str()) == Some("UPDATE users SET name = 'Alice' WHERE id = 1");
        #[derive(Clone)]
        struct ClosureGroup___132 {}
        impl ClosureGroup___132 {
            fn fn__8891(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("update basic".to_string());
            }
        }
        let closure_group = ClosureGroup___132 {};
        let fn__8891 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__8891())
        };
        test___122.assert(t___8903, fn__8891.clone());
        test___122.soft_fail_to_hard()
    }
    #[test]
    fn updateQueryMultipleSet__1773() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___123 = temper_std::testing::Test::new();
        let mut t___8875: SafeIdentifier;
        let mut t___8876: SafeIdentifier;
        let mut t___8877: SqlString;
        let mut t___8881: UpdateQuery;
        let mut t___8882: SqlBuilder;
        let mut t___4428: SqlFragment;
        let q__1224: SqlFragment;
        'ok___10992: {
            'orelse___1917: {
                t___8875 = sid__505("users");
                t___8876 = sid__505("name");
                t___8877 = SqlString::new("Bob");
                t___8881 = update(t___8875.clone()).set(t___8876.clone(), SqlPart::new(t___8877.clone())).set(sid__505("age"), SqlPart::new(SqlInt32::new(30)));
                t___8882 = SqlBuilder::new();
                t___8882.append_safe("id = ");
                t___8882.append_int32(2);
                t___4428 = match t___8881.r#where(t___8882.accumulated()).to_sql() {
                    Ok(x) => x,
                    _ => break 'orelse___1917
                };
                q__1224 = t___4428.clone();
                break 'ok___10992;
            }
            q__1224 = panic!();
        }
        let mut t___8889: bool = Some(q__1224.to_string().as_str()) == Some("UPDATE users SET name = 'Bob', age = 30 WHERE id = 2");
        #[derive(Clone)]
        struct ClosureGroup___133 {}
        impl ClosureGroup___133 {
            fn fn__8874(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("update multi set".to_string());
            }
        }
        let closure_group = ClosureGroup___133 {};
        let fn__8874 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__8874())
        };
        test___123.assert(t___8889, fn__8874.clone());
        test___123.soft_fail_to_hard()
    }
    #[test]
    fn updateQueryMultipleWhere__1775() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___124 = temper_std::testing::Test::new();
        let mut t___8856: SafeIdentifier;
        let mut t___8857: SafeIdentifier;
        let mut t___8858: SqlBoolean;
        let mut t___8859: UpdateQuery;
        let mut t___8860: SqlBuilder;
        let mut t___8864: UpdateQuery;
        let mut t___8865: SqlBuilder;
        let mut t___4410: SqlFragment;
        let q__1226: SqlFragment;
        'ok___10993: {
            'orelse___1918: {
                t___8856 = sid__505("users");
                t___8857 = sid__505("active");
                t___8858 = SqlBoolean::new(false);
                t___8859 = update(t___8856.clone()).set(t___8857.clone(), SqlPart::new(t___8858.clone()));
                t___8860 = SqlBuilder::new();
                t___8860.append_safe("age < ");
                t___8860.append_int32(18);
                t___8864 = t___8859.r#where(t___8860.accumulated());
                t___8865 = SqlBuilder::new();
                t___8865.append_safe("role = ");
                t___8865.append_string("guest");
                t___4410 = match t___8864.r#where(t___8865.accumulated()).to_sql() {
                    Ok(x) => x,
                    _ => break 'orelse___1918
                };
                q__1226 = t___4410.clone();
                break 'ok___10993;
            }
            q__1226 = panic!();
        }
        let mut t___8872: bool = Some(q__1226.to_string().as_str()) == Some("UPDATE users SET active = FALSE WHERE age < 18 AND role = 'guest'");
        #[derive(Clone)]
        struct ClosureGroup___134 {}
        impl ClosureGroup___134 {
            fn fn__8855(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("update multi where".to_string());
            }
        }
        let closure_group = ClosureGroup___134 {};
        let fn__8855 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__8855())
        };
        test___124.assert(t___8872, fn__8855.clone());
        test___124.soft_fail_to_hard()
    }
    #[test]
    fn updateQueryOrWhere__1778() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___125 = temper_std::testing::Test::new();
        let mut t___8837: SafeIdentifier;
        let mut t___8838: SafeIdentifier;
        let mut t___8839: SqlString;
        let mut t___8840: UpdateQuery;
        let mut t___8841: SqlBuilder;
        let mut t___8845: UpdateQuery;
        let mut t___8846: SqlBuilder;
        let mut t___4389: SqlFragment;
        let q__1228: SqlFragment;
        'ok___10994: {
            'orelse___1919: {
                t___8837 = sid__505("users");
                t___8838 = sid__505("status");
                t___8839 = SqlString::new("banned");
                t___8840 = update(t___8837.clone()).set(t___8838.clone(), SqlPart::new(t___8839.clone()));
                t___8841 = SqlBuilder::new();
                t___8841.append_safe("spam_count > ");
                t___8841.append_int32(10);
                t___8845 = t___8840.r#where(t___8841.accumulated());
                t___8846 = SqlBuilder::new();
                t___8846.append_safe("reported = ");
                t___8846.append_boolean(true);
                t___4389 = match t___8845.or_where(t___8846.accumulated()).to_sql() {
                    Ok(x) => x,
                    _ => break 'orelse___1919
                };
                q__1228 = t___4389.clone();
                break 'ok___10994;
            }
            q__1228 = panic!();
        }
        let mut t___8853: bool = Some(q__1228.to_string().as_str()) == Some("UPDATE users SET status = 'banned' WHERE spam_count > 10 OR reported = TRUE");
        #[derive(Clone)]
        struct ClosureGroup___135 {}
        impl ClosureGroup___135 {
            fn fn__8836(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("update orWhere".to_string());
            }
        }
        let closure_group = ClosureGroup___135 {};
        let fn__8836 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__8836())
        };
        test___125.assert(t___8853, fn__8836.clone());
        test___125.soft_fail_to_hard()
    }
    #[test]
    fn updateQueryBubblesWithoutWhere__1781() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___126 = temper_std::testing::Test::new();
        let mut t___8830: SafeIdentifier;
        let mut t___8831: SafeIdentifier;
        let mut t___8832: SqlInt32;
        let didBubble__1230: bool;
        'ok___10995: {
            'orelse___1920: {
                t___8830 = sid__505("users");
                t___8831 = sid__505("x");
                t___8832 = SqlInt32::new(1);
                match update(t___8830.clone()).set(t___8831.clone(), SqlPart::new(t___8832.clone())).to_sql() {
                    Ok(x) => x,
                    _ => break 'orelse___1920
                };
                didBubble__1230 = false;
                break 'ok___10995;
            }
            didBubble__1230 = true;
        }
        #[derive(Clone)]
        struct ClosureGroup___136 {}
        impl ClosureGroup___136 {
            fn fn__8829(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("update without WHERE should bubble".to_string());
            }
        }
        let closure_group = ClosureGroup___136 {};
        let fn__8829 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__8829())
        };
        test___126.assert(didBubble__1230, fn__8829.clone());
        test___126.soft_fail_to_hard()
    }
    #[test]
    fn updateQueryBubblesWithoutSet__1782() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___127 = temper_std::testing::Test::new();
        let mut t___8821: SafeIdentifier;
        let mut t___8822: SqlBuilder;
        let mut t___8825: SqlFragment;
        let didBubble__1232: bool;
        'ok___10996: {
            'orelse___1921: {
                t___8821 = sid__505("users");
                t___8822 = SqlBuilder::new();
                t___8822.append_safe("id = ");
                t___8822.append_int32(1);
                t___8825 = t___8822.accumulated();
                match update(t___8821.clone()).r#where(t___8825.clone()).to_sql() {
                    Ok(x) => x,
                    _ => break 'orelse___1921
                };
                didBubble__1232 = false;
                break 'ok___10996;
            }
            didBubble__1232 = true;
        }
        #[derive(Clone)]
        struct ClosureGroup___137 {}
        impl ClosureGroup___137 {
            fn fn__8820(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("update without SET should bubble".to_string());
            }
        }
        let closure_group = ClosureGroup___137 {};
        let fn__8820 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__8820())
        };
        test___127.assert(didBubble__1232, fn__8820.clone());
        test___127.soft_fail_to_hard()
    }
    #[test]
    fn updateQueryWithLimit__1784() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___128 = temper_std::testing::Test::new();
        let mut t___8807: SafeIdentifier;
        let mut t___8808: SafeIdentifier;
        let mut t___8809: SqlBoolean;
        let mut t___8810: UpdateQuery;
        let mut t___8811: SqlBuilder;
        let mut t___4352: UpdateQuery;
        let mut t___4353: SqlFragment;
        let q__1234: SqlFragment;
        'ok___10997: {
            'orelse___1922: {
                t___8807 = sid__505("users");
                t___8808 = sid__505("active");
                t___8809 = SqlBoolean::new(false);
                t___8810 = update(t___8807.clone()).set(t___8808.clone(), SqlPart::new(t___8809.clone()));
                t___8811 = SqlBuilder::new();
                t___8811.append_safe("last_login < ");
                t___8811.append_string("2024-01-01");
                t___4352 = match t___8810.r#where(t___8811.accumulated()).limit(100) {
                    Ok(x) => x,
                    _ => break 'orelse___1922
                };
                t___4353 = match t___4352.to_sql() {
                    Ok(x) => x,
                    _ => break 'orelse___1922
                };
                q__1234 = t___4353.clone();
                break 'ok___10997;
            }
            q__1234 = panic!();
        }
        let mut t___8818: bool = Some(q__1234.to_string().as_str()) == Some("UPDATE users SET active = FALSE WHERE last_login < '2024-01-01' LIMIT 100");
        #[derive(Clone)]
        struct ClosureGroup___138 {}
        impl ClosureGroup___138 {
            fn fn__8806(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("update limit".to_string());
            }
        }
        let closure_group = ClosureGroup___138 {};
        let fn__8806 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__8806())
        };
        test___128.assert(t___8818, fn__8806.clone());
        test___128.soft_fail_to_hard()
    }
    #[test]
    fn updateQueryEscaping__1786() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___129 = temper_std::testing::Test::new();
        let mut t___8793: SafeIdentifier;
        let mut t___8794: SafeIdentifier;
        let mut t___8795: SqlString;
        let mut t___8796: UpdateQuery;
        let mut t___8797: SqlBuilder;
        let mut t___4337: SqlFragment;
        let q__1236: SqlFragment;
        'ok___10998: {
            'orelse___1923: {
                t___8793 = sid__505("users");
                t___8794 = sid__505("bio");
                t___8795 = SqlString::new("It's a test");
                t___8796 = update(t___8793.clone()).set(t___8794.clone(), SqlPart::new(t___8795.clone()));
                t___8797 = SqlBuilder::new();
                t___8797.append_safe("id = ");
                t___8797.append_int32(1);
                t___4337 = match t___8796.r#where(t___8797.accumulated()).to_sql() {
                    Ok(x) => x,
                    _ => break 'orelse___1923
                };
                q__1236 = t___4337.clone();
                break 'ok___10998;
            }
            q__1236 = panic!();
        }
        let mut t___8804: bool = Some(q__1236.to_string().as_str()) == Some("UPDATE users SET bio = 'It''s a test' WHERE id = 1");
        #[derive(Clone)]
        struct ClosureGroup___139 {}
        impl ClosureGroup___139 {
            fn fn__8792(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("update escaping".to_string());
            }
        }
        let closure_group = ClosureGroup___139 {};
        let fn__8792 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__8792())
        };
        test___129.assert(t___8804, fn__8792.clone());
        test___129.soft_fail_to_hard()
    }
    #[test]
    fn deleteQueryBasic__1788() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___130 = temper_std::testing::Test::new();
        let mut t___8782: SafeIdentifier;
        let mut t___8783: SqlBuilder;
        let mut t___8786: SqlFragment;
        let mut t___4322: SqlFragment;
        let q__1238: SqlFragment;
        'ok___10999: {
            'orelse___1924: {
                t___8782 = sid__505("users");
                t___8783 = SqlBuilder::new();
                t___8783.append_safe("id = ");
                t___8783.append_int32(1);
                t___8786 = t___8783.accumulated();
                t___4322 = match delete_from(t___8782.clone()).r#where(t___8786.clone()).to_sql() {
                    Ok(x) => x,
                    _ => break 'orelse___1924
                };
                q__1238 = t___4322.clone();
                break 'ok___10999;
            }
            q__1238 = panic!();
        }
        let mut t___8790: bool = Some(q__1238.to_string().as_str()) == Some("DELETE FROM users WHERE id = 1");
        #[derive(Clone)]
        struct ClosureGroup___140 {}
        impl ClosureGroup___140 {
            fn fn__8781(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("delete basic".to_string());
            }
        }
        let closure_group = ClosureGroup___140 {};
        let fn__8781 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__8781())
        };
        test___130.assert(t___8790, fn__8781.clone());
        test___130.soft_fail_to_hard()
    }
    #[test]
    fn deleteQueryMultipleWhere__1790() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___131 = temper_std::testing::Test::new();
        let mut t___8766: SafeIdentifier;
        let mut t___8767: SqlBuilder;
        let mut t___8770: SqlFragment;
        let mut t___8771: DeleteQuery;
        let mut t___8772: SqlBuilder;
        let mut t___4310: SqlFragment;
        let q__1240: SqlFragment;
        'ok___11000: {
            'orelse___1925: {
                t___8766 = sid__505("logs");
                t___8767 = SqlBuilder::new();
                t___8767.append_safe("created_at < ");
                t___8767.append_string("2024-01-01");
                t___8770 = t___8767.accumulated();
                t___8771 = delete_from(t___8766.clone()).r#where(t___8770.clone());
                t___8772 = SqlBuilder::new();
                t___8772.append_safe("level = ");
                t___8772.append_string("debug");
                t___4310 = match t___8771.r#where(t___8772.accumulated()).to_sql() {
                    Ok(x) => x,
                    _ => break 'orelse___1925
                };
                q__1240 = t___4310.clone();
                break 'ok___11000;
            }
            q__1240 = panic!();
        }
        let mut t___8779: bool = Some(q__1240.to_string().as_str()) == Some("DELETE FROM logs WHERE created_at < '2024-01-01' AND level = 'debug'");
        #[derive(Clone)]
        struct ClosureGroup___141 {}
        impl ClosureGroup___141 {
            fn fn__8765(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("delete multi where".to_string());
            }
        }
        let closure_group = ClosureGroup___141 {};
        let fn__8765 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__8765())
        };
        test___131.assert(t___8779, fn__8765.clone());
        test___131.soft_fail_to_hard()
    }
    #[test]
    fn deleteQueryBubblesWithoutWhere__1793() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___132 = temper_std::testing::Test::new();
        let didBubble__1242: bool;
        'ok___11001: {
            'orelse___1926: {
                match delete_from(sid__505("users")).to_sql() {
                    Ok(x) => x,
                    _ => break 'orelse___1926
                };
                didBubble__1242 = false;
                break 'ok___11001;
            }
            didBubble__1242 = true;
        }
        #[derive(Clone)]
        struct ClosureGroup___142 {}
        impl ClosureGroup___142 {
            fn fn__8761(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("delete without WHERE should bubble".to_string());
            }
        }
        let closure_group = ClosureGroup___142 {};
        let fn__8761 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__8761())
        };
        test___132.assert(didBubble__1242, fn__8761.clone());
        test___132.soft_fail_to_hard()
    }
    #[test]
    fn deleteQueryOrWhere__1794() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___133 = temper_std::testing::Test::new();
        let mut t___8746: SafeIdentifier;
        let mut t___8747: SqlBuilder;
        let mut t___8750: SqlFragment;
        let mut t___8751: DeleteQuery;
        let mut t___8752: SqlBuilder;
        let mut t___4289: SqlFragment;
        let q__1244: SqlFragment;
        'ok___11002: {
            'orelse___1927: {
                t___8746 = sid__505("sessions");
                t___8747 = SqlBuilder::new();
                t___8747.append_safe("expired = ");
                t___8747.append_boolean(true);
                t___8750 = t___8747.accumulated();
                t___8751 = delete_from(t___8746.clone()).r#where(t___8750.clone());
                t___8752 = SqlBuilder::new();
                t___8752.append_safe("created_at < ");
                t___8752.append_string("2023-01-01");
                t___4289 = match t___8751.or_where(t___8752.accumulated()).to_sql() {
                    Ok(x) => x,
                    _ => break 'orelse___1927
                };
                q__1244 = t___4289.clone();
                break 'ok___11002;
            }
            q__1244 = panic!();
        }
        let mut t___8759: bool = Some(q__1244.to_string().as_str()) == Some("DELETE FROM sessions WHERE expired = TRUE OR created_at < '2023-01-01'");
        #[derive(Clone)]
        struct ClosureGroup___143 {}
        impl ClosureGroup___143 {
            fn fn__8745(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("delete orWhere".to_string());
            }
        }
        let closure_group = ClosureGroup___143 {};
        let fn__8745 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__8745())
        };
        test___133.assert(t___8759, fn__8745.clone());
        test___133.soft_fail_to_hard()
    }
    #[test]
    fn deleteQueryWithLimit__1797() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___134 = temper_std::testing::Test::new();
        let mut t___8735: SafeIdentifier;
        let mut t___8736: SqlBuilder;
        let mut t___8739: SqlFragment;
        let mut t___4270: DeleteQuery;
        let mut t___4271: SqlFragment;
        let q__1246: SqlFragment;
        'ok___11003: {
            'orelse___1928: {
                t___8735 = sid__505("logs");
                t___8736 = SqlBuilder::new();
                t___8736.append_safe("level = ");
                t___8736.append_string("debug");
                t___8739 = t___8736.accumulated();
                t___4270 = match delete_from(t___8735.clone()).r#where(t___8739.clone()).limit(1000) {
                    Ok(x) => x,
                    _ => break 'orelse___1928
                };
                t___4271 = match t___4270.to_sql() {
                    Ok(x) => x,
                    _ => break 'orelse___1928
                };
                q__1246 = t___4271.clone();
                break 'ok___11003;
            }
            q__1246 = panic!();
        }
        let mut t___8743: bool = Some(q__1246.to_string().as_str()) == Some("DELETE FROM logs WHERE level = 'debug' LIMIT 1000");
        #[derive(Clone)]
        struct ClosureGroup___144 {}
        impl ClosureGroup___144 {
            fn fn__8734(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("delete limit".to_string());
            }
        }
        let closure_group = ClosureGroup___144 {};
        let fn__8734 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__8734())
        };
        test___134.assert(t___8743, fn__8734.clone());
        test___134.soft_fail_to_hard()
    }
    #[test]
    fn safeIdentifierAcceptsValidNames__1799() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___141 = temper_std::testing::Test::new();
        let mut t___4259: SafeIdentifier;
        let id__1284: SafeIdentifier;
        'ok___11004: {
            'orelse___1929: {
                t___4259 = match safe_identifier("user_name") {
                    Ok(x) => x,
                    _ => break 'orelse___1929
                };
                id__1284 = t___4259.clone();
                break 'ok___11004;
            }
            id__1284 = temper_core::cast::<SafeIdentifier>(panic!()).unwrap();
        }
        let mut t___8732: bool = Some(id__1284.sql_value().as_str()) == Some("user_name");
        #[derive(Clone)]
        struct ClosureGroup___145 {}
        impl ClosureGroup___145 {
            fn fn__8729(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("value should round-trip".to_string());
            }
        }
        let closure_group = ClosureGroup___145 {};
        let fn__8729 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__8729())
        };
        test___141.assert(t___8732, fn__8729.clone());
        test___141.soft_fail_to_hard()
    }
    #[test]
    fn safeIdentifierRejectsEmptyString__1800() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___142 = temper_std::testing::Test::new();
        let didBubble__1286: bool;
        'ok___11005: {
            'orelse___1930: {
                match safe_identifier("") {
                    Ok(x) => x,
                    _ => break 'orelse___1930
                };
                didBubble__1286 = false;
                break 'ok___11005;
            }
            didBubble__1286 = true;
        }
        #[derive(Clone)]
        struct ClosureGroup___146 {}
        impl ClosureGroup___146 {
            fn fn__8726(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("empty string should bubble".to_string());
            }
        }
        let closure_group = ClosureGroup___146 {};
        let fn__8726 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__8726())
        };
        test___142.assert(didBubble__1286, fn__8726.clone());
        test___142.soft_fail_to_hard()
    }
    #[test]
    fn safeIdentifierRejectsLeadingDigit__1801() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___143 = temper_std::testing::Test::new();
        let didBubble__1288: bool;
        'ok___11006: {
            'orelse___1931: {
                match safe_identifier("1col") {
                    Ok(x) => x,
                    _ => break 'orelse___1931
                };
                didBubble__1288 = false;
                break 'ok___11006;
            }
            didBubble__1288 = true;
        }
        #[derive(Clone)]
        struct ClosureGroup___147 {}
        impl ClosureGroup___147 {
            fn fn__8723(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("leading digit should bubble".to_string());
            }
        }
        let closure_group = ClosureGroup___147 {};
        let fn__8723 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__8723())
        };
        test___143.assert(didBubble__1288, fn__8723.clone());
        test___143.soft_fail_to_hard()
    }
    #[test]
    fn safeIdentifierRejectsSqlMetacharacters__1802() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___144 = temper_std::testing::Test::new();
        let cases__1290: temper_core::List<std::sync::Arc<String>> = std::sync::Arc::new(vec![std::sync::Arc::new("name); DROP TABLE".to_string()), std::sync::Arc::new("col'".to_string()), std::sync::Arc::new("a b".to_string()), std::sync::Arc::new("a-b".to_string()), std::sync::Arc::new("a.b".to_string()), std::sync::Arc::new("a;b".to_string())]);
        #[derive(Clone)]
        struct ClosureGroup___148 {
            test___144: temper_std::testing::Test
        }
        impl ClosureGroup___148 {
            fn fn__8720(& self, c__1291: impl temper_core::ToArcString) {
                let c__1291 = c__1291.to_arc_string();
                let didBubble__1292: bool;
                'ok___11007: {
                    'orelse___1932: {
                        match safe_identifier(c__1291.clone()) {
                            Ok(x) => x,
                            _ => break 'orelse___1932
                        };
                        didBubble__1292 = false;
                        break 'ok___11007;
                    }
                    didBubble__1292 = true;
                }
                #[derive(Clone)]
                struct ClosureGroup___149 {
                    c__1291: std::sync::Arc<String>
                }
                impl ClosureGroup___149 {
                    fn fn__8717(& self) -> std::sync::Arc<String> {
                        return std::sync::Arc::new(format!("should reject: {}", self.c__1291.clone()));
                    }
                }
                let closure_group = ClosureGroup___149 {
                    c__1291: c__1291.clone()
                };
                let fn__8717 = {
                    let closure_group = closure_group.clone();
                    std::sync::Arc::new(move | | closure_group.fn__8717())
                };
                self.test___144.assert(didBubble__1292, fn__8717.clone());
            }
        }
        let closure_group = ClosureGroup___148 {
            test___144: test___144.clone()
        };
        let fn__8720 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | c__1291: std::sync::Arc<String> | closure_group.fn__8720(c__1291))
        };
        temper_core::listed::list_for_each( & cases__1290, & ( * fn__8720.clone()));
        test___144.soft_fail_to_hard()
    }
    #[test]
    fn tableDefFieldLookupFound__1803() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___145 = temper_std::testing::Test::new();
        let mut t___4236: SafeIdentifier;
        let mut t___4237: SafeIdentifier;
        let mut t___4238: SafeIdentifier;
        let mut t___4239: SafeIdentifier;
        let mut t___4242: SafeIdentifier;
        let mut t___4243: SafeIdentifier;
        let mut t___4247: FieldDef;
        'ok___11008: {
            'orelse___1933: {
                t___4236 = match safe_identifier("users") {
                    Ok(x) => x,
                    _ => break 'orelse___1933
                };
                t___4237 = t___4236.clone();
                break 'ok___11008;
            }
            t___4237 = temper_core::cast::<SafeIdentifier>(panic!()).unwrap();
        }
        'ok___11009: {
            'orelse___1934: {
                t___4238 = match safe_identifier("name") {
                    Ok(x) => x,
                    _ => break 'orelse___1934
                };
                t___4239 = t___4238.clone();
                break 'ok___11009;
            }
            t___4239 = temper_core::cast::<SafeIdentifier>(panic!()).unwrap();
        }
        let mut t___8707: StringField = StringField::new();
        let mut t___8708: FieldDef = FieldDef::new(t___4239.clone(), FieldType::new(t___8707.clone()), false);
        'ok___11010: {
            'orelse___1935: {
                t___4242 = match safe_identifier("age") {
                    Ok(x) => x,
                    _ => break 'orelse___1935
                };
                t___4243 = t___4242.clone();
                break 'ok___11010;
            }
            t___4243 = temper_core::cast::<SafeIdentifier>(panic!()).unwrap();
        }
        let mut t___8709: IntField = IntField::new();
        let mut t___8710: FieldDef = FieldDef::new(t___4243.clone(), FieldType::new(t___8709.clone()), false);
        let td__1294: TableDef = TableDef::new(t___4237.clone(), [t___8708.clone(), t___8710.clone()]);
        let f__1295: FieldDef;
        'ok___11011: {
            'orelse___1936: {
                t___4247 = match td__1294.field("age") {
                    Ok(x) => x,
                    _ => break 'orelse___1936
                };
                f__1295 = t___4247.clone();
                break 'ok___11011;
            }
            f__1295 = panic!();
        }
        let mut t___8715: bool = Some(f__1295.name().sql_value().as_str()) == Some("age");
        #[derive(Clone)]
        struct ClosureGroup___150 {}
        impl ClosureGroup___150 {
            fn fn__8706(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("should find age field".to_string());
            }
        }
        let closure_group = ClosureGroup___150 {};
        let fn__8706 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__8706())
        };
        test___145.assert(t___8715, fn__8706.clone());
        test___145.soft_fail_to_hard()
    }
    #[test]
    fn tableDefFieldLookupNotFoundBubbles__1804() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___146 = temper_std::testing::Test::new();
        let mut t___4227: SafeIdentifier;
        let mut t___4228: SafeIdentifier;
        let mut t___4229: SafeIdentifier;
        let mut t___4230: SafeIdentifier;
        'ok___11012: {
            'orelse___1937: {
                t___4227 = match safe_identifier("users") {
                    Ok(x) => x,
                    _ => break 'orelse___1937
                };
                t___4228 = t___4227.clone();
                break 'ok___11012;
            }
            t___4228 = temper_core::cast::<SafeIdentifier>(panic!()).unwrap();
        }
        'ok___11013: {
            'orelse___1938: {
                t___4229 = match safe_identifier("name") {
                    Ok(x) => x,
                    _ => break 'orelse___1938
                };
                t___4230 = t___4229.clone();
                break 'ok___11013;
            }
            t___4230 = temper_core::cast::<SafeIdentifier>(panic!()).unwrap();
        }
        let mut t___8701: StringField = StringField::new();
        let mut t___8702: FieldDef = FieldDef::new(t___4230.clone(), FieldType::new(t___8701.clone()), false);
        let td__1297: TableDef = TableDef::new(t___4228.clone(), [t___8702.clone()]);
        let didBubble__1298: bool;
        'ok___11014: {
            'orelse___1939: {
                match td__1297.field("nonexistent") {
                    Ok(x) => x,
                    _ => break 'orelse___1939
                };
                didBubble__1298 = false;
                break 'ok___11014;
            }
            didBubble__1298 = true;
        }
        #[derive(Clone)]
        struct ClosureGroup___151 {}
        impl ClosureGroup___151 {
            fn fn__8700(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("unknown field should bubble".to_string());
            }
        }
        let closure_group = ClosureGroup___151 {};
        let fn__8700 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__8700())
        };
        test___146.assert(didBubble__1298, fn__8700.clone());
        test___146.soft_fail_to_hard()
    }
    #[test]
    fn fieldDefNullableFlag__1805() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___147 = temper_std::testing::Test::new();
        let mut t___4215: SafeIdentifier;
        let mut t___4216: SafeIdentifier;
        let mut t___4219: SafeIdentifier;
        let mut t___4220: SafeIdentifier;
        'ok___11015: {
            'orelse___1940: {
                t___4215 = match safe_identifier("email") {
                    Ok(x) => x,
                    _ => break 'orelse___1940
                };
                t___4216 = t___4215.clone();
                break 'ok___11015;
            }
            t___4216 = temper_core::cast::<SafeIdentifier>(panic!()).unwrap();
        }
        let mut t___8689: StringField = StringField::new();
        let required__1300: FieldDef = FieldDef::new(t___4216.clone(), FieldType::new(t___8689.clone()), false);
        'ok___11016: {
            'orelse___1941: {
                t___4219 = match safe_identifier("bio") {
                    Ok(x) => x,
                    _ => break 'orelse___1941
                };
                t___4220 = t___4219.clone();
                break 'ok___11016;
            }
            t___4220 = temper_core::cast::<SafeIdentifier>(panic!()).unwrap();
        }
        let mut t___8691: StringField = StringField::new();
        let optional__1301: FieldDef = FieldDef::new(t___4220.clone(), FieldType::new(t___8691.clone()), true);
        let mut t___8695: bool = ! required__1300.nullable();
        #[derive(Clone)]
        struct ClosureGroup___152 {}
        impl ClosureGroup___152 {
            fn fn__8688(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("required field should not be nullable".to_string());
            }
        }
        let closure_group = ClosureGroup___152 {};
        let fn__8688 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__8688())
        };
        test___147.assert(t___8695, fn__8688.clone());
        let mut t___8697: bool = optional__1301.nullable();
        #[derive(Clone)]
        struct ClosureGroup___153 {}
        impl ClosureGroup___153 {
            fn fn__8687(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("optional field should be nullable".to_string());
            }
        }
        let closure_group = ClosureGroup___153 {};
        let fn__8687 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__8687())
        };
        test___147.assert(t___8697, fn__8687.clone());
        test___147.soft_fail_to_hard()
    }
    #[test]
    fn stringEscaping__1806() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___151 = temper_std::testing::Test::new();
        #[derive(Clone)]
        struct ClosureGroup___154 {}
        impl ClosureGroup___154 {
            fn build__1427(& self, name__1429: impl temper_core::ToArcString) -> std::sync::Arc<String> {
                let name__1429 = name__1429.to_arc_string();
                let mut t___8669: SqlBuilder = SqlBuilder::new();
                t___8669.append_safe("select * from hi where name = ");
                t___8669.append_string(name__1429.clone());
                return t___8669.accumulated().to_string();
            }
            fn buildWrong__1428(& self, name__1431: impl temper_core::ToArcString) -> std::sync::Arc<String> {
                let name__1431 = name__1431.to_arc_string();
                return std::sync::Arc::new(format!("select * from hi where name = '{}'", name__1431.clone()));
            }
        }
        let closure_group = ClosureGroup___154 {};
        let build__1427 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | name__1429: std::sync::Arc<String> | closure_group.build__1427(name__1429))
        };
        let buildWrong__1428 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | name__1431: std::sync::Arc<String> | closure_group.buildWrong__1428(name__1431))
        };
        let actual___1808: std::sync::Arc<String> = build__1427(std::sync::Arc::new("world".to_string()));
        let mut t___8679: bool = Some(actual___1808.as_str()) == Some("select * from hi where name = 'world'");
        #[derive(Clone)]
        struct ClosureGroup___155 {
            actual___1808: std::sync::Arc<String>
        }
        impl ClosureGroup___155 {
            fn fn__8676(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected build(\"world\") == (select * from hi where name = 'world') not ({})", self.actual___1808.clone()));
            }
        }
        let closure_group = ClosureGroup___155 {
            actual___1808: actual___1808.clone()
        };
        let fn__8676 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__8676())
        };
        test___151.assert(t___8679, fn__8676.clone());
        let bobbyTables__1433: std::sync::Arc<String> = std::sync::Arc::new("Robert'); drop table hi;--".to_string());
        let actual___1810: std::sync::Arc<String> = build__1427(std::sync::Arc::new("Robert'); drop table hi;--".to_string()));
        let mut t___8683: bool = Some(actual___1810.as_str()) == Some("select * from hi where name = 'Robert''); drop table hi;--'");
        #[derive(Clone)]
        struct ClosureGroup___156 {
            actual___1810: std::sync::Arc<String>
        }
        impl ClosureGroup___156 {
            fn fn__8675(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected build(bobbyTables) == (select * from hi where name = 'Robert''); drop table hi;--') not ({})", self.actual___1810.clone()));
            }
        }
        let closure_group = ClosureGroup___156 {
            actual___1810: actual___1810.clone()
        };
        let fn__8675 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__8675())
        };
        test___151.assert(t___8683, fn__8675.clone());
        #[derive(Clone)]
        struct ClosureGroup___157 {}
        impl ClosureGroup___157 {
            fn fn__8674(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new("expected buildWrong(bobbyTables) == (select * from hi where name = 'Robert'); drop table hi;--') not (select * from hi where name = 'Robert'); drop table hi;--')".to_string());
            }
        }
        let closure_group = ClosureGroup___157 {};
        let fn__8674 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__8674())
        };
        test___151.assert(true, fn__8674.clone());
        test___151.soft_fail_to_hard()
    }
    #[test]
    fn stringEdgeCases__1814() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___152 = temper_std::testing::Test::new();
        let mut t___8637: SqlBuilder = SqlBuilder::new();
        t___8637.append_safe("v = ");
        t___8637.append_string("");
        let actual___1815: std::sync::Arc<String> = t___8637.accumulated().to_string();
        let mut t___8643: bool = Some(actual___1815.as_str()) == Some("v = ''");
        #[derive(Clone)]
        struct ClosureGroup___158 {
            actual___1815: std::sync::Arc<String>
        }
        impl ClosureGroup___158 {
            fn fn__8636(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"v = \", \\interpolate, \"\").toString() == (v = '') not ({})", self.actual___1815.clone()));
            }
        }
        let closure_group = ClosureGroup___158 {
            actual___1815: actual___1815.clone()
        };
        let fn__8636 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__8636())
        };
        test___152.assert(t___8643, fn__8636.clone());
        let mut t___8645: SqlBuilder = SqlBuilder::new();
        t___8645.append_safe("v = ");
        t___8645.append_string("a''b");
        let actual___1818: std::sync::Arc<String> = t___8645.accumulated().to_string();
        let mut t___8651: bool = Some(actual___1818.as_str()) == Some("v = 'a''''b'");
        #[derive(Clone)]
        struct ClosureGroup___159 {
            actual___1818: std::sync::Arc<String>
        }
        impl ClosureGroup___159 {
            fn fn__8635(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"v = \", \\interpolate, \"a''b\").toString() == (v = 'a''''b') not ({})", self.actual___1818.clone()));
            }
        }
        let closure_group = ClosureGroup___159 {
            actual___1818: actual___1818.clone()
        };
        let fn__8635 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__8635())
        };
        test___152.assert(t___8651, fn__8635.clone());
        let mut t___8653: SqlBuilder = SqlBuilder::new();
        t___8653.append_safe("v = ");
        t___8653.append_string("Hello 世界");
        let actual___1821: std::sync::Arc<String> = t___8653.accumulated().to_string();
        let mut t___8659: bool = Some(actual___1821.as_str()) == Some("v = 'Hello 世界'");
        #[derive(Clone)]
        struct ClosureGroup___160 {
            actual___1821: std::sync::Arc<String>
        }
        impl ClosureGroup___160 {
            fn fn__8634(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"v = \", \\interpolate, \"Hello 世界\").toString() == (v = 'Hello 世界') not ({})", self.actual___1821.clone()));
            }
        }
        let closure_group = ClosureGroup___160 {
            actual___1821: actual___1821.clone()
        };
        let fn__8634 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__8634())
        };
        test___152.assert(t___8659, fn__8634.clone());
        let mut t___8661: SqlBuilder = SqlBuilder::new();
        t___8661.append_safe("v = ");
        t___8661.append_string("Line1\x0aLine2");
        let actual___1824: std::sync::Arc<String> = t___8661.accumulated().to_string();
        let mut t___8667: bool = Some(actual___1824.as_str()) == Some("v = 'Line1\x0aLine2'");
        #[derive(Clone)]
        struct ClosureGroup___161 {
            actual___1824: std::sync::Arc<String>
        }
        impl ClosureGroup___161 {
            fn fn__8633(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"v = \", \\interpolate, \"Line1\\nLine2\").toString() == (v = 'Line1\x0aLine2') not ({})", self.actual___1824.clone()));
            }
        }
        let closure_group = ClosureGroup___161 {
            actual___1824: actual___1824.clone()
        };
        let fn__8633 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__8633())
        };
        test___152.assert(t___8667, fn__8633.clone());
        test___152.soft_fail_to_hard()
    }
    #[test]
    fn numbersAndBooleans__1827() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___153 = temper_std::testing::Test::new();
        let mut t___4160: temper_std::temporal::Date;
        let mut t___8608: SqlBuilder = SqlBuilder::new();
        t___8608.append_safe("select ");
        t___8608.append_int32(42);
        t___8608.append_safe(", ");
        t___8608.append_int64(43);
        t___8608.append_safe(", ");
        t___8608.append_float64(19.99f64);
        t___8608.append_safe(", ");
        t___8608.append_boolean(true);
        t___8608.append_safe(", ");
        t___8608.append_boolean(false);
        let actual___1828: std::sync::Arc<String> = t___8608.accumulated().to_string();
        let mut t___8622: bool = Some(actual___1828.as_str()) == Some("select 42, 43, 19.99, TRUE, FALSE");
        #[derive(Clone)]
        struct ClosureGroup___162 {
            actual___1828: std::sync::Arc<String>
        }
        impl ClosureGroup___162 {
            fn fn__8607(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"select \", \\interpolate, 42, \", \", \\interpolate, 43, \", \", \\interpolate, 19.99, \", \", \\interpolate, true, \", \", \\interpolate, false).toString() == (select 42, 43, 19.99, TRUE, FALSE) not ({})", self.actual___1828.clone()));
            }
        }
        let closure_group = ClosureGroup___162 {
            actual___1828: actual___1828.clone()
        };
        let fn__8607 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__8607())
        };
        test___153.assert(t___8622, fn__8607.clone());
        let date__1436: temper_std::temporal::Date;
        'ok___11017: {
            'orelse___1942: {
                t___4160 = match temper_std::temporal::Date::new(2024, 12, 25) {
                    Ok(x) => x,
                    _ => break 'orelse___1942
                };
                date__1436 = t___4160.clone();
                break 'ok___11017;
            }
            date__1436 = panic!();
        }
        let mut t___8624: SqlBuilder = SqlBuilder::new();
        t___8624.append_safe("insert into t values (");
        t___8624.append_date(date__1436.clone());
        t___8624.append_safe(")");
        let actual___1831: std::sync::Arc<String> = t___8624.accumulated().to_string();
        let mut t___8631: bool = Some(actual___1831.as_str()) == Some("insert into t values ('2024-12-25')");
        #[derive(Clone)]
        struct ClosureGroup___163 {
            actual___1831: std::sync::Arc<String>
        }
        impl ClosureGroup___163 {
            fn fn__8606(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"insert into t values (\", \\interpolate, date, \")\").toString() == (insert into t values ('2024-12-25')) not ({})", self.actual___1831.clone()));
            }
        }
        let closure_group = ClosureGroup___163 {
            actual___1831: actual___1831.clone()
        };
        let fn__8606 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__8606())
        };
        test___153.assert(t___8631, fn__8606.clone());
        test___153.soft_fail_to_hard()
    }
    #[test]
    fn lists__1834() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___154 = temper_std::testing::Test::new();
        let mut t___4132: temper_std::temporal::Date;
        let mut t___4133: temper_std::temporal::Date;
        let mut t___4134: temper_std::temporal::Date;
        let mut t___4135: temper_std::temporal::Date;
        let mut t___8552: SqlBuilder = SqlBuilder::new();
        t___8552.append_safe("v IN (");
        t___8552.append_string_list(temper_core::ToListed::to_listed([std::sync::Arc::new("a".to_string()), std::sync::Arc::new("b".to_string()), std::sync::Arc::new("c'd".to_string())]));
        t___8552.append_safe(")");
        let actual___1835: std::sync::Arc<String> = t___8552.accumulated().to_string();
        let mut t___8559: bool = Some(actual___1835.as_str()) == Some("v IN ('a', 'b', 'c''d')");
        #[derive(Clone)]
        struct ClosureGroup___164 {
            actual___1835: std::sync::Arc<String>
        }
        impl ClosureGroup___164 {
            fn fn__8551(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"v IN (\", \\interpolate, list(\"a\", \"b\", \"c'd\"), \")\").toString() == (v IN ('a', 'b', 'c''d')) not ({})", self.actual___1835.clone()));
            }
        }
        let closure_group = ClosureGroup___164 {
            actual___1835: actual___1835.clone()
        };
        let fn__8551 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__8551())
        };
        test___154.assert(t___8559, fn__8551.clone());
        let mut t___8561: SqlBuilder = SqlBuilder::new();
        t___8561.append_safe("v IN (");
        t___8561.append_int32_list(temper_core::ToListed::to_listed([1, 2, 3]));
        t___8561.append_safe(")");
        let actual___1838: std::sync::Arc<String> = t___8561.accumulated().to_string();
        let mut t___8568: bool = Some(actual___1838.as_str()) == Some("v IN (1, 2, 3)");
        #[derive(Clone)]
        struct ClosureGroup___165 {
            actual___1838: std::sync::Arc<String>
        }
        impl ClosureGroup___165 {
            fn fn__8550(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"v IN (\", \\interpolate, list(1, 2, 3), \")\").toString() == (v IN (1, 2, 3)) not ({})", self.actual___1838.clone()));
            }
        }
        let closure_group = ClosureGroup___165 {
            actual___1838: actual___1838.clone()
        };
        let fn__8550 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__8550())
        };
        test___154.assert(t___8568, fn__8550.clone());
        let mut t___8570: SqlBuilder = SqlBuilder::new();
        t___8570.append_safe("v IN (");
        t___8570.append_int64_list(temper_core::ToListed::to_listed([1, 2]));
        t___8570.append_safe(")");
        let actual___1841: std::sync::Arc<String> = t___8570.accumulated().to_string();
        let mut t___8577: bool = Some(actual___1841.as_str()) == Some("v IN (1, 2)");
        #[derive(Clone)]
        struct ClosureGroup___166 {
            actual___1841: std::sync::Arc<String>
        }
        impl ClosureGroup___166 {
            fn fn__8549(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"v IN (\", \\interpolate, list(1, 2), \")\").toString() == (v IN (1, 2)) not ({})", self.actual___1841.clone()));
            }
        }
        let closure_group = ClosureGroup___166 {
            actual___1841: actual___1841.clone()
        };
        let fn__8549 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__8549())
        };
        test___154.assert(t___8577, fn__8549.clone());
        let mut t___8579: SqlBuilder = SqlBuilder::new();
        t___8579.append_safe("v IN (");
        t___8579.append_float64_list(temper_core::ToListed::to_listed([1.0f64, 2.0f64]));
        t___8579.append_safe(")");
        let actual___1844: std::sync::Arc<String> = t___8579.accumulated().to_string();
        let mut t___8586: bool = Some(actual___1844.as_str()) == Some("v IN (1.0, 2.0)");
        #[derive(Clone)]
        struct ClosureGroup___167 {
            actual___1844: std::sync::Arc<String>
        }
        impl ClosureGroup___167 {
            fn fn__8548(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"v IN (\", \\interpolate, list(1.0, 2.0), \")\").toString() == (v IN (1.0, 2.0)) not ({})", self.actual___1844.clone()));
            }
        }
        let closure_group = ClosureGroup___167 {
            actual___1844: actual___1844.clone()
        };
        let fn__8548 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__8548())
        };
        test___154.assert(t___8586, fn__8548.clone());
        let mut t___8588: SqlBuilder = SqlBuilder::new();
        t___8588.append_safe("v IN (");
        t___8588.append_boolean_list(temper_core::ToListed::to_listed([true, false]));
        t___8588.append_safe(")");
        let actual___1847: std::sync::Arc<String> = t___8588.accumulated().to_string();
        let mut t___8595: bool = Some(actual___1847.as_str()) == Some("v IN (TRUE, FALSE)");
        #[derive(Clone)]
        struct ClosureGroup___168 {
            actual___1847: std::sync::Arc<String>
        }
        impl ClosureGroup___168 {
            fn fn__8547(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"v IN (\", \\interpolate, list(true, false), \")\").toString() == (v IN (TRUE, FALSE)) not ({})", self.actual___1847.clone()));
            }
        }
        let closure_group = ClosureGroup___168 {
            actual___1847: actual___1847.clone()
        };
        let fn__8547 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__8547())
        };
        test___154.assert(t___8595, fn__8547.clone());
        'ok___11018: {
            'orelse___1943: {
                t___4132 = match temper_std::temporal::Date::new(2024, 1, 1) {
                    Ok(x) => x,
                    _ => break 'orelse___1943
                };
                t___4133 = t___4132.clone();
                break 'ok___11018;
            }
            t___4133 = panic!();
        }
        'ok___11019: {
            'orelse___1944: {
                t___4134 = match temper_std::temporal::Date::new(2024, 12, 25) {
                    Ok(x) => x,
                    _ => break 'orelse___1944
                };
                t___4135 = t___4134.clone();
                break 'ok___11019;
            }
            t___4135 = panic!();
        }
        let dates__1438: temper_core::List<temper_std::temporal::Date> = std::sync::Arc::new(vec![t___4133.clone(), t___4135.clone()]);
        let mut t___8597: SqlBuilder = SqlBuilder::new();
        t___8597.append_safe("v IN (");
        t___8597.append_date_list(temper_core::ToListed::to_listed(dates__1438.clone()));
        t___8597.append_safe(")");
        let actual___1850: std::sync::Arc<String> = t___8597.accumulated().to_string();
        let mut t___8604: bool = Some(actual___1850.as_str()) == Some("v IN ('2024-01-01', '2024-12-25')");
        #[derive(Clone)]
        struct ClosureGroup___169 {
            actual___1850: std::sync::Arc<String>
        }
        impl ClosureGroup___169 {
            fn fn__8546(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"v IN (\", \\interpolate, dates, \")\").toString() == (v IN ('2024-01-01', '2024-12-25')) not ({})", self.actual___1850.clone()));
            }
        }
        let closure_group = ClosureGroup___169 {
            actual___1850: actual___1850.clone()
        };
        let fn__8546 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__8546())
        };
        test___154.assert(t___8604, fn__8546.clone());
        test___154.soft_fail_to_hard()
    }
    #[test]
    fn sqlFloat64_naNRendersAsNull__1853() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___155 = temper_std::testing::Test::new();
        let nan__1440: f64;
        nan__1440 = temper_core::float64::div(0.0f64, 0.0f64) ? ;
        let mut t___8538: SqlBuilder = SqlBuilder::new();
        t___8538.append_safe("v = ");
        t___8538.append_float64(nan__1440);
        let actual___1854: std::sync::Arc<String> = t___8538.accumulated().to_string();
        let mut t___8544: bool = Some(actual___1854.as_str()) == Some("v = NULL");
        #[derive(Clone)]
        struct ClosureGroup___170 {
            actual___1854: std::sync::Arc<String>
        }
        impl ClosureGroup___170 {
            fn fn__8537(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"v = \", \\interpolate, nan).toString() == (v = NULL) not ({})", self.actual___1854.clone()));
            }
        }
        let closure_group = ClosureGroup___170 {
            actual___1854: actual___1854.clone()
        };
        let fn__8537 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__8537())
        };
        test___155.assert(t___8544, fn__8537.clone());
        test___155.soft_fail_to_hard()
    }
    #[test]
    fn sqlFloat64_infinityRendersAsNull__1857() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___156 = temper_std::testing::Test::new();
        let inf__1442: f64;
        inf__1442 = temper_core::float64::div(1.0f64, 0.0f64) ? ;
        let mut t___8529: SqlBuilder = SqlBuilder::new();
        t___8529.append_safe("v = ");
        t___8529.append_float64(inf__1442);
        let actual___1858: std::sync::Arc<String> = t___8529.accumulated().to_string();
        let mut t___8535: bool = Some(actual___1858.as_str()) == Some("v = NULL");
        #[derive(Clone)]
        struct ClosureGroup___171 {
            actual___1858: std::sync::Arc<String>
        }
        impl ClosureGroup___171 {
            fn fn__8528(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"v = \", \\interpolate, inf).toString() == (v = NULL) not ({})", self.actual___1858.clone()));
            }
        }
        let closure_group = ClosureGroup___171 {
            actual___1858: actual___1858.clone()
        };
        let fn__8528 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__8528())
        };
        test___156.assert(t___8535, fn__8528.clone());
        test___156.soft_fail_to_hard()
    }
    #[test]
    fn sqlFloat64_negativeInfinityRendersAsNull__1861() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___157 = temper_std::testing::Test::new();
        let ninf__1444: f64;
        ninf__1444 = temper_core::float64::div(-1.0f64, 0.0f64) ? ;
        let mut t___8520: SqlBuilder = SqlBuilder::new();
        t___8520.append_safe("v = ");
        t___8520.append_float64(ninf__1444);
        let actual___1862: std::sync::Arc<String> = t___8520.accumulated().to_string();
        let mut t___8526: bool = Some(actual___1862.as_str()) == Some("v = NULL");
        #[derive(Clone)]
        struct ClosureGroup___172 {
            actual___1862: std::sync::Arc<String>
        }
        impl ClosureGroup___172 {
            fn fn__8519(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"v = \", \\interpolate, ninf).toString() == (v = NULL) not ({})", self.actual___1862.clone()));
            }
        }
        let closure_group = ClosureGroup___172 {
            actual___1862: actual___1862.clone()
        };
        let fn__8519 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__8519())
        };
        test___157.assert(t___8526, fn__8519.clone());
        test___157.soft_fail_to_hard()
    }
    #[test]
    fn sqlFloat64_normalValuesStillWork__1865() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___158 = temper_std::testing::Test::new();
        let mut t___8495: SqlBuilder = SqlBuilder::new();
        t___8495.append_safe("v = ");
        t___8495.append_float64(3.14f64);
        let actual___1866: std::sync::Arc<String> = t___8495.accumulated().to_string();
        let mut t___8501: bool = Some(actual___1866.as_str()) == Some("v = 3.14");
        #[derive(Clone)]
        struct ClosureGroup___173 {
            actual___1866: std::sync::Arc<String>
        }
        impl ClosureGroup___173 {
            fn fn__8494(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"v = \", \\interpolate, 3.14).toString() == (v = 3.14) not ({})", self.actual___1866.clone()));
            }
        }
        let closure_group = ClosureGroup___173 {
            actual___1866: actual___1866.clone()
        };
        let fn__8494 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__8494())
        };
        test___158.assert(t___8501, fn__8494.clone());
        let mut t___8503: SqlBuilder = SqlBuilder::new();
        t___8503.append_safe("v = ");
        t___8503.append_float64(0.0f64);
        let actual___1869: std::sync::Arc<String> = t___8503.accumulated().to_string();
        let mut t___8509: bool = Some(actual___1869.as_str()) == Some("v = 0.0");
        #[derive(Clone)]
        struct ClosureGroup___174 {
            actual___1869: std::sync::Arc<String>
        }
        impl ClosureGroup___174 {
            fn fn__8493(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"v = \", \\interpolate, 0.0).toString() == (v = 0.0) not ({})", self.actual___1869.clone()));
            }
        }
        let closure_group = ClosureGroup___174 {
            actual___1869: actual___1869.clone()
        };
        let fn__8493 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__8493())
        };
        test___158.assert(t___8509, fn__8493.clone());
        let mut t___8511: SqlBuilder = SqlBuilder::new();
        t___8511.append_safe("v = ");
        t___8511.append_float64(-42.5f64);
        let actual___1872: std::sync::Arc<String> = t___8511.accumulated().to_string();
        let mut t___8517: bool = Some(actual___1872.as_str()) == Some("v = -42.5");
        #[derive(Clone)]
        struct ClosureGroup___175 {
            actual___1872: std::sync::Arc<String>
        }
        impl ClosureGroup___175 {
            fn fn__8492(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"v = \", \\interpolate, -42.5).toString() == (v = -42.5) not ({})", self.actual___1872.clone()));
            }
        }
        let closure_group = ClosureGroup___175 {
            actual___1872: actual___1872.clone()
        };
        let fn__8492 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__8492())
        };
        test___158.assert(t___8517, fn__8492.clone());
        test___158.soft_fail_to_hard()
    }
    #[test]
    fn sqlDateRendersWithQuotes__1875() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___159 = temper_std::testing::Test::new();
        let mut t___4028: temper_std::temporal::Date;
        let d__1447: temper_std::temporal::Date;
        'ok___11020: {
            'orelse___1945: {
                t___4028 = match temper_std::temporal::Date::new(2024, 6, 15) {
                    Ok(x) => x,
                    _ => break 'orelse___1945
                };
                d__1447 = t___4028.clone();
                break 'ok___11020;
            }
            d__1447 = panic!();
        }
        let mut t___8484: SqlBuilder = SqlBuilder::new();
        t___8484.append_safe("v = ");
        t___8484.append_date(d__1447.clone());
        let actual___1876: std::sync::Arc<String> = t___8484.accumulated().to_string();
        let mut t___8490: bool = Some(actual___1876.as_str()) == Some("v = '2024-06-15'");
        #[derive(Clone)]
        struct ClosureGroup___176 {
            actual___1876: std::sync::Arc<String>
        }
        impl ClosureGroup___176 {
            fn fn__8483(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"v = \", \\interpolate, d).toString() == (v = '2024-06-15') not ({})", self.actual___1876.clone()));
            }
        }
        let closure_group = ClosureGroup___176 {
            actual___1876: actual___1876.clone()
        };
        let fn__8483 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__8483())
        };
        test___159.assert(t___8490, fn__8483.clone());
        test___159.soft_fail_to_hard()
    }
    #[test]
    fn nesting__1879() -> temper_core::Result<()> {
        crate::init(None);
        temper_std::init(None);
        let test___160 = temper_std::testing::Test::new();
        let name__1449: std::sync::Arc<String> = std::sync::Arc::new("Someone".to_string());
        let mut t___8452: SqlBuilder = SqlBuilder::new();
        t___8452.append_safe("where p.last_name = ");
        t___8452.append_string("Someone");
        let condition__1450: SqlFragment = t___8452.accumulated();
        let mut t___8456: SqlBuilder = SqlBuilder::new();
        t___8456.append_safe("select p.id from person p ");
        t___8456.append_fragment(condition__1450.clone());
        let actual___1881: std::sync::Arc<String> = t___8456.accumulated().to_string();
        let mut t___8462: bool = Some(actual___1881.as_str()) == Some("select p.id from person p where p.last_name = 'Someone'");
        #[derive(Clone)]
        struct ClosureGroup___177 {
            actual___1881: std::sync::Arc<String>
        }
        impl ClosureGroup___177 {
            fn fn__8451(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"select p.id from person p \", \\interpolate, condition).toString() == (select p.id from person p where p.last_name = 'Someone') not ({})", self.actual___1881.clone()));
            }
        }
        let closure_group = ClosureGroup___177 {
            actual___1881: actual___1881.clone()
        };
        let fn__8451 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__8451())
        };
        test___160.assert(t___8462, fn__8451.clone());
        let mut t___8464: SqlBuilder = SqlBuilder::new();
        t___8464.append_safe("select p.id from person p ");
        t___8464.append_part(SqlPart::new(condition__1450.to_source()));
        let actual___1884: std::sync::Arc<String> = t___8464.accumulated().to_string();
        let mut t___8471: bool = Some(actual___1884.as_str()) == Some("select p.id from person p where p.last_name = 'Someone'");
        #[derive(Clone)]
        struct ClosureGroup___178 {
            actual___1884: std::sync::Arc<String>
        }
        impl ClosureGroup___178 {
            fn fn__8450(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"select p.id from person p \", \\interpolate, condition.toSource()).toString() == (select p.id from person p where p.last_name = 'Someone') not ({})", self.actual___1884.clone()));
            }
        }
        let closure_group = ClosureGroup___178 {
            actual___1884: actual___1884.clone()
        };
        let fn__8450 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__8450())
        };
        test___160.assert(t___8471, fn__8450.clone());
        let parts__1451: temper_core::List<SqlPart> = std::sync::Arc::new(vec![SqlPart::new(SqlString::new("a'b")), SqlPart::new(SqlInt32::new(3))]);
        let mut t___8475: SqlBuilder = SqlBuilder::new();
        t___8475.append_safe("select ");
        t___8475.append_part_list(parts__1451.clone());
        let actual___1887: std::sync::Arc<String> = t___8475.accumulated().to_string();
        let mut t___8481: bool = Some(actual___1887.as_str()) == Some("select 'a''b', 3");
        #[derive(Clone)]
        struct ClosureGroup___179 {
            actual___1887: std::sync::Arc<String>
        }
        impl ClosureGroup___179 {
            fn fn__8449(& self) -> std::sync::Arc<String> {
                return std::sync::Arc::new(format!("expected stringExpr(`-work//src/`.sql, true, \"select \", \\interpolate, parts).toString() == (select 'a''b', 3) not ({})", self.actual___1887.clone()));
            }
        }
        let closure_group = ClosureGroup___179 {
            actual___1887: actual___1887.clone()
        };
        let fn__8449 = {
            let closure_group = closure_group.clone();
            std::sync::Arc::new(move | | closure_group.fn__8449())
        };
        test___160.assert(t___8481, fn__8449.clone());
        test___160.soft_fail_to_hard()
    }
    use super::*;
}
