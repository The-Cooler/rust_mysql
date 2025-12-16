use tui::widgets::ListState;
use crate::database::DatabaseConnection;

pub enum Screen {
    Login,
    DatabaseList,
    TableList,
    TableData,
}

pub struct App {
    pub screen: Screen,
    
    // 登录相关
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub input_field: usize, // 0: host, 1: port, 2: username, 3: password, 4: connect
    pub input_cursor: usize,
    pub error_message: Option<String>, // 错误消息
    pub chunks_info: Option<Vec<(u16, u16)>>, // 存储每个输入框的Y坐标范围 (start_y, height)
    
    // 数据库连接
    pub connection: Option<DatabaseConnection>,
    
    // 数据库列表
    pub databases: Vec<String>,
    pub database_index: usize,
    pub database_state: ListState,
    
    // 表列表
    pub current_database: Option<String>,
    pub tables: Vec<String>,
    pub table_index: usize,
    pub table_state: ListState,
    
    // 表数据
    pub current_table: Option<String>,
    pub table_data: Vec<Vec<String>>, // 行数据，每行是一个Vec<String>
    pub columns: Vec<String>,
    pub data_index: usize,
    pub data_state: ListState,
}

impl App {
    pub fn new() -> Self {
        let mut app = App {
            screen: Screen::Login,
            host: String::new(),
            port: 3306,
            username: String::new(),
            password: String::new(),
            input_field: 0,
            input_cursor: 0,
            error_message: None,
            chunks_info: None,
            connection: None,
            databases: Vec::new(),
            database_index: 0,
            database_state: ListState::default(),
            current_database: None,
            tables: Vec::new(),
            table_index: 0,
            table_state: ListState::default(),
            current_table: None,
            table_data: Vec::new(),
            columns: Vec::new(),
            data_index: 0,
            data_state: ListState::default(),
        };
        
        // 尝试加载保存的配置
        if let Ok(config) = crate::config::load_config() {
            app.host = config.host;
            app.port = config.port;
            app.username = config.username;
            app.password = config.password;
        }
        
        app
    }
    
}

