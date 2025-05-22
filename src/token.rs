const AMPERSAND: char = '&';
const ASTERISK: char = '*';
const BACKTICK: char = '`';
const BRACKET_CLOSE: char = ']';
const BRACKET_OPEN: char = '[';
const CIRCUMFLEX: char = '^';
const COMMA: char = ',';
const CURLY_BRACKET_CLOSE: char = '}';
const CURLY_BRACKET_OPEN: char = '{';
const DELIMITER: char = ';';
const EQUAL: char = '=';
const FULL_STOP: char = '.';
const GREATER_THAN: char = '>';
const HYPHEN: char = '-';
const LESS_THAN: char = '<';
const NEW_LINE: char = '\n';
const PAREN_CLOSE: char = ')';
const PAREN_OPEN: char = '(';
const PERCENT: char = '%';
const PLUS: char = '+';
const QUOTE_DOUBLE: char = '"';
const QUOTE_SINGLE: char = '\'';
const SLASH_FORWARD: char = '/';
const VERTICAL_BAR: char = '|';

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub value: String,
    pub category: Option<TokenCategory>,
    pub behavior: Vec<TokenBehavior>,
}

impl Token {
    fn new() -> Token {
        Token {
            value: String::new(),
            category: None,
            behavior: vec![],
        }
    }

    pub fn new_space(space: String) -> Token {
        Token {
            value: space,
            category: Some(TokenCategory::Space),
            behavior: vec![],
        }
    }

    pub fn newline() -> Token {
        Token {
            value: "\n".to_string(),
            category: Some(TokenCategory::NewLine),
            behavior: vec![],
        }
    }

    fn len(&self) -> usize {
        self.value.len()
    }

    fn is_empty(&self) -> bool {
        self.value.is_empty()
    }

    fn count(&self, find: char) -> usize {
        self.value.match_indices(find).count()
    }

    fn setup(&mut self) {
        self.set_category();
        self.set_behavior();
    }

