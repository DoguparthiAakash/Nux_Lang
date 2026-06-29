import sys

def patch_compiler():
    path = r'E:\nux\Nux_Lang\nux\nux_oleg\nux_dist\src\compiler.rs'
    with open(path, 'r') as f:
        content = f.read()

    # The string we want to replace
    find_str = r'''                                  } else {
                                      let mut found = None;
                                      for (name, cinfo) in &self.classes {
                                          if cinfo.methods.contains_key(&member) { found = Some(name.clone()); }
                                      }
                                      if let Some(n) = found { n } else {
                                          return self.error(format!("Method '{}' not found", member));
                                      }
                                  }'''

    replace_str = r'''                                  } else {
                                      let mut found = None;
                                      for (name, _cinfo) in &self.classes {
                                          if self.functions.contains_key(&format!("{}_{}", name, member)) { found = Some(name.clone()); }
                                      }
                                      if let Some(n) = found { n } else {
                                          return self.error(format!("Method '{}' not found", member));
                                      }
                                  }'''

    if find_str in content:
        content = content.replace(find_str, replace_str)
        print("Patched methods lookup")
    else:
        print("Could not find methods lookup")

    with open(path, 'w') as f:
        f.write(content)

if __name__ == '__main__':
    patch_compiler()
