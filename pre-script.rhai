let project_name = variable::get("project-name");
let class_name = to_upper_camel_case(project_name);
let file_name = project_name + ".rs";
debug(`project_name: ${project_name}`);
debug(`class_name: ${class_name}`);
debug(`file_name: ${file_name}`);
variable::set("class_name", class_name);

file::rename("src/lib.rs", "src/"+file_name);