    fn set_category(&mut self) {
        if self.category.is_some() {
            return;
        }

        match self.value.to_uppercase().as_str() {
            // Keywords
            "ABORT" => self.category = Some(TokenCategory::Keyword),
            "ABORTSESSION" => self.category = Some(TokenCategory::Keyword),
            "ABSENT" => self.category = Some(TokenCategory::Keyword),
            "ABSOLUTE" => self.category = Some(TokenCategory::Keyword),
            "ACCESS" => self.category = Some(TokenCategory::Keyword),
            "ACCESSIBLE" => self.category = Some(TokenCategory::Keyword),
            "ACCESS_LOCK" => self.category = Some(TokenCategory::Keyword),
            "ACCOUNT" => self.category = Some(TokenCategory::Keyword),
            "ACOSH" => self.category = Some(TokenCategory::Keyword),
            "ACTION" => self.category = Some(TokenCategory::Keyword),
            "ADA" => self.category = Some(TokenCategory::Keyword),
            "ADD" => self.category = Some(TokenCategory::Keyword),
            "ADD_MONTHS" => self.category = Some(TokenCategory::Keyword),
            "ADMIN" => self.category = Some(TokenCategory::Keyword),
            "AFTER" => self.category = Some(TokenCategory::Keyword),
            "AGGREGATE" => self.category = Some(TokenCategory::Keyword),
            "ALIAS" => self.category = Some(TokenCategory::Keyword),
            "ALL" => self.category = Some(TokenCategory::Keyword),
            "ALLOCATE" => self.category = Some(TokenCategory::Keyword),
            "ALLOW" => self.category = Some(TokenCategory::Keyword),
            "ALTER" => self.category = Some(TokenCategory::Keyword),
            "ALTERAND" => self.category = Some(TokenCategory::Keyword),
            "AMP" => self.category = Some(TokenCategory::Keyword),
            "ANALYSE" => self.category = Some(TokenCategory::Keyword),
            "ANALYZE" => self.category = Some(TokenCategory::Keyword),
            "AND" => self.category = Some(TokenCategory::Keyword),
            "ANSIDATE" => self.category = Some(TokenCategory::Keyword),
            "ANY" => self.category = Some(TokenCategory::Keyword),
            "ANY_VALUE" => self.category = Some(TokenCategory::Keyword),
            "ARE" => self.category = Some(TokenCategory::Keyword),
            "ARRAY" => self.category = Some(TokenCategory::Keyword),
            "ARRAY_AGG" => self.category = Some(TokenCategory::Keyword),
            "ARRAY_EXISTS" => self.category = Some(TokenCategory::Keyword),
            "ARRAY_MAX_CARDINALITY" => self.category = Some(TokenCategory::Keyword),
            "AS" => self.category = Some(TokenCategory::Keyword),
            "ASC" => self.category = Some(TokenCategory::Keyword),
            "ASCII" => self.category = Some(TokenCategory::Keyword),
            "ASENSITIVE" => self.category = Some(TokenCategory::Keyword),
            "ASINH" => self.category = Some(TokenCategory::Keyword),
            "ASSERTION" => self.category = Some(TokenCategory::Keyword),
            "ASSOCIATE" => self.category = Some(TokenCategory::Keyword),
            "ASUTIME" => self.category = Some(TokenCategory::Keyword),
            "ASYMMETRIC" => self.category = Some(TokenCategory::Keyword),
            "AT" => self.category = Some(TokenCategory::Keyword),
            "ATANH" => self.category = Some(TokenCategory::Keyword),
            "ATN2" => self.category = Some(TokenCategory::Keyword),
            "ATOMIC" => self.category = Some(TokenCategory::Keyword),
            "AUDIT" => self.category = Some(TokenCategory::Keyword),
            "AUTHORIZATION" => self.category = Some(TokenCategory::Keyword),
            "AUX" => self.category = Some(TokenCategory::Keyword),
            "AUXILIARY" => self.category = Some(TokenCategory::Keyword),
            "AVE" => self.category = Some(TokenCategory::Keyword),
            "AVERAGE" => self.category = Some(TokenCategory::Keyword),
            "BACKUP" => self.category = Some(TokenCategory::Keyword),
            "BEFORE" => self.category = Some(TokenCategory::Keyword),
            "BEGIN" => self.category = Some(TokenCategory::Keyword),
            "BEGIN_FRAME" => self.category = Some(TokenCategory::Keyword),
            "BEGIN_PARTITION" => self.category = Some(TokenCategory::Keyword),
            "BETWEEN" => self.category = Some(TokenCategory::Keyword),
            "BIT_LENGTH" => self.category = Some(TokenCategory::Keyword),
            "BOTH" => self.category = Some(TokenCategory::Keyword),
            "BREADTH" => self.category = Some(TokenCategory::Keyword),
            "BREAK" => self.category = Some(TokenCategory::Keyword),
            "BROWSE" => self.category = Some(TokenCategory::Keyword),
            "BT" => self.category = Some(TokenCategory::Keyword),
            "BTRIM" => self.category = Some(TokenCategory::Keyword),
            "BUFFERPOOL" => self.category = Some(TokenCategory::Keyword),
            "BULK" => self.category = Some(TokenCategory::Keyword),
            "BUT" => self.category = Some(TokenCategory::Keyword),
            "BY" => self.category = Some(TokenCategory::Keyword),
            "BYTE" => self.category = Some(TokenCategory::Keyword),
            "BYTEINT" => self.category = Some(TokenCategory::Keyword),
            "BYTES" => self.category = Some(TokenCategory::Keyword),
            "CALLED" => self.category = Some(TokenCategory::Keyword),
            "CAPTURE" => self.category = Some(TokenCategory::Keyword),
            "CARDINALITY" => self.category = Some(TokenCategory::Keyword),
            "CASCADE" => self.category = Some(TokenCategory::Keyword),
            "CASCADED" => self.category = Some(TokenCategory::Keyword),
            "CASE" => self.category = Some(TokenCategory::Keyword),
            "CASESPECIFIC" => self.category = Some(TokenCategory::Keyword),
            "CASE_N" => self.category = Some(TokenCategory::Keyword),
            "CATALOG" => self.category = Some(TokenCategory::Keyword),
            "CCSID" => self.category = Some(TokenCategory::Keyword),
            "CD" => self.category = Some(TokenCategory::Keyword),
            "CHANGE" => self.category = Some(TokenCategory::Keyword),
            "CHAR2HEXINT" => self.category = Some(TokenCategory::Keyword),
            "CHARACTER" => self.category = Some(TokenCategory::Keyword),
            "CHARACTERS" => self.category = Some(TokenCategory::Keyword),
            "CHARINDEX" => self.category = Some(TokenCategory::Keyword),
            "CHARS" => self.category = Some(TokenCategory::Keyword),
            "CHECK" => self.category = Some(TokenCategory::Keyword),
            "CHECKPOINT" => self.category = Some(TokenCategory::Keyword),
            "CLASS" => self.category = Some(TokenCategory::Keyword),
            "CLASSIFIER" => self.category = Some(TokenCategory::Keyword),
            "CLOB" => self.category = Some(TokenCategory::Keyword),
            "CLONE" => self.category = Some(TokenCategory::Keyword),
            "CLOSE" => self.category = Some(TokenCategory::Keyword),
            "CLUSTER" => self.category = Some(TokenCategory::Keyword),
            "CLUSTERED" => self.category = Some(TokenCategory::Keyword),
            "CM" => self.category = Some(TokenCategory::Keyword),
            "COLLATE" => self.category = Some(TokenCategory::Keyword),
            "COLLATION" => self.category = Some(TokenCategory::Keyword),
            "COLLECT" => self.category = Some(TokenCategory::Keyword),
            "COLLECTION" => self.category = Some(TokenCategory::Keyword),
            "COLLID" => self.category = Some(TokenCategory::Keyword),
            "COLUMN" => self.category = Some(TokenCategory::Keyword),
            "COLUMN_VALUE" => self.category = Some(TokenCategory::Keyword),
            "COMMENT" => self.category = Some(TokenCategory::Keyword),
            "COMMIT" => self.category = Some(TokenCategory::Keyword),
            "COMPLETION" => self.category = Some(TokenCategory::Keyword),
            "COMPRESS" => self.category = Some(TokenCategory::Keyword),
            "COMPUTE" => self.category = Some(TokenCategory::Keyword),
            "CONCURRENTLY" => self.category = Some(TokenCategory::Keyword),
            "CONDITION" => self.category = Some(TokenCategory::Keyword),
            "CONNECT" => self.category = Some(TokenCategory::Keyword),
            "CONNECTION" => self.category = Some(TokenCategory::Keyword),
            "CONSTRAINT" => self.category = Some(TokenCategory::Keyword),
            "CONSTRAINTS" => self.category = Some(TokenCategory::Keyword),
            "CONSTRUCTOR" => self.category = Some(TokenCategory::Keyword),
            "CONTAINS" => self.category = Some(TokenCategory::Keyword),
            "CONTAINSTABLE" => self.category = Some(TokenCategory::Keyword),
            "CONTENT" => self.category = Some(TokenCategory::Keyword),
            "CONTINUE" => self.category = Some(TokenCategory::Keyword),
            "CONVERT_TABLE_HEADER" => self.category = Some(TokenCategory::Keyword),
            "COPY" => self.category = Some(TokenCategory::Keyword),
            "CORR" => self.category = Some(TokenCategory::Keyword),
            "CORRESPONDING" => self.category = Some(TokenCategory::Keyword),
            "COSH" => self.category = Some(TokenCategory::Keyword),
            "COVAR_POP" => self.category = Some(TokenCategory::Keyword),
            "COVAR_SAMP" => self.category = Some(TokenCategory::Keyword),
            "CREATE" => self.category = Some(TokenCategory::Keyword),
            "CROSS" => self.category = Some(TokenCategory::Keyword),
            "CS" => self.category = Some(TokenCategory::Keyword),
            "CSUM" => self.category = Some(TokenCategory::Keyword),
            "CT" => self.category = Some(TokenCategory::Keyword),
            "CUBE" => self.category = Some(TokenCategory::Keyword),
            "CUME_DIST" => self.category = Some(TokenCategory::Keyword),
            "CURRENT" => self.category = Some(TokenCategory::Keyword),
            "CURRENT_CATALOG" => self.category = Some(TokenCategory::Keyword),
            "CURRENT_DEFAULT_TRANSFORM_GROUP" => self.category = Some(TokenCategory::Keyword),
            "CURRENT_LC_CTYPE" => self.category = Some(TokenCategory::Keyword),
            "CURRENT_PATH" => self.category = Some(TokenCategory::Keyword),
            "CURRENT_ROLE" => self.category = Some(TokenCategory::Keyword),
            "CURRENT_ROW" => self.category = Some(TokenCategory::Keyword),
            "CURRENT_SCHEMA" => self.category = Some(TokenCategory::Keyword),
            "CURRENT_SERVER" => self.category = Some(TokenCategory::Keyword),
            "CURRENT_TIMEZONE" => self.category = Some(TokenCategory::Keyword),
            "CURRENT_TRANSFORM_GROUP_FOR_TYPE" => self.category = Some(TokenCategory::Keyword),
            "CURRVAL" => self.category = Some(TokenCategory::Keyword),
            "CURSOR" => self.category = Some(TokenCategory::Keyword),
            "CV" => self.category = Some(TokenCategory::Keyword),
            "CYCLE" => self.category = Some(TokenCategory::Keyword),
            "DATA" => self.category = Some(TokenCategory::Keyword),
            "DATABASE" => self.category = Some(TokenCategory::Keyword),
            "DATABASES" => self.category = Some(TokenCategory::Keyword),
            "DATABLOCKSIZE" => self.category = Some(TokenCategory::Keyword),
            "DATALENGTH" => self.category = Some(TokenCategory::Keyword),
            "DATEADD" => self.category = Some(TokenCategory::Keyword),
            "DATEFORM" => self.category = Some(TokenCategory::Keyword),
            "DATEFROMPARTS" => self.category = Some(TokenCategory::Keyword),
            "DATENAME" => self.category = Some(TokenCategory::Keyword),
            "DATEPART" => self.category = Some(TokenCategory::Keyword),
            "DAYS" => self.category = Some(TokenCategory::Keyword),
            "DAY_HOUR" => self.category = Some(TokenCategory::Keyword),
            "DAY_MICROSECOND" => self.category = Some(TokenCategory::Keyword),
            "DAY_MINUTE" => self.category = Some(TokenCategory::Keyword),
            "DAY_SECOND" => self.category = Some(TokenCategory::Keyword),
            "DBCC" => self.category = Some(TokenCategory::Keyword),
            "DBINFO" => self.category = Some(TokenCategory::Keyword),
            "DEALLOCATE" => self.category = Some(TokenCategory::Keyword),
            "DECFLOAT" => self.category = Some(TokenCategory::Keyword),
            "DECLARE" => self.category = Some(TokenCategory::Keyword),
            "DEFAULT" => self.category = Some(TokenCategory::Keyword),
            "DEFERRABLE" => self.category = Some(TokenCategory::Keyword),
            "DEFERRED" => self.category = Some(TokenCategory::Keyword),
            "DEFINE" => self.category = Some(TokenCategory::Keyword),
            "DEL" => self.category = Some(TokenCategory::Keyword),
            "DELAYED" => self.category = Some(TokenCategory::Keyword),
            "DELETE" => self.category = Some(TokenCategory::Keyword),
            "DENSE_RANK" => self.category = Some(TokenCategory::Keyword),
            "DENY" => self.category = Some(TokenCategory::Keyword),
            "DEPTH" => self.category = Some(TokenCategory::Keyword),
            "DEREF" => self.category = Some(TokenCategory::Keyword),
            "DESC" => self.category = Some(TokenCategory::Keyword),
            "DESCRIBE" => self.category = Some(TokenCategory::Keyword),
            "DESCRIPTOR" => self.category = Some(TokenCategory::Keyword),
            "DESTROY" => self.category = Some(TokenCategory::Keyword),
            "DESTRUCTOR" => self.category = Some(TokenCategory::Keyword),
            "DETERMINISTIC" => self.category = Some(TokenCategory::Keyword),
            "DIAGNOSTIC" => self.category = Some(TokenCategory::Keyword),
            "DIAGNOSTICS" => self.category = Some(TokenCategory::Keyword),
            "DICTIONARY" => self.category = Some(TokenCategory::Keyword),
            "DIFFERENCE" => self.category = Some(TokenCategory::Keyword),
            "DISABLE" => self.category = Some(TokenCategory::Keyword),
            "DISABLED" => self.category = Some(TokenCategory::Keyword),
            "DISALLOW" => self.category = Some(TokenCategory::Keyword),
            "DISCONNECT" => self.category = Some(TokenCategory::Keyword),
            "DISK" => self.category = Some(TokenCategory::Keyword),
            "DISTINCT" => self.category = Some(TokenCategory::Keyword),
            "DISTINCTROW" => self.category = Some(TokenCategory::Keyword),
            "DISTRIBUTED" => self.category = Some(TokenCategory::Keyword),
            "DO" => self.category = Some(TokenCategory::Keyword),
            "DOCUMENT" => self.category = Some(TokenCategory::Keyword),
            "DOMAIN" => self.category = Some(TokenCategory::Keyword),
            "DROP" => self.category = Some(TokenCategory::Keyword),
            "DSSIZE" => self.category = Some(TokenCategory::Keyword),
            "DUAL" => self.category = Some(TokenCategory::Keyword),
            "DUMP" => self.category = Some(TokenCategory::Keyword),
            "DYNAMIC" => self.category = Some(TokenCategory::Keyword),
            "EACH" => self.category = Some(TokenCategory::Keyword),
            "ECHO" => self.category = Some(TokenCategory::Keyword),
            "EDITPROC" => self.category = Some(TokenCategory::Keyword),
            "ELEMENT" => self.category = Some(TokenCategory::Keyword),
            "ELSE" => self.category = Some(TokenCategory::Keyword),
            "ELSEIF" => self.category = Some(TokenCategory::Keyword),
            "EMPTY" => self.category = Some(TokenCategory::Keyword),
            "ENABLED" => self.category = Some(TokenCategory::Keyword),
            "ENCLOSED" => self.category = Some(TokenCategory::Keyword),
            "ENCODING" => self.category = Some(TokenCategory::Keyword),
            "ENCRYPTION" => self.category = Some(TokenCategory::Keyword),
            "END" => self.category = Some(TokenCategory::Keyword),
            "END-EXEC" => self.category = Some(TokenCategory::Keyword),
            "ENDING" => self.category = Some(TokenCategory::Keyword),
            "END_FRAME" => self.category = Some(TokenCategory::Keyword),
            "END_PARTITION" => self.category = Some(TokenCategory::Keyword),
            "EQ" => self.category = Some(TokenCategory::Keyword),
            "EQUALS" => self.category = Some(TokenCategory::Keyword),
            "ERASE" => self.category = Some(TokenCategory::Keyword),
            "ERRLVL" => self.category = Some(TokenCategory::Keyword),
            "ERROR" => self.category = Some(TokenCategory::Keyword),
            "ERRORFILES" => self.category = Some(TokenCategory::Keyword),
            "ERRORTABLES" => self.category = Some(TokenCategory::Keyword),
            "ESCAPE" => self.category = Some(TokenCategory::Keyword),
            "ESCAPED" => self.category = Some(TokenCategory::Keyword),
            "ET" => self.category = Some(TokenCategory::Keyword),
            "EVERY" => self.category = Some(TokenCategory::Keyword),
            "EXCEPT" => self.category = Some(TokenCategory::Keyword),
            "EXCEPTION" => self.category = Some(TokenCategory::Keyword),
            "EXCLUSIVE" => self.category = Some(TokenCategory::Keyword),
            "EXISTS" => self.category = Some(TokenCategory::Keyword),
            "EXIT" => self.category = Some(TokenCategory::Keyword),
            "EXPLAIN" => self.category = Some(TokenCategory::Keyword),
            "EXTERNAL" => self.category = Some(TokenCategory::Keyword),
            "FALLBACK" => self.category = Some(TokenCategory::Keyword),
            "FALSE" => self.category = Some(TokenCategory::Keyword),
            "FASTEXPORT" => self.category = Some(TokenCategory::Keyword),
            "FENCED" => self.category = Some(TokenCategory::Keyword),
            "FETCH" => self.category = Some(TokenCategory::Keyword),
            "FIELD" => self.category = Some(TokenCategory::Keyword),
            "FIELDPROC" => self.category = Some(TokenCategory::Keyword),
            "FILE" => self.category = Some(TokenCategory::Keyword),
            "FILLFACTOR" => self.category = Some(TokenCategory::Keyword),
            "FILTER" => self.category = Some(TokenCategory::Keyword),
            "FINAL" => self.category = Some(TokenCategory::Keyword),
            "FLOAT4" => self.category = Some(TokenCategory::Keyword),
            "FLOAT8" => self.category = Some(TokenCategory::Keyword),
            "FOR" => self.category = Some(TokenCategory::Keyword),
            "FORCE" => self.category = Some(TokenCategory::Keyword),
            "FOREIGN" => self.category = Some(TokenCategory::Keyword),
            "FORMAT" => self.category = Some(TokenCategory::Keyword),
            "FORTRAN" => self.category = Some(TokenCategory::Keyword),
            "FOUND" => self.category = Some(TokenCategory::Keyword),
            "FRAME_ROW" => self.category = Some(TokenCategory::Keyword),
            "FREE" => self.category = Some(TokenCategory::Keyword),
            "FREESPACE" => self.category = Some(TokenCategory::Keyword),
            "FREETEXT" => self.category = Some(TokenCategory::Keyword),
            "FREETEXTTABLE" => self.category = Some(TokenCategory::Keyword),
            "FREEZE" => self.category = Some(TokenCategory::Keyword),
            "FROM" => self.category = Some(TokenCategory::Keyword),
            "FULL" => self.category = Some(TokenCategory::Keyword),
            "FULLTEXT" => self.category = Some(TokenCategory::Keyword),
            "FUNCTION" => self.category = Some(TokenCategory::Keyword),
            "FUSION" => self.category = Some(TokenCategory::Keyword),
            "GE" => self.category = Some(TokenCategory::Keyword),
            "GENERAL" => self.category = Some(TokenCategory::Keyword),
            "GENERATED" => self.category = Some(TokenCategory::Keyword),
            "GET" => self.category = Some(TokenCategory::Keyword),
            "GETUTCDATE" => self.category = Some(TokenCategory::Keyword),
            "GIVE" => self.category = Some(TokenCategory::Keyword),
            "GLOBAL" => self.category = Some(TokenCategory::Keyword),
            "GO" => self.category = Some(TokenCategory::Keyword),
            "GOTO" => self.category = Some(TokenCategory::Keyword),
            "GRANT" => self.category = Some(TokenCategory::Keyword),
            "GRAPHIC" => self.category = Some(TokenCategory::Keyword),
            "GROUP" => self.category = Some(TokenCategory::Keyword),
            "GROUPING" => self.category = Some(TokenCategory::Keyword),
            "GROUPS" => self.category = Some(TokenCategory::Keyword),
            "GT" => self.category = Some(TokenCategory::Keyword),
            "HANDLER" => self.category = Some(TokenCategory::Keyword),
            "HASH" => self.category = Some(TokenCategory::Keyword),
            "HASHAMP" => self.category = Some(TokenCategory::Keyword),
            "HASHBAKAMP" => self.category = Some(TokenCategory::Keyword),
            "HASHBUCKET" => self.category = Some(TokenCategory::Keyword),
            "HASHROW" => self.category = Some(TokenCategory::Keyword),
            "HAVING" => self.category = Some(TokenCategory::Keyword),
            "HELP" => self.category = Some(TokenCategory::Keyword),
            "HIGH_PRIORITY" => self.category = Some(TokenCategory::Keyword),
            "HOLD" => self.category = Some(TokenCategory::Keyword),
            "HOLDLOCK" => self.category = Some(TokenCategory::Keyword),
            "HOST" => self.category = Some(TokenCategory::Keyword),
            "HOURS" => self.category = Some(TokenCategory::Keyword),
            "HOUR_MICROSECOND" => self.category = Some(TokenCategory::Keyword),
            "HOUR_MINUTE" => self.category = Some(TokenCategory::Keyword),
            "HOUR_SECOND" => self.category = Some(TokenCategory::Keyword),
            "IDENTIFIED" => self.category = Some(TokenCategory::Keyword),
            "IDENTITY" => self.category = Some(TokenCategory::Keyword),
            "IDENTITYCOL" => self.category = Some(TokenCategory::Keyword),
            "IDENTITY_INSERT" => self.category = Some(TokenCategory::Keyword),
            "IGNORE" => self.category = Some(TokenCategory::Keyword),
            "ILIKE" => self.category = Some(TokenCategory::Keyword),
            "IMMEDIATE" => self.category = Some(TokenCategory::Keyword),
            "IN" => self.category = Some(TokenCategory::Keyword),
            "INCLUDE" => self.category = Some(TokenCategory::Keyword),
            "INCLUSIVE" => self.category = Some(TokenCategory::Keyword),
            "INCONSISTENT" => self.category = Some(TokenCategory::Keyword),
            "INCREMENT" => self.category = Some(TokenCategory::Keyword),
            "INDEX" => self.category = Some(TokenCategory::Keyword),
            "INDICATOR" => self.category = Some(TokenCategory::Keyword),
            "INFILE" => self.category = Some(TokenCategory::Keyword),
            "INHERIT" => self.category = Some(TokenCategory::Keyword),
            "INITIAL" => self.category = Some(TokenCategory::Keyword),
            "INITIALIZE" => self.category = Some(TokenCategory::Keyword),
            "INITIALLY" => self.category = Some(TokenCategory::Keyword),
            "INITIATE" => self.category = Some(TokenCategory::Keyword),
            "INNER" => self.category = Some(TokenCategory::Keyword),
            "INOUT" => self.category = Some(TokenCategory::Keyword),
            "INPUT" => self.category = Some(TokenCategory::Keyword),
            "INS" => self.category = Some(TokenCategory::Keyword),
            "INSENSITIVE" => self.category = Some(TokenCategory::Keyword),
            "INSERT" => self.category = Some(TokenCategory::Keyword),
            "INSTEAD" => self.category = Some(TokenCategory::Keyword),
            "INT1" => self.category = Some(TokenCategory::Keyword),
            "INT2" => self.category = Some(TokenCategory::Keyword),
            "INT3" => self.category = Some(TokenCategory::Keyword),
            "INT4" => self.category = Some(TokenCategory::Keyword),
            "INT8" => self.category = Some(TokenCategory::Keyword),
            "INTEGERDATE" => self.category = Some(TokenCategory::Keyword),
            "INTERSECT" => self.category = Some(TokenCategory::Keyword),
            "INTERSECTION" => self.category = Some(TokenCategory::Keyword),
            "INTERVAL" => self.category = Some(TokenCategory::Keyword),
            "INTO" => self.category = Some(TokenCategory::Keyword),
            "IO_AFTER_GTIDS" => self.category = Some(TokenCategory::Keyword),
            "IO_BEFORE_GTIDS" => self.category = Some(TokenCategory::Keyword),
            "IS" => self.category = Some(TokenCategory::Keyword),
            "ISDATE" => self.category = Some(TokenCategory::Keyword),
            "ISNUMERIC" => self.category = Some(TokenCategory::Keyword),
            "ISOBID" => self.category = Some(TokenCategory::Keyword),
            "ISOLATION" => self.category = Some(TokenCategory::Keyword),
            "ITERATE" => self.category = Some(TokenCategory::Keyword),
            "JAR" => self.category = Some(TokenCategory::Keyword),
            "JOIN" => self.category = Some(TokenCategory::Keyword),
            "JOURNAL" => self.category = Some(TokenCategory::Keyword),
            "JSON" => self.category = Some(TokenCategory::Keyword),
            "JSON_ARRAY" => self.category = Some(TokenCategory::Keyword),
            "JSON_ARRAYAGG" => self.category = Some(TokenCategory::Keyword),
            "JSON_EXISTS" => self.category = Some(TokenCategory::Keyword),
            "JSON_OBJECT" => self.category = Some(TokenCategory::Keyword),
            "JSON_OBJECTAGG" => self.category = Some(TokenCategory::Keyword),
            "JSON_QUERY" => self.category = Some(TokenCategory::Keyword),
            "JSON_SCALAR" => self.category = Some(TokenCategory::Keyword),
            "JSON_SERIALIZE" => self.category = Some(TokenCategory::Keyword),
            "JSON_TABLE" => self.category = Some(TokenCategory::Keyword),
            "JSON_TABLE_PRIMITIVE" => self.category = Some(TokenCategory::Keyword),
            "JSON_VALUE" => self.category = Some(TokenCategory::Keyword),
            "KEEP" => self.category = Some(TokenCategory::Keyword),
            "KEYS" => self.category = Some(TokenCategory::Keyword),
            "KILL" => self.category = Some(TokenCategory::Keyword),
            "KURTOSIS" => self.category = Some(TokenCategory::Keyword),
            "LABEL" => self.category = Some(TokenCategory::Keyword),
            "LANGUAGE" => self.category = Some(TokenCategory::Keyword),
            "LARGE" => self.category = Some(TokenCategory::Keyword),
            "LATERAL" => self.category = Some(TokenCategory::Keyword),
            "LC_CTYPE" => self.category = Some(TokenCategory::Keyword),
            "LE" => self.category = Some(TokenCategory::Keyword),
            "LEADING" => self.category = Some(TokenCategory::Keyword),
            "LEAVE" => self.category = Some(TokenCategory::Keyword),
            "LEN" => self.category = Some(TokenCategory::Keyword),
            "LESS" => self.category = Some(TokenCategory::Keyword),
            "LEVEL" => self.category = Some(TokenCategory::Keyword),
            "LIKE" => self.category = Some(TokenCategory::Keyword),
            "LIKE_REGEX" => self.category = Some(TokenCategory::Keyword),
            "LIMIT" => self.category = Some(TokenCategory::Keyword),
            "LINEAR" => self.category = Some(TokenCategory::Keyword),
            "LINENO" => self.category = Some(TokenCategory::Keyword),
            "LINES" => self.category = Some(TokenCategory::Keyword),
            "LISTAGG" => self.category = Some(TokenCategory::Keyword),
            "LOAD" => self.category = Some(TokenCategory::Keyword),
            "LOADING" => self.category = Some(TokenCategory::Keyword),
            "LOCAL" => self.category = Some(TokenCategory::Keyword),
            "LOCALE" => self.category = Some(TokenCategory::Keyword),
            "LOCATOR" => self.category = Some(TokenCategory::Keyword),
            "LOCATORS" => self.category = Some(TokenCategory::Keyword),
            "LOCK" => self.category = Some(TokenCategory::Keyword),
            "LOCKING" => self.category = Some(TokenCategory::Keyword),
            "LOCKMAX" => self.category = Some(TokenCategory::Keyword),
            "LOCKSIZE" => self.category = Some(TokenCategory::Keyword),
            "LOGGING" => self.category = Some(TokenCategory::Keyword),
            "LOGON" => self.category = Some(TokenCategory::Keyword),
            "LONG" => self.category = Some(TokenCategory::Keyword),
            "LOOP" => self.category = Some(TokenCategory::Keyword),
            "LOW_PRIORITY" => self.category = Some(TokenCategory::Keyword),
            "LT" => self.category = Some(TokenCategory::Keyword),
            "MACRO" => self.category = Some(TokenCategory::Keyword),
            "MAINTAINED" => self.category = Some(TokenCategory::Keyword),
            "MAP" => self.category = Some(TokenCategory::Keyword),
            "MASTER_BIND" => self.category = Some(TokenCategory::Keyword),
            "MASTER_SSL_VERIFY_SERVER_CERT" => self.category = Some(TokenCategory::Keyword),
            "MATCH" => self.category = Some(TokenCategory::Keyword),
            "MATCHES" => self.category = Some(TokenCategory::Keyword),
            "MATCH_NUMBER" => self.category = Some(TokenCategory::Keyword),
            "MATCH_RECOGNIZE" => self.category = Some(TokenCategory::Keyword),
            "MATERIALIZED" => self.category = Some(TokenCategory::Keyword),
            "MAVG" => self.category = Some(TokenCategory::Keyword),
            "MAXEXTENTS" => self.category = Some(TokenCategory::Keyword),
            "MAXIMUM" => self.category = Some(TokenCategory::Keyword),
            "MAXVALUE" => self.category = Some(TokenCategory::Keyword),
            "MCHARACTERS" => self.category = Some(TokenCategory::Keyword),
            "MDIFF" => self.category = Some(TokenCategory::Keyword),
            "MEMBER" => self.category = Some(TokenCategory::Keyword),
            "MERGE" => self.category = Some(TokenCategory::Keyword),
            "METHOD" => self.category = Some(TokenCategory::Keyword),
            "MICROSECONDS" => self.category = Some(TokenCategory::Keyword),
            "MIDDLEINT" => self.category = Some(TokenCategory::Keyword),
            "MINDEX" => self.category = Some(TokenCategory::Keyword),
            "MINIMUM" => self.category = Some(TokenCategory::Keyword),
            "MINUS" => self.category = Some(TokenCategory::Keyword),
            "MINUTES" => self.category = Some(TokenCategory::Keyword),
            "MINUTE_MICROSECOND" => self.category = Some(TokenCategory::Keyword),
            "MINUTE_SECOND" => self.category = Some(TokenCategory::Keyword),
            "MLINREG" => self.category = Some(TokenCategory::Keyword),
            "MLOAD" => self.category = Some(TokenCategory::Keyword),
            "MLSLABEL" => self.category = Some(TokenCategory::Keyword),
            "MODE" => self.category = Some(TokenCategory::Keyword),
            "MODIFIES" => self.category = Some(TokenCategory::Keyword),
            "MODIFY" => self.category = Some(TokenCategory::Keyword),
            "MODULE" => self.category = Some(TokenCategory::Keyword),
            "MONRESOURCE" => self.category = Some(TokenCategory::Keyword),
            "MONSESSION" => self.category = Some(TokenCategory::Keyword),
            "MONTHS" => self.category = Some(TokenCategory::Keyword),
            "MSUBSTR" => self.category = Some(TokenCategory::Keyword),
            "MSUM" => self.category = Some(TokenCategory::Keyword),
            "MULTISET" => self.category = Some(TokenCategory::Keyword),
            "NAMED" => self.category = Some(TokenCategory::Keyword),
            "NAMES" => self.category = Some(TokenCategory::Keyword),
            "NATIONAL" => self.category = Some(TokenCategory::Keyword),
            "NATURAL" => self.category = Some(TokenCategory::Keyword),
            "NCLOB" => self.category = Some(TokenCategory::Keyword),
            "NE" => self.category = Some(TokenCategory::Keyword),
            "NESTED_TABLE_ID" => self.category = Some(TokenCategory::Keyword),
            "NEW" => self.category = Some(TokenCategory::Keyword),
            "NEW_TABLE" => self.category = Some(TokenCategory::Keyword),
            "NEXT" => self.category = Some(TokenCategory::Keyword),
            "NEXTVAL" => self.category = Some(TokenCategory::Keyword),
            "NO" => self.category = Some(TokenCategory::Keyword),
            "NOAUDIT" => self.category = Some(TokenCategory::Keyword),
            "NOCHECK" => self.category = Some(TokenCategory::Keyword),
            "NOCOMPRESS" => self.category = Some(TokenCategory::Keyword),
            "NONCLUSTERED" => self.category = Some(TokenCategory::Keyword),
            "NONE" => self.category = Some(TokenCategory::Keyword),
            "NORMALIZE" => self.category = Some(TokenCategory::Keyword),
            "NOT" => self.category = Some(TokenCategory::Keyword),
            "NOTNULL" => self.category = Some(TokenCategory::Keyword),
            "NOWAIT" => self.category = Some(TokenCategory::Keyword),
            "NO_WRITE_TO_BINLOG" => self.category = Some(TokenCategory::Keyword),
            "NTH_VALUE" => self.category = Some(TokenCategory::Keyword),
            "NTILE" => self.category = Some(TokenCategory::Keyword),
            "NULL" => self.category = Some(TokenCategory::Keyword),
            "NULLIFZERO" => self.category = Some(TokenCategory::Keyword),
            "NULLS" => self.category = Some(TokenCategory::Keyword),
            "NUMBER" => self.category = Some(TokenCategory::Keyword),
            "NUMPARTS" => self.category = Some(TokenCategory::Keyword),
            "OBID" => self.category = Some(TokenCategory::Keyword),
            "OBJECT" => self.category = Some(TokenCategory::Keyword),
            "OBJECTS" => self.category = Some(TokenCategory::Keyword),
            "OCCURRENCES_REGEX" => self.category = Some(TokenCategory::Keyword),
            "OCTET_LENGTH" => self.category = Some(TokenCategory::Keyword),
            "OF" => self.category = Some(TokenCategory::Keyword),
            "OFF" => self.category = Some(TokenCategory::Keyword),
            "OFFLINE" => self.category = Some(TokenCategory::Keyword),
            "OFFSET" => self.category = Some(TokenCategory::Keyword),
            "OFFSETS" => self.category = Some(TokenCategory::Keyword),
            "OLD" => self.category = Some(TokenCategory::Keyword),
            "OLD_TABLE" => self.category = Some(TokenCategory::Keyword),
            "OMIT" => self.category = Some(TokenCategory::Keyword),
            "ON" => self.category = Some(TokenCategory::Keyword),
            "ONE" => self.category = Some(TokenCategory::Keyword),
            "ONLINE" => self.category = Some(TokenCategory::Keyword),
            "ONLY" => self.category = Some(TokenCategory::Keyword),
            "OPEN" => self.category = Some(TokenCategory::Keyword),
            "OPENDATASOURCE" => self.category = Some(TokenCategory::Keyword),
            "OPENQUERY" => self.category = Some(TokenCategory::Keyword),
            "OPENROWSET" => self.category = Some(TokenCategory::Keyword),
            "OPENXML" => self.category = Some(TokenCategory::Keyword),
            "OPERATION" => self.category = Some(TokenCategory::Keyword),
            "OPTIMIZATION" => self.category = Some(TokenCategory::Keyword),
            "OPTIMIZE" => self.category = Some(TokenCategory::Keyword),
            "OPTIMIZER_COSTS" => self.category = Some(TokenCategory::Keyword),
            "OPTION" => self.category = Some(TokenCategory::Keyword),
            "OPTIONALLY" => self.category = Some(TokenCategory::Keyword),
            "OR" => self.category = Some(TokenCategory::Keyword),
            "ORDER" => self.category = Some(TokenCategory::Keyword),
            "ORDINALITY" => self.category = Some(TokenCategory::Keyword),
            "ORGANIZATION" => self.category = Some(TokenCategory::Keyword),
            "OUT" => self.category = Some(TokenCategory::Keyword),
            "OUTER" => self.category = Some(TokenCategory::Keyword),
            "OUTFILE" => self.category = Some(TokenCategory::Keyword),
            "OUTPUT" => self.category = Some(TokenCategory::Keyword),
            "OVER" => self.category = Some(TokenCategory::Keyword),
            "OVERLAPS" => self.category = Some(TokenCategory::Keyword),
            "OVERLAY" => self.category = Some(TokenCategory::Keyword),
            "OVERRIDE" => self.category = Some(TokenCategory::Keyword),
            "PACKAGE" => self.category = Some(TokenCategory::Keyword),
            "PAD" => self.category = Some(TokenCategory::Keyword),
            "PADDED" => self.category = Some(TokenCategory::Keyword),
            "PARAMETER" => self.category = Some(TokenCategory::Keyword),
            "PARAMETERS" => self.category = Some(TokenCategory::Keyword),
            "PART" => self.category = Some(TokenCategory::Keyword),
            "PARTIAL" => self.category = Some(TokenCategory::Keyword),
            "PARTITION" => self.category = Some(TokenCategory::Keyword),
            "PARTITIONED" => self.category = Some(TokenCategory::Keyword),
            "PARTITIONING" => self.category = Some(TokenCategory::Keyword),
            "PASCAL" => self.category = Some(TokenCategory::Keyword),
            "PASSWORD" => self.category = Some(TokenCategory::Keyword),
            "PATH" => self.category = Some(TokenCategory::Keyword),
            "PATINDEX" => self.category = Some(TokenCategory::Keyword),
            "PATTERN" => self.category = Some(TokenCategory::Keyword),
            "PCTFREE" => self.category = Some(TokenCategory::Keyword),
            "PER" => self.category = Some(TokenCategory::Keyword),
            "PERCENT" => self.category = Some(TokenCategory::Keyword),
            "PERCENTILE_CONT" => self.category = Some(TokenCategory::Keyword),
            "PERCENTILE_DISC" => self.category = Some(TokenCategory::Keyword),
            "PERCENT_RANK" => self.category = Some(TokenCategory::Keyword),
            "PERIOD" => self.category = Some(TokenCategory::Keyword),
            "PERM" => self.category = Some(TokenCategory::Keyword),
            "PERMANENT" => self.category = Some(TokenCategory::Keyword),
            "PIECESIZE" => self.category = Some(TokenCategory::Keyword),
            "PIVOT" => self.category = Some(TokenCategory::Keyword),
            "PLACING" => self.category = Some(TokenCategory::Keyword),
            "PLAN" => self.category = Some(TokenCategory::Keyword),
            "PORTION" => self.category = Some(TokenCategory::Keyword),
            "POSITION_REGEX" => self.category = Some(TokenCategory::Keyword),
            "POSTFIX" => self.category = Some(TokenCategory::Keyword),
            "PRECEDES" => self.category = Some(TokenCategory::Keyword),
            "PRECISION" => self.category = Some(TokenCategory::Keyword),
            "PREFIX" => self.category = Some(TokenCategory::Keyword),
            "PREORDER" => self.category = Some(TokenCategory::Keyword),
            "PREPARE" => self.category = Some(TokenCategory::Keyword),
            "PRESERVE" => self.category = Some(TokenCategory::Keyword),
            "PREVVAL" => self.category = Some(TokenCategory::Keyword),
            "PRIMARY" => self.category = Some(TokenCategory::Keyword),
            "PRINT" => self.category = Some(TokenCategory::Keyword),
            "PRIOR" => self.category = Some(TokenCategory::Keyword),
            "PRIQTY" => self.category = Some(TokenCategory::Keyword),
            "PRIVATE" => self.category = Some(TokenCategory::Keyword),
            "PRIVILEGES" => self.category = Some(TokenCategory::Keyword),
            "PROC" => self.category = Some(TokenCategory::Keyword),
            "PROCEDURE" => self.category = Some(TokenCategory::Keyword),
            "PROFILE" => self.category = Some(TokenCategory::Keyword),
            "PROGRAM" => self.category = Some(TokenCategory::Keyword),
            "PROPORTIONAL" => self.category = Some(TokenCategory::Keyword),
            "PROTECTION" => self.category = Some(TokenCategory::Keyword),
            "PSID" => self.category = Some(TokenCategory::Keyword),
            "PTF" => self.category = Some(TokenCategory::Keyword),
            "PUBLIC" => self.category = Some(TokenCategory::Keyword),
            "PURGE" => self.category = Some(TokenCategory::Keyword),
            "QUALIFIED" => self.category = Some(TokenCategory::Keyword),
            "QUALIFY" => self.category = Some(TokenCategory::Keyword),
            "QUANTILE" => self.category = Some(TokenCategory::Keyword),
            "QUERY" => self.category = Some(TokenCategory::Keyword),
            "QUERYNO" => self.category = Some(TokenCategory::Keyword),
            "QUOTENAME" => self.category = Some(TokenCategory::Keyword),
            "RAISERROR" => self.category = Some(TokenCategory::Keyword),
            "RANDOM" => self.category = Some(TokenCategory::Keyword),
            "RANGE" => self.category = Some(TokenCategory::Keyword),
            "RANGE_N" => self.category = Some(TokenCategory::Keyword),
            "RANK" => self.category = Some(TokenCategory::Keyword),
            "RAW" => self.category = Some(TokenCategory::Keyword),
            "READ" => self.category = Some(TokenCategory::Keyword),
            "READS" => self.category = Some(TokenCategory::Keyword),
            "READTEXT" => self.category = Some(TokenCategory::Keyword),
            "READ_WRITE" => self.category = Some(TokenCategory::Keyword),
            "RECONFIGURE" => self.category = Some(TokenCategory::Keyword),
            "RECURSIVE" => self.category = Some(TokenCategory::Keyword),
            "REF" => self.category = Some(TokenCategory::Keyword),
            "REFERENCES" => self.category = Some(TokenCategory::Keyword),
            "REFERENCING" => self.category = Some(TokenCategory::Keyword),
            "REFRESH" => self.category = Some(TokenCategory::Keyword),
            "REGEXP" => self.category = Some(TokenCategory::Keyword),
            "REGR_AVGX" => self.category = Some(TokenCategory::Keyword),
            "REGR_AVGY" => self.category = Some(TokenCategory::Keyword),
            "REGR_COUNT" => self.category = Some(TokenCategory::Keyword),
            "REGR_INTERCEPT" => self.category = Some(TokenCategory::Keyword),
            "REGR_R2" => self.category = Some(TokenCategory::Keyword),
            "REGR_SLOPE" => self.category = Some(TokenCategory::Keyword),
            "REGR_SXX" => self.category = Some(TokenCategory::Keyword),
            "REGR_SXY" => self.category = Some(TokenCategory::Keyword),
            "REGR_SYY" => self.category = Some(TokenCategory::Keyword),
            "RELATIVE" => self.category = Some(TokenCategory::Keyword),
            "RELEASE" => self.category = Some(TokenCategory::Keyword),
            "RENAME" => self.category = Some(TokenCategory::Keyword),
            "REPLICATE" => self.category = Some(TokenCategory::Keyword),
            "REPLICATION" => self.category = Some(TokenCategory::Keyword),
            "REPOVERRIDE" => self.category = Some(TokenCategory::Keyword),
            "REQUEST" => self.category = Some(TokenCategory::Keyword),
            "REQUIRE" => self.category = Some(TokenCategory::Keyword),
            "RESIGNAL" => self.category = Some(TokenCategory::Keyword),
            "RESOURCE" => self.category = Some(TokenCategory::Keyword),
            "RESTART" => self.category = Some(TokenCategory::Keyword),
            "RESTORE" => self.category = Some(TokenCategory::Keyword),
            "RESTRICT" => self.category = Some(TokenCategory::Keyword),
            "RESULT" => self.category = Some(TokenCategory::Keyword),
            "RESULT_SET_LOCATOR" => self.category = Some(TokenCategory::Keyword),
            "RESUME" => self.category = Some(TokenCategory::Keyword),
            "RET" => self.category = Some(TokenCategory::Keyword),
            "RETRIEVE" => self.category = Some(TokenCategory::Keyword),
            "RETURN" => self.category = Some(TokenCategory::Keyword),
            "RETURNING" => self.category = Some(TokenCategory::Keyword),
            "RETURNS" => self.category = Some(TokenCategory::Keyword),
            "REVALIDATE" => self.category = Some(TokenCategory::Keyword),
            "REVERT" => self.category = Some(TokenCategory::Keyword),
            "REVOKE" => self.category = Some(TokenCategory::Keyword),
            "RIGHTS" => self.category = Some(TokenCategory::Keyword),
            "RLIKE" => self.category = Some(TokenCategory::Keyword),
            "ROLE" => self.category = Some(TokenCategory::Keyword),
            "ROLLBACK" => self.category = Some(TokenCategory::Keyword),
            "ROLLFORWARD" => self.category = Some(TokenCategory::Keyword),
            "ROLLUP" => self.category = Some(TokenCategory::Keyword),
            "ROUND_CEILING" => self.category = Some(TokenCategory::Keyword),
            "ROUND_DOWN" => self.category = Some(TokenCategory::Keyword),
            "ROUND_FLOOR" => self.category = Some(TokenCategory::Keyword),
            "ROUND_HALF_DOWN" => self.category = Some(TokenCategory::Keyword),
            "ROUND_HALF_EVEN" => self.category = Some(TokenCategory::Keyword),
            "ROUND_HALF_UP" => self.category = Some(TokenCategory::Keyword),
            "ROUND_UP" => self.category = Some(TokenCategory::Keyword),
            "ROUTINE" => self.category = Some(TokenCategory::Keyword),
            "ROW" => self.category = Some(TokenCategory::Keyword),
            "ROWCOUNT" => self.category = Some(TokenCategory::Keyword),
            "ROWGUIDCOL" => self.category = Some(TokenCategory::Keyword),
            "ROWID" => self.category = Some(TokenCategory::Keyword),
            "ROWNUM" => self.category = Some(TokenCategory::Keyword),
            "ROWS" => self.category = Some(TokenCategory::Keyword),
            "ROWSET" => self.category = Some(TokenCategory::Keyword),
            "RULE" => self.category = Some(TokenCategory::Keyword),
            "RUN" => self.category = Some(TokenCategory::Keyword),
            "RUNNING" => self.category = Some(TokenCategory::Keyword),
            "SAMPLE" => self.category = Some(TokenCategory::Keyword),
            "SAMPLEID" => self.category = Some(TokenCategory::Keyword),
            "SAVE" => self.category = Some(TokenCategory::Keyword),
            "SAVEPOINT" => self.category = Some(TokenCategory::Keyword),
            "SCHEMA" => self.category = Some(TokenCategory::Keyword),
            "SCHEMAS" => self.category = Some(TokenCategory::Keyword),
            "SCOPE" => self.category = Some(TokenCategory::Keyword),
            "SCRATCHPAD" => self.category = Some(TokenCategory::Keyword),
            "SCROLL" => self.category = Some(TokenCategory::Keyword),
            "SEARCH" => self.category = Some(TokenCategory::Keyword),
            "SECONDS" => self.category = Some(TokenCategory::Keyword),
            "SECOND_MICROSECOND" => self.category = Some(TokenCategory::Keyword),
            "SECQTY" => self.category = Some(TokenCategory::Keyword),
            "SECTION" => self.category = Some(TokenCategory::Keyword),
            "SECURITY" => self.category = Some(TokenCategory::Keyword),
            "SECURITYAUDIT" => self.category = Some(TokenCategory::Keyword),
            "SEEK" => self.category = Some(TokenCategory::Keyword),
            "SEL" => self.category = Some(TokenCategory::Keyword),
            "SELECT" => self.category = Some(TokenCategory::Keyword),
            "SEMANTICKEYPHRASETABLE" => self.category = Some(TokenCategory::Keyword),
            "SEMANTICSIMILARITYDETAILSTABLE" => self.category = Some(TokenCategory::Keyword),
            "SEMANTICSIMILARITYTABLE" => self.category = Some(TokenCategory::Keyword),
            "SENSITIVE" => self.category = Some(TokenCategory::Keyword),
            "SEPARATOR" => self.category = Some(TokenCategory::Keyword),
            "SEQUENCE" => self.category = Some(TokenCategory::Keyword),
            "SESSION" => self.category = Some(TokenCategory::Keyword),
            "SESSIONPROPERTY" => self.category = Some(TokenCategory::Keyword),
            "SETRESRATE" => self.category = Some(TokenCategory::Keyword),
            "SETS" => self.category = Some(TokenCategory::Keyword),
            "SETSESSRATE" => self.category = Some(TokenCategory::Keyword),
            "SETUSER" => self.category = Some(TokenCategory::Keyword),
            "SHARE" => self.category = Some(TokenCategory::Keyword),
            "SHOW" => self.category = Some(TokenCategory::Keyword),
            "SHUTDOWN" => self.category = Some(TokenCategory::Keyword),
            "SIGNAL" => self.category = Some(TokenCategory::Keyword),
            "SIMILAR" => self.category = Some(TokenCategory::Keyword),
            "SIMPLE" => self.category = Some(TokenCategory::Keyword),
            "SINH" => self.category = Some(TokenCategory::Keyword),
            "SIZE" => self.category = Some(TokenCategory::Keyword),
            "SKEW" => self.category = Some(TokenCategory::Keyword),
            "SKIP" => self.category = Some(TokenCategory::Keyword),
            "SOME" => self.category = Some(TokenCategory::Keyword),
            "SOUNDEX" => self.category = Some(TokenCategory::Keyword),
            "SOURCE" => self.category = Some(TokenCategory::Keyword),
            "SPATIAL" => self.category = Some(TokenCategory::Keyword),
            "SPECIFIC" => self.category = Some(TokenCategory::Keyword),
            "SPECIFICTYPE" => self.category = Some(TokenCategory::Keyword),
            "SPOOL" => self.category = Some(TokenCategory::Keyword),
            "SQL" => self.category = Some(TokenCategory::Keyword),
            "SQLCA" => self.category = Some(TokenCategory::Keyword),
            "SQLCODE" => self.category = Some(TokenCategory::Keyword),
            "SQLERROR" => self.category = Some(TokenCategory::Keyword),
            "SQLEXCEPTION" => self.category = Some(TokenCategory::Keyword),
            "SQLSTATE" => self.category = Some(TokenCategory::Keyword),
            "SQLTEXT" => self.category = Some(TokenCategory::Keyword),
            "SQLWARNING" => self.category = Some(TokenCategory::Keyword),
            "SQL_BIG_RESULT" => self.category = Some(TokenCategory::Keyword),
            "SQL_CALC_FOUND_ROWS" => self.category = Some(TokenCategory::Keyword),
            "SQL_SMALL_RESULT" => self.category = Some(TokenCategory::Keyword),
            "SQUARE" => self.category = Some(TokenCategory::Keyword),
            "SS" => self.category = Some(TokenCategory::Keyword),
            "SSL" => self.category = Some(TokenCategory::Keyword),
            "STANDARD" => self.category = Some(TokenCategory::Keyword),
            "START" => self.category = Some(TokenCategory::Keyword),
            "STARTING" => self.category = Some(TokenCategory::Keyword),
            "STARTUP" => self.category = Some(TokenCategory::Keyword),
            "STATE" => self.category = Some(TokenCategory::Keyword),
            "STATEMENT" => self.category = Some(TokenCategory::Keyword),
            "STATIC" => self.category = Some(TokenCategory::Keyword),
            "STATISTICS" => self.category = Some(TokenCategory::Keyword),
            "STAY" => self.category = Some(TokenCategory::Keyword),
            "STDDEV_POP" => self.category = Some(TokenCategory::Keyword),
            "STDDEV_SAMP" => self.category = Some(TokenCategory::Keyword),
            "STEPINFO" => self.category = Some(TokenCategory::Keyword),
            "STOGROUP" => self.category = Some(TokenCategory::Keyword),
            "STORED" => self.category = Some(TokenCategory::Keyword),
            "STORES" => self.category = Some(TokenCategory::Keyword),
            "STR" => self.category = Some(TokenCategory::Keyword),
            "STRAIGHT_JOIN" => self.category = Some(TokenCategory::Keyword),
            "STRING_CS" => self.category = Some(TokenCategory::Keyword),
            "STRUCTURE" => self.category = Some(TokenCategory::Keyword),
            "STUFF" => self.category = Some(TokenCategory::Keyword),
            "STYLE" => self.category = Some(TokenCategory::Keyword),
            "SUBMULTISET" => self.category = Some(TokenCategory::Keyword),
            "SUBSCRIBER" => self.category = Some(TokenCategory::Keyword),
            "SUBSET" => self.category = Some(TokenCategory::Keyword),
            "SUBSTRING_REGEX" => self.category = Some(TokenCategory::Keyword),
            "SUCCEEDS" => self.category = Some(TokenCategory::Keyword),
            "SUCCESSFUL" => self.category = Some(TokenCategory::Keyword),
            "SUMMARY" => self.category = Some(TokenCategory::Keyword),
            "SUSPEND" => self.category = Some(TokenCategory::Keyword),
            "SYMMETRIC" => self.category = Some(TokenCategory::Keyword),
            "SYNONYM" => self.category = Some(TokenCategory::Keyword),
            "SYSDATETIME" => self.category = Some(TokenCategory::Keyword),
            "SYSTEM" => self.category = Some(TokenCategory::Keyword),
            "SYSTEM_TIME" => self.category = Some(TokenCategory::Keyword),
            "SYSTIMESTAMP" => self.category = Some(TokenCategory::Keyword),
            "TABLE" => self.category = Some(TokenCategory::Keyword),
            "TABLESAMPLE" => self.category = Some(TokenCategory::Keyword),
            "TABLESPACE" => self.category = Some(TokenCategory::Keyword),
            "TANH" => self.category = Some(TokenCategory::Keyword),
            "TBL_CS" => self.category = Some(TokenCategory::Keyword),
            "TEMPORARY" => self.category = Some(TokenCategory::Keyword),
            "TERMINATE" => self.category = Some(TokenCategory::Keyword),
            "TERMINATED" => self.category = Some(TokenCategory::Keyword),
            "TEXTSIZE" => self.category = Some(TokenCategory::Keyword),
            "THAN" => self.category = Some(TokenCategory::Keyword),
            "THEN" => self.category = Some(TokenCategory::Keyword),
            "THRESHOLD" => self.category = Some(TokenCategory::Keyword),
            "TIMEZONE_HOUR" => self.category = Some(TokenCategory::Keyword),
            "TIMEZONE_MINUTE" => self.category = Some(TokenCategory::Keyword),
            "TITLE" => self.category = Some(TokenCategory::Keyword),
            "TO" => self.category = Some(TokenCategory::Keyword),
            "TOP" => self.category = Some(TokenCategory::Keyword),
            "TRACE" => self.category = Some(TokenCategory::Keyword),
            "TRAILING" => self.category = Some(TokenCategory::Keyword),
            "TRAN" => self.category = Some(TokenCategory::Keyword),
            "TRANSACTION" => self.category = Some(TokenCategory::Keyword),
            "TRANSLATE" => self.category = Some(TokenCategory::Keyword),
            "TRANSLATE_CHK" => self.category = Some(TokenCategory::Keyword),
            "TRANSLATE_REGEX" => self.category = Some(TokenCategory::Keyword),
            "TRANSLATION" => self.category = Some(TokenCategory::Keyword),
            "TREAT" => self.category = Some(TokenCategory::Keyword),
            "TRIGGER" => self.category = Some(TokenCategory::Keyword),
            "TRIM_ARRAY" => self.category = Some(TokenCategory::Keyword),
            "TRUE" => self.category = Some(TokenCategory::Keyword),
            "TRY_CONVERT" => self.category = Some(TokenCategory::Keyword),
            "TSEQUAL" => self.category = Some(TokenCategory::Keyword),
            "TYPE" => self.category = Some(TokenCategory::Keyword),
            "UC" => self.category = Some(TokenCategory::Keyword),
            "UESCAPE" => self.category = Some(TokenCategory::Keyword),
            "UID" => self.category = Some(TokenCategory::Keyword),
            "UNDEFINED" => self.category = Some(TokenCategory::Keyword),
            "UNDER" => self.category = Some(TokenCategory::Keyword),
            "UNDO" => self.category = Some(TokenCategory::Keyword),
            "UNICODE" => self.category = Some(TokenCategory::Keyword),
            "UNION" => self.category = Some(TokenCategory::Keyword),
            "UNIQUE" => self.category = Some(TokenCategory::Keyword),
            "UNKNOWN" => self.category = Some(TokenCategory::Keyword),
            "UNLOCK" => self.category = Some(TokenCategory::Keyword),
            "UNNEST" => self.category = Some(TokenCategory::Keyword),
            "UNPIVOT" => self.category = Some(TokenCategory::Keyword),
            "UNSIGNED" => self.category = Some(TokenCategory::Keyword),
            "UNTIL" => self.category = Some(TokenCategory::Keyword),
            "UPD" => self.category = Some(TokenCategory::Keyword),
            "UPDATE" => self.category = Some(TokenCategory::Keyword),
            "UPDATETEXT" => self.category = Some(TokenCategory::Keyword),
            "UPPERCASE" => self.category = Some(TokenCategory::Keyword),
            "USAGE" => self.category = Some(TokenCategory::Keyword),
            "USE" => self.category = Some(TokenCategory::Keyword),
            "USER_NAME" => self.category = Some(TokenCategory::Keyword),
            "USING" => self.category = Some(TokenCategory::Keyword),
            "UTC_DATE" => self.category = Some(TokenCategory::Keyword),
            "UTC_TIME" => self.category = Some(TokenCategory::Keyword),
            "UTC_TIMESTAMP" => self.category = Some(TokenCategory::Keyword),
            "VALIDATE" => self.category = Some(TokenCategory::Keyword),
            "VALIDPROC" => self.category = Some(TokenCategory::Keyword),
            "VALUE" => self.category = Some(TokenCategory::Keyword),
            "VALUES" => self.category = Some(TokenCategory::Keyword),
            "VALUE_OF" => self.category = Some(TokenCategory::Keyword),
            "VARGRAPHIC" => self.category = Some(TokenCategory::Keyword),
            "VARIABLE" => self.category = Some(TokenCategory::Keyword),
            "VARIADIC" => self.category = Some(TokenCategory::Keyword),
            "VARIANT" => self.category = Some(TokenCategory::Keyword),
            "VARYING" => self.category = Some(TokenCategory::Keyword),
            "VAR_POP" => self.category = Some(TokenCategory::Keyword),
            "VAR_SAMP" => self.category = Some(TokenCategory::Keyword),
            "VCAT" => self.category = Some(TokenCategory::Keyword),
            "VERBOSE" => self.category = Some(TokenCategory::Keyword),
            "VERSIONING" => self.category = Some(TokenCategory::Keyword),
            "VIEW" => self.category = Some(TokenCategory::Keyword),
            "VIRTUAL" => self.category = Some(TokenCategory::Keyword),
            "VOLATILE" => self.category = Some(TokenCategory::Keyword),
            "VOLUMES" => self.category = Some(TokenCategory::Keyword),
            "WAIT" => self.category = Some(TokenCategory::Keyword),
            "WAITFOR" => self.category = Some(TokenCategory::Keyword),
            "WHEN" => self.category = Some(TokenCategory::Keyword),
            "WHENEVER" => self.category = Some(TokenCategory::Keyword),
            "WHERE" => self.category = Some(TokenCategory::Keyword),
            "WHILE" => self.category = Some(TokenCategory::Keyword),
            "WIDTH_BUCKET" => self.category = Some(TokenCategory::Keyword),
            "WINDOW" => self.category = Some(TokenCategory::Keyword),
            "WITH" => self.category = Some(TokenCategory::Keyword),
            "WITHIN" => self.category = Some(TokenCategory::Keyword),
            "WITHIN_GROUP" => self.category = Some(TokenCategory::Keyword),
            "WITHOUT" => self.category = Some(TokenCategory::Keyword),
            "WLM" => self.category = Some(TokenCategory::Keyword),
            "WORK" => self.category = Some(TokenCategory::Keyword),
            "WRITE" => self.category = Some(TokenCategory::Keyword),
            "WRITETEXT" => self.category = Some(TokenCategory::Keyword),
            "XMLCAST" => self.category = Some(TokenCategory::Keyword),
            "XMLEXISTS" => self.category = Some(TokenCategory::Keyword),
            "XMLNAMESPACES" => self.category = Some(TokenCategory::Keyword),
            "XOR" => self.category = Some(TokenCategory::Keyword),
            "YEARS" => self.category = Some(TokenCategory::Keyword),
            "YEAR_MONTH" => self.category = Some(TokenCategory::Keyword),
            "ZEROFILL" => self.category = Some(TokenCategory::Keyword),
            "ZEROIFNULL" => self.category = Some(TokenCategory::Keyword),
            "ZONE" => self.category = Some(TokenCategory::Keyword),

            // DataTypes
            "BIGINT" => self.category = Some(TokenCategory::DataType),
            "BINARY" => self.category = Some(TokenCategory::DataType),
            "BIT" => self.category = Some(TokenCategory::DataType),
            "BLOB" => self.category = Some(TokenCategory::DataType),
            "BOOL" => self.category = Some(TokenCategory::DataType),
            "BOOLEAN" => self.category = Some(TokenCategory::DataType),
            "CHAR" => self.category = Some(TokenCategory::DataType),
            "DATE" => self.category = Some(TokenCategory::DataType),
            "DATETIME" => self.category = Some(TokenCategory::DataType),
            "DATETIME2" => self.category = Some(TokenCategory::DataType),
            "DATETIMEOFFSET" => self.category = Some(TokenCategory::DataType),
            "DEC" => self.category = Some(TokenCategory::DataType),
            "DECIMAL" => self.category = Some(TokenCategory::DataType),
            "DOUBLE" => self.category = Some(TokenCategory::DataType),
            "ENUM" => self.category = Some(TokenCategory::DataType),
            "FLOAT" => self.category = Some(TokenCategory::DataType),
            "INT" => self.category = Some(TokenCategory::DataType),
            "INTEGER" => self.category = Some(TokenCategory::DataType),
            "KEY" => self.category = Some(TokenCategory::DataType),
            "LONGBLOB" => self.category = Some(TokenCategory::DataType),
            "LONGTEXT" => self.category = Some(TokenCategory::DataType),
            "MEDIUMBLOB" => self.category = Some(TokenCategory::DataType),
            "MEDIUMINT" => self.category = Some(TokenCategory::DataType),
            "MEDIUMTEXT" => self.category = Some(TokenCategory::DataType),
            "MONEY" => self.category = Some(TokenCategory::DataType),
            "NCHAR" => self.category = Some(TokenCategory::DataType),
            "NUMERIC" => self.category = Some(TokenCategory::DataType),
            "NVARCHAR" => self.category = Some(TokenCategory::DataType),
            "REAL" => self.category = Some(TokenCategory::DataType),
            "SET" => self.category = Some(TokenCategory::DataType),
            "SMALLDATETIME" => self.category = Some(TokenCategory::DataType),
            "SMALLINT" => self.category = Some(TokenCategory::DataType),
            "SMALLMONEY" => self.category = Some(TokenCategory::DataType),
            "SQL_VARIANT" => self.category = Some(TokenCategory::DataType),
            "TEXT" => self.category = Some(TokenCategory::DataType),
            "TIME" => self.category = Some(TokenCategory::DataType),
            "TIMESTAMP" => self.category = Some(TokenCategory::DataType),
            "TINYBLOB" => self.category = Some(TokenCategory::DataType),
            "TINYINT" => self.category = Some(TokenCategory::DataType),
            "TINYTEXT" => self.category = Some(TokenCategory::DataType),
            "UNIQUEIDENTIFIER" => self.category = Some(TokenCategory::DataType),
            "UUID" => self.category = Some(TokenCategory::DataType),
            "VARBINARY" => self.category = Some(TokenCategory::DataType),
            "VARBYTE" => self.category = Some(TokenCategory::DataType),
            "VARCHAR" => self.category = Some(TokenCategory::DataType),
            "VARCHAR2" => self.category = Some(TokenCategory::DataType),
            "VARCHARACTER" => self.category = Some(TokenCategory::DataType),
            "XML" => self.category = Some(TokenCategory::DataType),
            "YEAR" => self.category = Some(TokenCategory::DataType),

            // Methods
            "ABS" => self.category = Some(TokenCategory::Method),
            "ACOS" => self.category = Some(TokenCategory::Method),
            "ADDDATE" => self.category = Some(TokenCategory::Method),
            "ADDTIME" => self.category = Some(TokenCategory::Method),
            "ASIN" => self.category = Some(TokenCategory::Method),
            "ATAN" => self.category = Some(TokenCategory::Method),
            "ATAN2" => self.category = Some(TokenCategory::Method),
            "AVG" => self.category = Some(TokenCategory::Method),
            "BIN" => self.category = Some(TokenCategory::Method),
            "CALL" => self.category = Some(TokenCategory::Method),
            "CAST" => self.category = Some(TokenCategory::Method),
            "CEIL" => self.category = Some(TokenCategory::Method),
            "CEILING" => self.category = Some(TokenCategory::Method),
            "CHARACTER_LENGTH" => self.category = Some(TokenCategory::Method),
            "CHAR_LENGTH" => self.category = Some(TokenCategory::Method),
            "COALESCE" => self.category = Some(TokenCategory::Method),
            "CONCAT" => self.category = Some(TokenCategory::Method),
            "CONCAT_WS" => self.category = Some(TokenCategory::Method),
            "CONNECTION_ID" => self.category = Some(TokenCategory::Method),
            "CONV" => self.category = Some(TokenCategory::Method),
            "CONVERT" => self.category = Some(TokenCategory::Method),
            "COS" => self.category = Some(TokenCategory::Method),
            "COT" => self.category = Some(TokenCategory::Method),
            "COUNT" => self.category = Some(TokenCategory::Method),
            "CURDATE" => self.category = Some(TokenCategory::Method),
            "CURRENT_DATE" => self.category = Some(TokenCategory::Method),
            "CURRENT_TIME" => self.category = Some(TokenCategory::Method),
            "CURRENT_TIMESTAMP" => self.category = Some(TokenCategory::Method),
            "CURRENT_USER" => self.category = Some(TokenCategory::Method),
            "CURTIME" => self.category = Some(TokenCategory::Method),
            "DATEDIFF" => self.category = Some(TokenCategory::Method),
            "DATE_ADD" => self.category = Some(TokenCategory::Method),
            "DATE_FORMAT" => self.category = Some(TokenCategory::Method),
            "DATE_SUB" => self.category = Some(TokenCategory::Method),
            "DAY" => self.category = Some(TokenCategory::Method),
            "DAYNAME" => self.category = Some(TokenCategory::Method),
            "DAYOFMONTH" => self.category = Some(TokenCategory::Method),
            "DAYOFWEEK" => self.category = Some(TokenCategory::Method),
            "DAYOFYEAR" => self.category = Some(TokenCategory::Method),
            "DEGREES" => self.category = Some(TokenCategory::Method),
            "DIV" => self.category = Some(TokenCategory::Method),
            "EXEC" => self.category = Some(TokenCategory::Method),
            "EXECUTE" => self.category = Some(TokenCategory::Method),
            "EXP" => self.category = Some(TokenCategory::Method),
            "EXTRACT" => self.category = Some(TokenCategory::Method),
            "FIND_IN_SET" => self.category = Some(TokenCategory::Method),
            "FIRST" => self.category = Some(TokenCategory::Method),
            "FIRST_VALUE" => self.category = Some(TokenCategory::Method),
            "FLOOR" => self.category = Some(TokenCategory::Method),
            "FROM_DAYS" => self.category = Some(TokenCategory::Method),
            "GETDATE" => self.category = Some(TokenCategory::Method),
            "GREATEST" => self.category = Some(TokenCategory::Method),
            "HOUR" => self.category = Some(TokenCategory::Method),
            "IF" => self.category = Some(TokenCategory::Method),
            "IFNULL" => self.category = Some(TokenCategory::Method),
            "IIF" => self.category = Some(TokenCategory::Method),
            "INSTR" => self.category = Some(TokenCategory::Method),
            "ISNULL" => self.category = Some(TokenCategory::Method),
            "LAG" => self.category = Some(TokenCategory::Method),
            "LAST" => self.category = Some(TokenCategory::Method),
            "LAST_DAY" => self.category = Some(TokenCategory::Method),
            "LAST_INSERT_ID" => self.category = Some(TokenCategory::Method),
            "LAST_VALUE" => self.category = Some(TokenCategory::Method),
            "LCASE" => self.category = Some(TokenCategory::Method),
            "LEAD" => self.category = Some(TokenCategory::Method),
            "LEAST" => self.category = Some(TokenCategory::Method),
            "LEFT" => self.category = Some(TokenCategory::Method),
            "LENGTH" => self.category = Some(TokenCategory::Method),
            "LN" => self.category = Some(TokenCategory::Method),
            "LOCALTIME" => self.category = Some(TokenCategory::Method),
            "LOCALTIMESTAMP" => self.category = Some(TokenCategory::Method),
            "LOCATE" => self.category = Some(TokenCategory::Method),
            "LOG" => self.category = Some(TokenCategory::Method),
            "LOG10" => self.category = Some(TokenCategory::Method),
            "LOG2" => self.category = Some(TokenCategory::Method),
            "LOWER" => self.category = Some(TokenCategory::Method),
            "LPAD" => self.category = Some(TokenCategory::Method),
            "LTRIM" => self.category = Some(TokenCategory::Method),
            "MAKEDATE" => self.category = Some(TokenCategory::Method),
            "MAKETIME" => self.category = Some(TokenCategory::Method),
            "MAX" => self.category = Some(TokenCategory::Method),
            "MICROSECOND" => self.category = Some(TokenCategory::Method),
            "MID" => self.category = Some(TokenCategory::Method),
            "MIN" => self.category = Some(TokenCategory::Method),
            "MINUTE" => self.category = Some(TokenCategory::Method),
            "MOD" => self.category = Some(TokenCategory::Method),
            "MONTH" => self.category = Some(TokenCategory::Method),
            "MONTHNAME" => self.category = Some(TokenCategory::Method),
            "NEWID" => self.category = Some(TokenCategory::Method),
            "NOW" => self.category = Some(TokenCategory::Method),
            "NULLIF" => self.category = Some(TokenCategory::Method),
            "PERIOD_ADD" => self.category = Some(TokenCategory::Method),
            "PERIOD_DIFF" => self.category = Some(TokenCategory::Method),
            "PI" => self.category = Some(TokenCategory::Method),
            "POSITION" => self.category = Some(TokenCategory::Method),
            "POW" => self.category = Some(TokenCategory::Method),
            "POWER" => self.category = Some(TokenCategory::Method),
            "QUARTER" => self.category = Some(TokenCategory::Method),
            "RADIANS" => self.category = Some(TokenCategory::Method),
            "RAND" => self.category = Some(TokenCategory::Method),
            "REPEAT" => self.category = Some(TokenCategory::Method),
            "REPLACE" => self.category = Some(TokenCategory::Method),
            "REVERSE" => self.category = Some(TokenCategory::Method),
            "RIGHT" => self.category = Some(TokenCategory::Method),
            "ROUND" => self.category = Some(TokenCategory::Method),
            "ROW_NUMBER" => self.category = Some(TokenCategory::Method),
            "RPAD" => self.category = Some(TokenCategory::Method),
            "RTRIM" => self.category = Some(TokenCategory::Method),
            "SECOND" => self.category = Some(TokenCategory::Method),
            "SEC_TO_TIME" => self.category = Some(TokenCategory::Method),
            "SESSION_USER" => self.category = Some(TokenCategory::Method),
            "SIGN" => self.category = Some(TokenCategory::Method),
            "SIN" => self.category = Some(TokenCategory::Method),
            "SPACE" => self.category = Some(TokenCategory::Method),
            "SQRT" => self.category = Some(TokenCategory::Method),
            "STRCMP" => self.category = Some(TokenCategory::Method),
            "STR_TO_DATE" => self.category = Some(TokenCategory::Method),
            "SUBDATE" => self.category = Some(TokenCategory::Method),
            "SUBSTR" => self.category = Some(TokenCategory::Method),
            "SUBSTRING" => self.category = Some(TokenCategory::Method),
            "SUBSTRING_INDEX" => self.category = Some(TokenCategory::Method),
            "SUBTIME" => self.category = Some(TokenCategory::Method),
            "SUM" => self.category = Some(TokenCategory::Method),
            "SYSDATE" => self.category = Some(TokenCategory::Method),
            "SYSTEM_USER" => self.category = Some(TokenCategory::Method),
            "TAN" => self.category = Some(TokenCategory::Method),
            "TIMEDIFF" => self.category = Some(TokenCategory::Method),
            "TIME_FORMAT" => self.category = Some(TokenCategory::Method),
            "TIME_TO_SEC" => self.category = Some(TokenCategory::Method),
            "TO_DAYS" => self.category = Some(TokenCategory::Method),
            "TRIM" => self.category = Some(TokenCategory::Method),
            "TRUNCATE" => self.category = Some(TokenCategory::Method),
            "UCASE" => self.category = Some(TokenCategory::Method),
            "UPPER" => self.category = Some(TokenCategory::Method),
            "USER" => self.category = Some(TokenCategory::Method),
            "VERSION" => self.category = Some(TokenCategory::Method),
            "WEEK" => self.category = Some(TokenCategory::Method),
            "WEEKDAY" => self.category = Some(TokenCategory::Method),
            "WEEKOFYEAR" => self.category = Some(TokenCategory::Method),
            "YEARWEEK" => self.category = Some(TokenCategory::Method),
            _ => (),
        };
    }

