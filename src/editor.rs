

use tempfile::NamedTempFile;

use std::io::{Error,Write,Read,ErrorKind};
use std::fs;
use std::process::Command;


/// Struct for calling an extern editor like vim,vi or notepad
///
/// @author: d.stu
#[derive(Ord, PartialOrd, Eq, PartialEq,Clone,Debug,Hash)]
pub struct Editor {
    editor: EditorType,
}

impl Editor{

    /// creates a new Editor with default values
    /// Notepad for windows and Vi for Unix
    pub fn new() -> Editor {
        let editor_type = if cfg!(windows){EditorType::Notepad} else{EditorType::Vi};
        Editor{editor: editor_type}
    }

    /// choose the editor
    pub fn with_type(editor: EditorType) -> Editor{
        Editor{editor}
    }

    /// Open the editor to edit the given string, returns non if txt was not edited
    pub fn open_with_text(&self, txt: &str) -> Result<Option<String>,Error> {

        let mut tmp_file = NamedTempFile::new()?;
        tmp_file.write(txt.as_bytes())?;
        tmp_file.flush()?;
        let timestamp = fs::metadata(tmp_file.path())?.modified()?;

        let mut editor_process = Command::new(self.editor.program());
        editor_process.arg(tmp_file.path());
        let status = editor_process.spawn()?.wait()?;

        if !status.success(){
            let msg = format!("The Editor was not successfully closed. Bad return Code: {:#?}", status.code());
            return Err(Error::new(ErrorKind::InvalidData, msg));
        }

        let file_metadata = fs::metadata(tmp_file.path())?;
        let timestamp_after = file_metadata.modified()?;
        let edited = timestamp.lt(&timestamp_after); //check if modified after first write

        if !edited {
            return Ok(None);
        }

        let mut s = String::with_capacity(file_metadata.len() as usize); //allocate enough memory for the file
        let mut tf = tmp_file.reopen()?; //need to reopen to begin reading at beginning of file
        tmp_file.close();
        tf.read_to_string(&mut s)?;

        Ok(Some(s))
    }
}

/// enum of all supported editors
///
/// @author: d.stu
#[derive(Ord, PartialOrd, Eq, PartialEq,Clone,Copy,Debug,Hash)]
pub enum EditorType{
    Vim,
    Vi,
    Nano,
    Notepad,
}

impl EditorType{

    pub fn program(&self) -> &str {

        match self {
            &EditorType::Vi => {"vi"},
            &EditorType::Vim => {"vim"},
            &EditorType::Nano => {"nano"},
            &EditorType::Notepad => {"notepad"},
        }

    }

}