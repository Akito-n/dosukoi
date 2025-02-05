# DOSUKOI

```
██████╗  ██████╗ ███████╗██╗   ██╗██╗  ██╗ ██████╗  ██████╗
██╔══██╗██╔═══██╗██╔════╝██║   ██║██║ ██╔╝██╔═══██╗  ╚██╔╝
██║  ██║██║   ██║███████╗██║   ██║█████╔╝ ██║   ██║   ██║  
██║  ██║██║   ██║╚════██║██║   ██║██╔═██╗ ██║   ██║   ██║  
██████╔╝╚██████╔╝███████║╚██████╔╝██║  ██╗╚███ ███╔╝██████║  
╚═════╝  ╚═════╝ ╚══════╝ ╚═════╝ ╚═╝  ╚═╝ ╚═══╝╚═╝   ╚═╝  

   (╯°□°）╯︵ ┻━┻  
```

## 🚀 About DOSUKOI
DOSUKOI is a simple CLI tool to stop Docker containers efficiently.  
You can:
- Stop **all** running containers at once.
- Stop **a specific Docker Compose project** using an argument.
- Use `-k` option to **kill** containers instead of stopping them.
- Check the installed version with `--version`.

---

## 🛠 Installation
You can install DOSUKOI using Homebrew:

```sh
brew tap Akito-n/dosukoi
brew install dosukoi
```

---

## 🎯 Usage

### **🛑 Stop all running Docker containers**
```sh
dosukoi
```

### **🔍 Stop a specific Docker Compose project**
```sh
dosukoi my_project
```
This stops containers inside the `my_project` Docker Compose group.

### **💀 Kill containers instead of stopping them**
```sh
dosukoi -k
```
```sh
dosukoi -k my_project
```
This will run `docker kill` instead of `docker stop`.

### **📌 Check DOSUKOI version**
```sh
dosukoi --version
```
or
```sh
dosukoi -v
```

---

## 🧹 Uninstallation
If you want to remove DOSUKOI:
```sh
brew uninstall dosukoi
```

---

## 📜 License
MIT License

---

## 🤝 Contributing
Feel free to open **issues** or **pull requests** to improve DOSUKOI!

---

🚀 **Stop your containers with style! DOSUKOI!**