    fn set_behavior(&mut self) {
        let mut behavior: Vec<TokenBehavior> = vec![];

        match self.category {
            Some(TokenCategory::Delimiter) => (),
            _ => (),
        }

        match self.value.to_uppercase().as_str() {
            "AFTER" => behavior.push(TokenBehavior::NewLineBefore),
            "AND" => behavior.push(TokenBehavior::NewLineBefore),
            "BEFORE" => behavior.push(TokenBehavior::NewLineBefore),
            "BEGIN" => behavior.push(TokenBehavior::NewLineBefore),
            "CALL" => behavior.push(TokenBehavior::NewLineBefore),
            "CASE" => {
                behavior.push(TokenBehavior::NewLineBefore);
                behavior.push(TokenBehavior::NewLineAfter);
            }
            "CLOSE" => behavior.push(TokenBehavior::NewLineBefore),
            "CROSS" => behavior.push(TokenBehavior::NewLineBefore),
            "DECLARE" => behavior.push(TokenBehavior::NewLineBefore),
            "DELETE" => behavior.push(TokenBehavior::NewLineBefore),
            "DISTINCT" => behavior.push(TokenBehavior::NewLineAfter),
            "DO" => {
                behavior.push(TokenBehavior::NewLineBefore);
                behavior.push(TokenBehavior::NewLineAfter);
            }
            "DROP" => behavior.push(TokenBehavior::NewLineBefore),
            "ELSE" => behavior.push(TokenBehavior::NewLineBefore),
            "END" => behavior.push(TokenBehavior::NewLineBefore),
            "EXEC" => behavior.push(TokenBehavior::NewLineBefore),
            "EXECUTE" => behavior.push(TokenBehavior::NewLineBefore),
            "FETCH" => behavior.push(TokenBehavior::NewLineBefore),
            "FOR" => behavior.push(TokenBehavior::NewLineBefore),
            "FROM" => behavior.push(TokenBehavior::NewLineBefore),
            "GROUP" => behavior.push(TokenBehavior::NewLineBefore),
            "INNER" => behavior.push(TokenBehavior::NewLineBefore),
            "LEFT" => behavior.push(TokenBehavior::NewLineBefore),
            "LIMIT" => behavior.push(TokenBehavior::NewLineBefore),
            "OPEN" => behavior.push(TokenBehavior::NewLineBefore),
            "OR" => behavior.push(TokenBehavior::NewLineBefore),
            "ORDER" => behavior.push(TokenBehavior::NewLineBefore),
            "OUTER" => behavior.push(TokenBehavior::NewLineBefore),
            "PRIMARY" => behavior.push(TokenBehavior::NewLineBefore),
            "RETURN" => behavior.push(TokenBehavior::NewLineBefore),
            "RIGHT" => behavior.push(TokenBehavior::NewLineBefore),
            "SELECT" => behavior.push(TokenBehavior::NewLineBefore),
            "SET" => behavior.push(TokenBehavior::NewLineBefore),
            "UNION" => {
                behavior.push(TokenBehavior::NewLineBefore);
                behavior.push(TokenBehavior::NewLineAfter);
            }
            "WHEN" => behavior.push(TokenBehavior::NewLineBefore),
            "WHERE" => behavior.push(TokenBehavior::NewLineBefore),
            _ => (),
        };

        self.behavior = behavior;
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum TokenCategory {
    Space,
    Comment,
    Quote,
    NewLine,
    Delimiter,
    Comma,
    ParenOpen,
    ParenClose,
    Operator,
    Bitwise,
    Compare,
    Keyword,
    DataType,
    Method,
}

#[derive(Debug, PartialEq, Clone)]
pub enum TokenBehavior {
    NewLineBefore,
    NewLineAfter,
}

#[derive(Clone)]
enum CommentCategory {
    SingleLine,
    MultiLine,
}

#[derive(Clone)]
enum QuoteCategory {
    Backtick,
    QuoteSingle,
    QuoteDouble,
    Bracket,
    CurlyBracket,
}

pub fn get_sql_tokens(sql: String) -> Vec<Token> {
    let mut tokens: Vec<Token> = vec![];

    let mut curr_token: Token = Token::new();
    let mut in_comment: Option<CommentCategory> = None;
    let mut in_quote: Option<QuoteCategory> = None;

    let sql_bytes: &[u8] = sql.as_bytes();
    for i in 0..sql_bytes.len() {
        let curr_ch: char = sql_bytes[i].into();

        let prev2_ch: Option<char> = if i >= 2 {
            Some(sql_bytes[i - 2].into())
        } else {
            None
        };
        let prev_ch: Option<char> = if i >= 1 {
            Some(sql_bytes[i - 1].into())
        } else {
            None
        };
        let next_ch: Option<char> = if (i + 1) < sql_bytes.len() {
            Some(sql_bytes[i + 1].into())
        } else {
            None
        };

        let was_in_comment: Option<CommentCategory> = in_comment.clone();
        in_comment = get_in_comment(
            &in_comment,
            prev2_ch,
            prev_ch,
            curr_ch,
            next_ch,
            curr_token.len(),
        );
        if in_comment.is_some() {
            if was_in_comment.is_none() {
                // start of new comment, add any current token if any
                if !curr_token.is_empty() {
                    curr_token.setup();
                    tokens.push(curr_token);
                    curr_token = Token::new();
                }
                curr_token.category = Some(TokenCategory::Comment);
            }

            curr_token.value.push(curr_ch);
            continue;
        } else if was_in_comment.is_some() {
            // comment just ended, add comment token
            tokens.push(curr_token);
            curr_token = Token::new();
        }

        let was_in_quote: Option<QuoteCategory> = in_quote.clone();
        in_quote = get_in_quote(&in_quote, prev_ch, curr_ch, next_ch, &curr_token);
        if in_quote.is_some() {
            if was_in_quote.is_none() {
                // start of new quote, add any current token if any
                if !curr_token.is_empty() {
                    curr_token.setup();
                    tokens.push(curr_token);
                    curr_token = Token::new();
                }
                curr_token.category = Some(TokenCategory::Quote);
            }

            curr_token.value.push(curr_ch);
            continue;
        } else if was_in_quote.is_some() {
            // quote just ended, add quote token
            tokens.push(curr_token);
            curr_token = Token::new();
        }

        match curr_ch {
            DELIMITER | NEW_LINE | COMMA | PAREN_OPEN | PAREN_CLOSE | AMPERSAND | VERTICAL_BAR
            | CIRCUMFLEX => {
                if !curr_token.is_empty() {
                    curr_token.setup();
                    tokens.push(curr_token);
                    curr_token = Token::new();
                }
                curr_token.value.push(curr_ch);
                curr_token.category = match curr_ch {
                    DELIMITER => Some(TokenCategory::Delimiter),
                    NEW_LINE => Some(TokenCategory::NewLine),
                    COMMA => Some(TokenCategory::Comma),
                    PAREN_OPEN => Some(TokenCategory::ParenOpen),
                    PAREN_CLOSE => Some(TokenCategory::ParenClose),
                    AMPERSAND => Some(TokenCategory::Bitwise),
                    VERTICAL_BAR => Some(TokenCategory::Bitwise),
                    CIRCUMFLEX => Some(TokenCategory::Bitwise),
                    _ => None,
                };
                tokens.push(curr_token);
                curr_token = Token::new();
                continue;
            }
            LESS_THAN | ASTERISK | SLASH_FORWARD | PERCENT => {
                if !curr_token.is_empty() {
                    curr_token.setup();
                    tokens.push(curr_token);
                    curr_token = Token::new();
                }
                curr_token.value.push(curr_ch);
                curr_token.category = match curr_ch {
                    LESS_THAN => Some(TokenCategory::Compare),
                    PLUS => Some(TokenCategory::Operator),
                    HYPHEN => Some(TokenCategory::Operator),
                    ASTERISK => Some(TokenCategory::Operator),
                    SLASH_FORWARD => Some(TokenCategory::Operator),
                    PERCENT => Some(TokenCategory::Operator),
                    _ => None,
                };

                if next_ch != Some(EQUAL) && next_ch != Some(GREATER_THAN) {
                    tokens.push(curr_token);
                    curr_token = Token::new();
                }

                continue;
            }
            EQUAL | GREATER_THAN => {
                if !curr_token.is_empty()
                    && prev_ch != Some(LESS_THAN)
                    && prev_ch != Some(GREATER_THAN)
                    && prev_ch != Some(PLUS)
                    && prev_ch != Some(HYPHEN)
                    && prev_ch != Some(ASTERISK)
                    && prev_ch != Some(SLASH_FORWARD)
                    && prev_ch != Some(PERCENT)
                {
                    curr_token.setup();
                    tokens.push(curr_token);
                    curr_token = Token::new();
                }
                curr_token.value.push(curr_ch);
                curr_token.category = match prev_ch {
                    Some(PLUS) => Some(TokenCategory::Operator),
                    Some(HYPHEN) => Some(TokenCategory::Operator),
                    Some(ASTERISK) => Some(TokenCategory::Operator),
                    Some(SLASH_FORWARD) => Some(TokenCategory::Operator),
                    Some(PERCENT) => Some(TokenCategory::Operator),
                    _ => Some(TokenCategory::Compare),
                };

                if next_ch != Some(EQUAL) {
                    tokens.push(curr_token);
                    curr_token = Token::new();
                }

                continue;
            }
            PLUS | HYPHEN => {
                if !curr_token.is_empty() {
                    curr_token.setup();
                    tokens.push(curr_token);
                    curr_token = Token::new();
                }
                curr_token.value.push(curr_ch);

                if tokens.last().is_some_and(|t| t.category.is_some()) {
                    continue;
                }

                curr_token.category = Some(TokenCategory::Operator);

                if next_ch != Some(EQUAL) && next_ch != Some(GREATER_THAN) {
                    tokens.push(curr_token);
                    curr_token = Token::new();
                }

                continue;
            }
            _ => (),
        }

        if curr_ch.is_whitespace() {
            if !curr_token.is_empty() {
                curr_token.setup();
                tokens.push(curr_token);
                curr_token = Token::new();
            }
            continue;
        }

        curr_token.value.push(curr_ch);
    }

    if !curr_token.is_empty() {
        curr_token.setup();
        tokens.push(curr_token);
    }

    return tokens;
}

fn get_in_comment(
    in_comment: &Option<CommentCategory>,
    prev2_ch: Option<char>,
    prev_ch: Option<char>,
    curr_ch: char,
    next_ch: Option<char>,
    curr_token_len: usize,
) -> Option<CommentCategory> {
    match in_comment {
        Some(cc) => {
            if curr_token_len <= 1 {
                return in_comment.clone();
            }

            match cc {
                CommentCategory::SingleLine => {
                    if curr_ch == NEW_LINE {
                        return None;
                    }
                    return Some(CommentCategory::SingleLine);
                }
                CommentCategory::MultiLine => {
                    if prev2_ch == Some(ASTERISK) && prev_ch == Some(SLASH_FORWARD) {
                        return None;
                    }
                    return Some(CommentCategory::MultiLine);
                }
            }
        }
        None => {
            if curr_ch == HYPHEN && next_ch == Some(HYPHEN) {
                return Some(CommentCategory::SingleLine);
            }

            if curr_ch == SLASH_FORWARD && next_ch == Some(ASTERISK) {
                return Some(CommentCategory::MultiLine);
            }

            return None;
        }
    }
}

fn get_in_quote(
    in_quote: &Option<QuoteCategory>,
    prev_ch: Option<char>,
    curr_ch: char,
    next_ch: Option<char>,
    curr_token: &Token,
) -> Option<QuoteCategory> {
    match in_quote {
        Some(qc) => {
            if curr_token.len() <= 1 {
                return in_quote.clone();
            }

            match qc {
                QuoteCategory::Backtick => {
                    if prev_ch == Some(BACKTICK) {
                        return None;
                    }
                    return in_quote.clone();
                }
                QuoteCategory::QuoteSingle => {
                    if prev_ch == Some(QUOTE_SINGLE) && curr_ch != QUOTE_SINGLE {
                        if curr_token.count(QUOTE_SINGLE) % 2 == 0 {
                            return None;
                        }
                    }
                    return in_quote.clone();
                }
                QuoteCategory::QuoteDouble => {
                    if prev_ch == Some(QUOTE_DOUBLE) {
                        return None;
                    }
                    return in_quote.clone();
                }
                QuoteCategory::Bracket => {
                    if prev_ch == Some(BRACKET_CLOSE) && curr_ch != FULL_STOP {
                        return None;
                    }
                    return in_quote.clone();
                }
                QuoteCategory::CurlyBracket => {
                    if prev_ch == Some(CURLY_BRACKET_CLOSE) {
                        return None;
                    }
                    return in_quote.clone();
                }
            }
        }
        None => {
            return match curr_ch {
                BACKTICK => Some(QuoteCategory::Backtick),
                QUOTE_SINGLE => Some(QuoteCategory::QuoteSingle),
                QUOTE_DOUBLE => Some(QuoteCategory::QuoteDouble),
                BRACKET_OPEN => Some(QuoteCategory::Bracket),
                CURLY_BRACKET_OPEN => Some(QuoteCategory::CurlyBracket),
                'N' => {
                    if next_ch == Some(QUOTE_SINGLE) {
                        return Some(QuoteCategory::QuoteSingle);
                    }
                    return None;
                }
                _ => None,
            };
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_sql_tokens_basic() {
        assert_eq!(
            get_sql_tokens(String::from("SELECT * FROM TBL1")),
            vec![
                Token {
                    value: String::from("SELECT"),
                    category: Some(TokenCategory::Keyword),
                    behavior: vec![TokenBehavior::NewLineBefore],
                },
                Token {
                    value: String::from("*"),
                    category: Some(TokenCategory::Operator),
                    behavior: vec![],
                },
                Token {
                    value: String::from("FROM"),
                    category: Some(TokenCategory::Keyword),
                    behavior: vec![TokenBehavior::NewLineBefore],
                },
                Token {
                    value: String::from("TBL1"),
                    category: None,
                    behavior: vec![],
                },
            ]
        );
    }

    #[test]
    fn test_get_sql_tokens_comment_single_inline() {
        assert_eq!(
            get_sql_tokens(String::from("SELECT 1 --comment inline")),
            vec![
                Token {
                    value: String::from("SELECT"),
                    category: Some(TokenCategory::Keyword),
                    behavior: vec![TokenBehavior::NewLineBefore],
                },
                Token {
                    value: String::from("1"),
                    category: None,
                    behavior: vec![],
                },
                Token {
                    value: String::from("--comment inline"),
                    category: Some(TokenCategory::Comment),
                    behavior: vec![],
                },
            ]
        );
    }

    #[test]
    fn test_get_sql_tokens_comment_single_newline() {
        assert_eq!(
            get_sql_tokens(String::from(
                r#"SELECT *
                -- comment newline
                FROM TBL1"#
            )),
            vec![
                Token {
                    value: String::from("SELECT"),
                    category: Some(TokenCategory::Keyword),
                    behavior: vec![TokenBehavior::NewLineBefore],
                },
                Token {
                    value: String::from("*"),
                    category: Some(TokenCategory::Operator),
                    behavior: vec![],
                },
                Token {
                    value: String::from("\n"),
                    category: Some(TokenCategory::NewLine),
                    behavior: vec![],
                },
                Token {
                    value: String::from("-- comment newline"),
                    category: Some(TokenCategory::Comment),
                    behavior: vec![],
                },
                Token {
                    value: String::from("\n"),
                    category: Some(TokenCategory::NewLine),
                    behavior: vec![],
                },
                Token {
                    value: String::from("FROM"),
                    category: Some(TokenCategory::Keyword),
                    behavior: vec![TokenBehavior::NewLineBefore],
                },
                Token {
                    value: String::from("TBL1"),
                    category: None,
                    behavior: vec![],
                },
            ]
        );
    }

    #[test]
    fn test_get_sql_tokens_comment_multi_inline() {
        assert_eq!(
            get_sql_tokens(String::from("SELECT * /*multi inline*/ FROM TBL1")),
            vec![
                Token {
                    value: String::from("SELECT"),
                    category: Some(TokenCategory::Keyword),
                    behavior: vec![TokenBehavior::NewLineBefore],
                },
                Token {
                    value: String::from("*"),
                    category: Some(TokenCategory::Operator),
                    behavior: vec![],
                },
                Token {
                    value: String::from("/*multi inline*/"),
                    category: Some(TokenCategory::Comment),
                    behavior: vec![],
                },
                Token {
                    value: String::from("FROM"),
                    category: Some(TokenCategory::Keyword),
                    behavior: vec![TokenBehavior::NewLineBefore],
                },
                Token {
                    value: String::from("TBL1"),
                    category: None,
                    behavior: vec![],
                },
            ]
        );
    }

    #[test]
    fn test_get_sql_tokens_comment_multi_odd() {
        assert_eq!(
            get_sql_tokens(String::from("*/*multi odd*/*")),
            vec![
                Token {
                    value: String::from("*"),
                    category: Some(TokenCategory::Operator),
                    behavior: vec![],
                },
                Token {
                    value: String::from("/*multi odd*/"),
                    category: Some(TokenCategory::Comment),
                    behavior: vec![],
                },
                Token {
                    value: String::from("*"),
                    category: Some(TokenCategory::Operator),
                    behavior: vec![],
                },
            ]
        );
    }

    #[test]
    fn test_get_sql_tokens_comment_multi_newline() {
        assert_eq!(
            get_sql_tokens(String::from(
                r#"SELECT *
                /*
                    multi line
                    comment
                */
                FROM TBL1"#
            )),
            vec![
                Token {
                    value: String::from("SELECT"),
                    category: Some(TokenCategory::Keyword),
                    behavior: vec![TokenBehavior::NewLineBefore],
                },
                Token {
                    value: String::from("*"),
                    category: Some(TokenCategory::Operator),
                    behavior: vec![],
                },
                Token {
                    value: String::from("\n"),
                    category: Some(TokenCategory::NewLine),
                    behavior: vec![],
                },
                Token {
                    value: String::from(
                        r#"/*
                    multi line
                    comment
                */"#
                    ),
                    category: Some(TokenCategory::Comment),
                    behavior: vec![],
                },
                Token {
                    value: String::from("\n"),
                    category: Some(TokenCategory::NewLine),
                    behavior: vec![],
                },
                Token {
                    value: String::from("FROM"),
                    category: Some(TokenCategory::Keyword),
                    behavior: vec![TokenBehavior::NewLineBefore],
                },
                Token {
                    value: String::from("TBL1"),
                    category: None,
                    behavior: vec![],
                },
            ]
        );
    }

    #[test]
    fn test_get_sql_tokens_quote_backtick() {
        assert_eq!(
            get_sql_tokens(String::from("SELECT `Column 1`")),
            vec![
                Token {
                    value: String::from("SELECT"),
                    category: Some(TokenCategory::Keyword),
                    behavior: vec![TokenBehavior::NewLineBefore],
                },
                Token {
                    value: String::from("`Column 1`"),
                    category: Some(TokenCategory::Quote),
                    behavior: vec![],
                },
            ]
        );
    }

    #[test]
    fn test_get_sql_tokens_quote_single() {
        assert_eq!(
            get_sql_tokens(String::from("SELECT 'Column 1'")),
            vec![
                Token {
                    value: String::from("SELECT"),
                    category: Some(TokenCategory::Keyword),
                    behavior: vec![TokenBehavior::NewLineBefore],
                },
                Token {
                    value: String::from("'Column 1'"),
                    category: Some(TokenCategory::Quote),
                    behavior: vec![],
                },
            ]
        );
    }

    #[test]
    fn test_get_sql_tokens_quote_double() {
        assert_eq!(
            get_sql_tokens(String::from("SELECT \"Column 1\"")),
            vec![
                Token {
                    value: String::from("SELECT"),
                    category: Some(TokenCategory::Keyword),
                    behavior: vec![TokenBehavior::NewLineBefore],
                },
                Token {
                    value: String::from("\"Column 1\""),
                    category: Some(TokenCategory::Quote),
                    behavior: vec![],
                },
            ]
        );
    }

    #[test]
    fn test_get_sql_tokens_quote_bracket() {
        assert_eq!(
            get_sql_tokens(String::from("SELECT [Column 1]")),
            vec![
                Token {
                    value: String::from("SELECT"),
                    category: Some(TokenCategory::Keyword),
                    behavior: vec![TokenBehavior::NewLineBefore],
                },
                Token {
                    value: String::from("[Column 1]"),
                    category: Some(TokenCategory::Quote),
                    behavior: vec![],
                },
            ]
        );
    }

    #[test]
    fn test_get_sql_tokens_quote_bracket_schema() {
        assert_eq!(
            get_sql_tokens(String::from("SELECT * FROM [S].[TBL1]")),
            vec![
                Token {
                    value: String::from("SELECT"),
                    category: Some(TokenCategory::Keyword),
                    behavior: vec![TokenBehavior::NewLineBefore],
                },
                Token {
                    value: String::from("*"),
                    category: Some(TokenCategory::Operator),
                    behavior: vec![],
                },
                Token {
                    value: String::from("FROM"),
                    category: Some(TokenCategory::Keyword),
                    behavior: vec![TokenBehavior::NewLineBefore],
                },
                Token {
                    value: String::from("[S].[TBL1]"),
                    category: Some(TokenCategory::Quote),
                    behavior: vec![],
                },
            ]
        );
    }

    #[test]
    fn test_get_sql_tokens_quote_curly_bracket() {
        assert_eq!(
            get_sql_tokens(String::from("SELECT C1 FROM {tableNames[i]}")),
            vec![
                Token {
                    value: String::from("SELECT"),
                    category: Some(TokenCategory::Keyword),
                    behavior: vec![TokenBehavior::NewLineBefore],
                },
                Token {
                    value: String::from("C1"),
                    category: None,
                    behavior: vec![],
                },
                Token {
                    value: String::from("FROM"),
                    category: Some(TokenCategory::Keyword),
                    behavior: vec![TokenBehavior::NewLineBefore],
                },
                Token {
                    value: String::from("{tableNames[i]}"),
                    category: Some(TokenCategory::Quote),
                    behavior: vec![],
                },
            ]
        );
    }

    #[test]
    fn test_get_sql_tokens_quote_empty() {
        assert_eq!(
            get_sql_tokens(String::from("DECLARE V1 = '';")),
            vec![
                Token {
                    value: String::from("DECLARE"),
                    category: Some(TokenCategory::Keyword),
                    behavior: vec![TokenBehavior::NewLineBefore],
                },
                Token {
                    value: String::from("V1"),
                    category: None,
                    behavior: vec![],
                },
                Token {
                    value: String::from("="),
                    category: Some(TokenCategory::Compare),
                    behavior: vec![],
                },
                Token {
                    value: String::from("''"),
                    category: Some(TokenCategory::Quote),
                    behavior: vec![],
                },
                Token {
                    value: String::from(";"),
                    category: Some(TokenCategory::Delimiter),
                    behavior: vec![],
                },
            ]
        );
    }

    #[test]
    fn test_get_sql_tokens_quote_of_empty() {
        assert_eq!(
            get_sql_tokens(String::from("DECLARE V1 = '''';")),
            vec![
                Token {
                    value: String::from("DECLARE"),
                    category: Some(TokenCategory::Keyword),
                    behavior: vec![TokenBehavior::NewLineBefore],
                },
                Token {
                    value: String::from("V1"),
                    category: None,
                    behavior: vec![],
                },
                Token {
                    value: String::from("="),
                    category: Some(TokenCategory::Compare),
                    behavior: vec![],
                },
                Token {
                    value: String::from("''''"),
                    category: Some(TokenCategory::Quote),
                    behavior: vec![],
                },
                Token {
                    value: String::from(";"),
                    category: Some(TokenCategory::Delimiter),
                    behavior: vec![],
                },
            ]
        );
    }

    #[test]
    fn test_get_sql_tokens_quote_single_n() {
        assert_eq!(
            get_sql_tokens(String::from("SELECT N'Column Name'")),
            vec![
                Token {
                    value: String::from("SELECT"),
                    category: Some(TokenCategory::Keyword),
                    behavior: vec![TokenBehavior::NewLineBefore],
                },
                Token {
                    value: String::from("N'Column Name'"),
                    category: Some(TokenCategory::Quote),
                    behavior: vec![],
                },
            ]
        );
    }

    #[test]
    fn test_get_sql_tokens_quote_single_escape() {
        assert_eq!(
            get_sql_tokens(String::from("SELECT 'Column''s Name'")),
            vec![
                Token {
                    value: String::from("SELECT"),
                    category: Some(TokenCategory::Keyword),
                    behavior: vec![TokenBehavior::NewLineBefore],
                },
                Token {
                    value: String::from("'Column''s Name'"),
                    category: Some(TokenCategory::Quote),
                    behavior: vec![],
                },
            ]
        );
    }

    #[test]
    fn test_get_sql_tokens_quote_single_multiline() {
        assert_eq!(
            get_sql_tokens(String::from(
                r#"SELECT 'Column
Name'"#
            )),
            vec![
                Token {
                    value: String::from("SELECT"),
                    category: Some(TokenCategory::Keyword),
                    behavior: vec![TokenBehavior::NewLineBefore],
                },
                Token {
                    value: String::from(
                        r#"'Column
Name'"#
                    ),
                    category: Some(TokenCategory::Quote),
                    behavior: vec![],
                },
            ]
        );
    }

    #[test]
    fn test_get_sql_tokens_quote_single_abrupt_end() {
        assert_eq!(
            get_sql_tokens(String::from("SELECT 'Column")),
            vec![
                Token {
                    value: String::from("SELECT"),
                    category: Some(TokenCategory::Keyword),
                    behavior: vec![TokenBehavior::NewLineBefore],
                },
                Token {
                    value: String::from("'Column"),
                    category: Some(TokenCategory::Quote),
                    behavior: vec![],
                },
            ]
        );
    }

    #[test]
    fn test_get_sql_tokens_delimiter_basic() {
        assert_eq!(
            get_sql_tokens(String::from("SELECT 1;")),
            vec![
                Token {
                    value: String::from("SELECT"),
                    category: Some(TokenCategory::Keyword),
                    behavior: vec![TokenBehavior::NewLineBefore],
                },
                Token {
                    value: String::from("1"),
                    category: None,
                    behavior: vec![],
                },
                Token {
                    value: String::from(";"),
                    category: Some(TokenCategory::Delimiter),
                    behavior: vec![],
                },
            ]
        );
    }

    #[test]
    fn test_get_sql_tokens_delimiter_two() {
        assert_eq!(
            get_sql_tokens(String::from("SELECT 1; SELECT 1;")),
            vec![
                Token {
                    value: String::from("SELECT"),
                    category: Some(TokenCategory::Keyword),
                    behavior: vec![TokenBehavior::NewLineBefore],
                },
                Token {
                    value: String::from("1"),
                    category: None,
                    behavior: vec![],
                },
                Token {
                    value: String::from(";"),
                    category: Some(TokenCategory::Delimiter),
                    behavior: vec![],
                },
                Token {
                    value: String::from("SELECT"),
                    category: Some(TokenCategory::Keyword),
                    behavior: vec![TokenBehavior::NewLineBefore],
                },
                Token {
                    value: String::from("1"),
                    category: None,
                    behavior: vec![],
                },
                Token {
                    value: String::from(";"),
                    category: Some(TokenCategory::Delimiter),
                    behavior: vec![],
                },
            ]
        );
    }

    #[test]
    fn test_get_sql_tokens_comma() {
        assert_eq!(
            get_sql_tokens(String::from("SELECT 1,2, 3")),
            vec![
                Token {
                    value: String::from("SELECT"),
                    category: Some(TokenCategory::Keyword),
                    behavior: vec![TokenBehavior::NewLineBefore],
                },
                Token {
                    value: String::from("1"),
                    category: None,
                    behavior: vec![],
                },
                Token {
                    value: String::from(","),
                    category: Some(TokenCategory::Comma),
                    behavior: vec![],
                },
                Token {
                    value: String::from("2"),
                    category: None,
                    behavior: vec![],
                },
                Token {
                    value: String::from(","),
                    category: Some(TokenCategory::Comma),
                    behavior: vec![],
                },
                Token {
                    value: String::from("3"),
                    category: None,
                    behavior: vec![],
                },
            ]
        );
    }

    #[test]
    fn test_get_sql_tokens_negative_number() {
        assert_eq!(
            get_sql_tokens(String::from("SELECT -1")),
            vec![
                Token {
                    value: String::from("SELECT"),
                    category: Some(TokenCategory::Keyword),
                    behavior: vec![TokenBehavior::NewLineBefore],
                },
                Token {
                    value: String::from("-1"),
                    category: None,
                    behavior: vec![],
                },
            ]
        );
    }

    #[test]
    fn test_get_sql_tokens_paren_empty() {
        assert_eq!(
            get_sql_tokens(String::from("SELECT MIN()")),
            vec![
                Token {
                    value: String::from("SELECT"),
                    category: Some(TokenCategory::Keyword),
                    behavior: vec![TokenBehavior::NewLineBefore],
                },
                Token {
                    value: String::from("MIN"),
                    category: Some(TokenCategory::Method),
                    behavior: vec![],
                },
                Token {
                    value: String::from("("),
                    category: Some(TokenCategory::ParenOpen),
                    behavior: vec![],
                },
                Token {
                    value: String::from(")"),
                    category: Some(TokenCategory::ParenClose),
                    behavior: vec![],
                },
            ]
        );
    }

    #[test]
    fn test_get_sql_tokens_paren_content() {
        assert_eq!(
            get_sql_tokens(String::from("SELECT (SELECT 1)")),
            vec![
                Token {
                    value: String::from("SELECT"),
                    category: Some(TokenCategory::Keyword),
                    behavior: vec![TokenBehavior::NewLineBefore],
                },
                Token {
                    value: String::from("("),
                    category: Some(TokenCategory::ParenOpen),
                    behavior: vec![],
                },
                Token {
                    value: String::from("SELECT"),
                    category: Some(TokenCategory::Keyword),
                    behavior: vec![TokenBehavior::NewLineBefore],
                },
                Token {
                    value: String::from("1"),
                    category: None,
                    behavior: vec![],
                },
                Token {
                    value: String::from(")"),
                    category: Some(TokenCategory::ParenClose),
                    behavior: vec![],
                },
            ]
        );
    }

    #[test]
    fn test_get_sql_tokens_operator_add() {
        assert_eq!(
            get_sql_tokens(String::from("1+2 + 3")),
            vec![
                Token {
                    value: String::from("1"),
                    category: None,
                    behavior: vec![],
                },
                Token {
                    value: String::from("+"),
                    category: Some(TokenCategory::Operator),
                    behavior: vec![],
                },
                Token {
                    value: String::from("2"),
                    category: None,
                    behavior: vec![],
                },
                Token {
                    value: String::from("+"),
                    category: Some(TokenCategory::Operator),
                    behavior: vec![],
                },
                Token {
                    value: String::from("3"),
                    category: None,
                    behavior: vec![],
                },
            ]
        );
    }

    #[test]
    fn test_get_sql_tokens_operator_subtract() {
        assert_eq!(
            get_sql_tokens(String::from("1-2 - 3")),
            vec![
                Token {
                    value: String::from("1"),
                    category: None,
                    behavior: vec![],
                },
                Token {
                    value: String::from("-"),
                    category: Some(TokenCategory::Operator),
                    behavior: vec![],
                },
                Token {
                    value: String::from("2"),
                    category: None,
                    behavior: vec![],
                },
                Token {
                    value: String::from("-"),
                    category: Some(TokenCategory::Operator),
                    behavior: vec![],
                },
                Token {
                    value: String::from("3"),
                    category: None,
                    behavior: vec![],
                },
            ]
        );
    }

    #[test]
    fn test_get_sql_tokens_operator_multiply() {
        assert_eq!(
            get_sql_tokens(String::from("1*2 * 3")),
            vec![
                Token {
                    value: String::from("1"),
                    category: None,
                    behavior: vec![],
                },
                Token {
                    value: String::from("*"),
                    category: Some(TokenCategory::Operator),
                    behavior: vec![],
                },
                Token {
                    value: String::from("2"),
                    category: None,
                    behavior: vec![],
                },
                Token {
                    value: String::from("*"),
                    category: Some(TokenCategory::Operator),
                    behavior: vec![],
                },
                Token {
                    value: String::from("3"),
                    category: None,
                    behavior: vec![],
                },
            ]
        );
    }

    #[test]
    fn test_get_sql_tokens_operator_divide() {
        assert_eq!(
            get_sql_tokens(String::from("1/2 / 3")),
            vec![
                Token {
                    value: String::from("1"),
                    category: None,
                    behavior: vec![],
                },
                Token {
                    value: String::from("/"),
                    category: Some(TokenCategory::Operator),
                    behavior: vec![],
                },
                Token {
                    value: String::from("2"),
                    category: None,
                    behavior: vec![],
                },
                Token {
                    value: String::from("/"),
                    category: Some(TokenCategory::Operator),
                    behavior: vec![],
                },
                Token {
                    value: String::from("3"),
                    category: None,
                    behavior: vec![],
                },
            ]
        );
    }

    #[test]
    fn test_get_sql_tokens_operator_modulo() {
        assert_eq!(
            get_sql_tokens(String::from("1%2 % 3")),
            vec![
                Token {
                    value: String::from("1"),
                    category: None,
                    behavior: vec![],
                },
                Token {
                    value: String::from("%"),
                    category: Some(TokenCategory::Operator),
                    behavior: vec![],
                },
                Token {
                    value: String::from("2"),
                    category: None,
                    behavior: vec![],
                },
                Token {
                    value: String::from("%"),
                    category: Some(TokenCategory::Operator),
                    behavior: vec![],
                },
                Token {
                    value: String::from("3"),
                    category: None,
                    behavior: vec![],
                },
            ]
        );
    }

    #[test]
    fn test_get_sql_tokens_operator_add_equal() {
        assert_eq!(
            get_sql_tokens(String::from("V+=1")),
            vec![
                Token {
                    value: String::from("V"),
                    category: None,
                    behavior: vec![],
                },
                Token {
                    value: String::from("+="),
                    category: Some(TokenCategory::Operator),
                    behavior: vec![],
                },
                Token {
                    value: String::from("1"),
                    category: None,
                    behavior: vec![],
                },
            ]
        );
    }

    #[test]
    fn test_get_sql_tokens_operator_minus_equal() {
        assert_eq!(
            get_sql_tokens(String::from("V-=1")),
            vec![
                Token {
                    value: String::from("V"),
                    category: None,
                    behavior: vec![],
                },
                Token {
                    value: String::from("-="),
                    category: Some(TokenCategory::Operator),
                    behavior: vec![],
                },
                Token {
                    value: String::from("1"),
                    category: None,
                    behavior: vec![],
                },
            ]
        );
    }

    #[test]
    fn test_get_sql_tokens_operator_multiply_equal() {
        assert_eq!(
            get_sql_tokens(String::from("V*=1")),
            vec![
                Token {
                    value: String::from("V"),
                    category: None,
                    behavior: vec![],
                },
                Token {
                    value: String::from("*="),
                    category: Some(TokenCategory::Operator),
                    behavior: vec![],
                },
                Token {
                    value: String::from("1"),
                    category: None,
                    behavior: vec![],
                },
            ]
        );
    }

    #[test]
    fn test_get_sql_tokens_operator_divide_equal() {
        assert_eq!(
            get_sql_tokens(String::from("V/=1")),
            vec![
                Token {
                    value: String::from("V"),
                    category: None,
                    behavior: vec![],
                },
                Token {
                    value: String::from("/="),
                    category: Some(TokenCategory::Operator),
                    behavior: vec![],
                },
                Token {
                    value: String::from("1"),
                    category: None,
                    behavior: vec![],
                },
            ]
        );
    }

    #[test]
    fn test_get_sql_tokens_operator_modulo_equal() {
        assert_eq!(
            get_sql_tokens(String::from("V%=1")),
            vec![
                Token {
                    value: String::from("V"),
                    category: None,
                    behavior: vec![],
                },
                Token {
                    value: String::from("%="),
                    category: Some(TokenCategory::Operator),
                    behavior: vec![],
                },
                Token {
                    value: String::from("1"),
                    category: None,
                    behavior: vec![],
                },
            ]
        );
    }

    #[test]
    fn test_get_sql_tokens_bitwise_and() {
        assert_eq!(
            get_sql_tokens(String::from("V1&V2")),
            vec![
                Token {
                    value: String::from("V1"),
                    category: None,
                    behavior: vec![],
                },
                Token {
                    value: String::from("&"),
                    category: Some(TokenCategory::Bitwise),
                    behavior: vec![],
                },
                Token {
                    value: String::from("V2"),
                    category: None,
                    behavior: vec![],
                },
            ]
        );
    }

    #[test]
    fn test_get_sql_tokens_bitwise_or() {
        assert_eq!(
            get_sql_tokens(String::from("V1|V2")),
            vec![
                Token {
                    value: String::from("V1"),
                    category: None,
                    behavior: vec![],
                },
                Token {
                    value: String::from("|"),
                    category: Some(TokenCategory::Bitwise),
                    behavior: vec![],
                },
                Token {
                    value: String::from("V2"),
                    category: None,
                    behavior: vec![],
                },
            ]
        );
    }

    #[test]
    fn test_get_sql_tokens_bitwise_exclusive_or() {
        assert_eq!(
            get_sql_tokens(String::from("V1^V2")),
            vec![
                Token {
                    value: String::from("V1"),
                    category: None,
                    behavior: vec![],
                },
                Token {
                    value: String::from("^"),
                    category: Some(TokenCategory::Bitwise),
                    behavior: vec![],
                },
                Token {
                    value: String::from("V2"),
                    category: None,
                    behavior: vec![],
                },
            ]
        );
    }

    #[test]
    fn test_get_sql_tokens_paren_compare_lt() {
        assert_eq!(
            get_sql_tokens(String::from("WHERE C1<C2 AND C1 < C2")),
            vec![
                Token {
                    value: String::from("WHERE"),
                    category: Some(TokenCategory::Keyword),
                    behavior: vec![TokenBehavior::NewLineBefore],
                },
                Token {
                    value: String::from("C1"),
                    category: None,
                    behavior: vec![],
                },
                Token {
                    value: String::from("<"),
                    category: Some(TokenCategory::Compare),
                    behavior: vec![],
                },
                Token {
                    value: String::from("C2"),
                    category: None,
                    behavior: vec![],
                },
                Token {
                    value: String::from("AND"),
                    category: Some(TokenCategory::Keyword),
                    behavior: vec![TokenBehavior::NewLineBefore],
                },
                Token {
                    value: String::from("C1"),
                    category: None,
                    behavior: vec![],
                },
                Token {
                    value: String::from("<"),
                    category: Some(TokenCategory::Compare),
                    behavior: vec![],
                },
                Token {
                    value: String::from("C2"),
                    category: None,
                    behavior: vec![],
                },
            ]
        );
    }

    #[test]
    fn test_get_sql_tokens_paren_compare_gt() {
        assert_eq!(
            get_sql_tokens(String::from("WHERE C1>C2 AND C1 > C2")),
            vec![
                Token {
                    value: String::from("WHERE"),
                    category: Some(TokenCategory::Keyword),
                    behavior: vec![TokenBehavior::NewLineBefore],
                },
                Token {
                    value: String::from("C1"),
                    category: None,
                    behavior: vec![],
                },
                Token {
                    value: String::from(">"),
                    category: Some(TokenCategory::Compare),
                    behavior: vec![],
                },
                Token {
                    value: String::from("C2"),
                    category: None,
                    behavior: vec![],
                },
                Token {
                    value: String::from("AND"),
                    category: Some(TokenCategory::Keyword),
                    behavior: vec![TokenBehavior::NewLineBefore],
                },
                Token {
                    value: String::from("C1"),
                    category: None,
                    behavior: vec![],
                },
                Token {
                    value: String::from(">"),
                    category: Some(TokenCategory::Compare),
                    behavior: vec![],
                },
                Token {
                    value: String::from("C2"),
                    category: None,
                    behavior: vec![],
                },
            ]
        );
    }

    #[test]
    fn test_get_sql_tokens_paren_compare_eq() {
        assert_eq!(
            get_sql_tokens(String::from("WHERE C1=C2 AND C1 = C2")),
            vec![
                Token {
                    value: String::from("WHERE"),
                    category: Some(TokenCategory::Keyword),
                    behavior: vec![TokenBehavior::NewLineBefore],
                },
                Token {
                    value: String::from("C1"),
                    category: None,
                    behavior: vec![],
                },
                Token {
                    value: String::from("="),
                    category: Some(TokenCategory::Compare),
                    behavior: vec![],
                },
                Token {
                    value: String::from("C2"),
                    category: None,
                    behavior: vec![],
                },
                Token {
                    value: String::from("AND"),
                    category: Some(TokenCategory::Keyword),
                    behavior: vec![TokenBehavior::NewLineBefore],
                },
                Token {
                    value: String::from("C1"),
                    category: None,
                    behavior: vec![],
                },
                Token {
                    value: String::from("="),
                    category: Some(TokenCategory::Compare),
                    behavior: vec![],
                },
                Token {
                    value: String::from("C2"),
                    category: None,
                    behavior: vec![],
                },
            ]
        );
    }

    #[test]
    fn test_get_sql_tokens_paren_compare_neq() {
        assert_eq!(
            get_sql_tokens(String::from("WHERE C1<>C2 AND C1 <> C2")),
            vec![
                Token {
                    value: String::from("WHERE"),
                    category: Some(TokenCategory::Keyword),
                    behavior: vec![TokenBehavior::NewLineBefore],
                },
                Token {
                    value: String::from("C1"),
                    category: None,
                    behavior: vec![],
                },
                Token {
                    value: String::from("<>"),
                    category: Some(TokenCategory::Compare),
                    behavior: vec![],
                },
                Token {
                    value: String::from("C2"),
                    category: None,
                    behavior: vec![],
                },
                Token {
                    value: String::from("AND"),
                    category: Some(TokenCategory::Keyword),
                    behavior: vec![TokenBehavior::NewLineBefore],
                },
                Token {
                    value: String::from("C1"),
                    category: None,
                    behavior: vec![],
                },
                Token {
                    value: String::from("<>"),
                    category: Some(TokenCategory::Compare),
                    behavior: vec![],
                },
                Token {
                    value: String::from("C2"),
                    category: None,
                    behavior: vec![],
                },
            ]
        );
    }

    #[test]
    fn test_get_sql_tokens_paren_compare_gteq() {
        assert_eq!(
            get_sql_tokens(String::from("WHERE C1>=C2 AND C1 >= C2")),
            vec![
                Token {
                    value: String::from("WHERE"),
                    category: Some(TokenCategory::Keyword),
                    behavior: vec![TokenBehavior::NewLineBefore],
                },
                Token {
                    value: String::from("C1"),
                    category: None,
                    behavior: vec![],
                },
                Token {
                    value: String::from(">="),
                    category: Some(TokenCategory::Compare),
                    behavior: vec![],
                },
                Token {
                    value: String::from("C2"),
                    category: None,
                    behavior: vec![],
                },
                Token {
                    value: String::from("AND"),
                    category: Some(TokenCategory::Keyword),
                    behavior: vec![TokenBehavior::NewLineBefore],
                },
                Token {
                    value: String::from("C1"),
                    category: None,
                    behavior: vec![],
                },
                Token {
                    value: String::from(">="),
                    category: Some(TokenCategory::Compare),
                    behavior: vec![],
                },
                Token {
                    value: String::from("C2"),
                    category: None,
                    behavior: vec![],
                },
            ]
        );
    }

    #[test]
    fn test_get_sql_tokens_paren_compare_lteq() {
        assert_eq!(
            get_sql_tokens(String::from("WHERE C1<=C2 AND C1 <= C2")),
            vec![
                Token {
                    value: String::from("WHERE"),
                    category: Some(TokenCategory::Keyword),
                    behavior: vec![TokenBehavior::NewLineBefore],
                },
                Token {
                    value: String::from("C1"),
                    category: None,
                    behavior: vec![],
                },
                Token {
                    value: String::from("<="),
                    category: Some(TokenCategory::Compare),
                    behavior: vec![],
                },
                Token {
                    value: String::from("C2"),
                    category: None,
                    behavior: vec![],
                },
                Token {
                    value: String::from("AND"),
                    category: Some(TokenCategory::Keyword),
                    behavior: vec![TokenBehavior::NewLineBefore],
                },
                Token {
                    value: String::from("C1"),
                    category: None,
                    behavior: vec![],
                },
                Token {
                    value: String::from("<="),
                    category: Some(TokenCategory::Compare),
                    behavior: vec![],
                },
                Token {
                    value: String::from("C2"),
                    category: None,
                    behavior: vec![],
                },
            ]
        );
    }
}
