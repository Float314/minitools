/* Clipboard settings */

use arboard::Clipboard;

use crate::clipboard;

pub fn copy_to_clipboard(content: String) 
    -> Result<(), Box<dyn std::error::Error>>
{
    // initialise clipboard
    let mut clipboard = Clipboard::new().unwrap();

    clipboard.set_text(content)?;
    Ok(())
}

pub fn clear_clipboard() 
    -> Result<(), Box<dyn std::error::Error>>
{
    let mut clipboard = Clipboard::new().unwrap();

    clipboard.clear();
    Ok(())
}