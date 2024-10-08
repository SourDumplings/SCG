use std::env;
use std::fs;
use std::path::Path;

fn main()
{
    // 获取输出目录
    let out_dir = env::var("OUT_DIR").unwrap();
    let target_dir = Path::new(&out_dir)
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .parent()
        .unwrap();

    // 定义源目录和目标目录
    let src_dir = Path::new("res");
    let dest_dir = target_dir.join("res");

    // 复制目录
    if dest_dir.exists()
    {
        fs::remove_dir_all(&dest_dir).unwrap();
    }
    fs::create_dir_all(&dest_dir).unwrap();
    copy_dir(src_dir, &dest_dir).unwrap();
}

fn copy_dir(src: &Path, dest: &Path) -> std::io::Result<()>
{
    for entry in fs::read_dir(src)?
    {
        let entry = entry?;
        let file_type = entry.file_type()?;
        if file_type.is_dir()
        {
            let new_dest = dest.join(entry.file_name());
            fs::create_dir_all(&new_dest)?;
            copy_dir(&entry.path(), &new_dest)?;
        }
        else
        {
            fs::copy(entry.path(), dest.join(entry.file_name()))?;
        }
    }
    Ok(())
}
