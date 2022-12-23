package main

import (
	"flag"
	"fmt"
	"os"
	"path/filepath"

	"github.com/mbndr/figlet4go"
)

func main() {
	renderASCII()

	ls := flag.String("ls", ".", "List directory contents")
	mkdir := flag.String("mkdir", "", "Create a directory")
	touch := flag.String("touch", "", "Create a file")

	flag.Parse()

	if *ls != "" {
		listDirContents(*ls)
	}
	if *mkdir != "" {
		createDir(filepath.Join("./", *mkdir))
	}
	if *touch != "" {
		createFile(filepath.Join("./", *touch))
	}

	if flag.NFlag() == 0 {
		printUsage()
	}
}

func renderASCII() {
	ascii := figlet4go.NewAsciiRender()

	// The underscore would be an error
	renderStr, _ := ascii.Render("Dir Manager")
	fmt.Print(renderStr)
}

func printUsage() {
	var CommandLine = flag.NewFlagSet(os.Args[0], flag.ExitOnError)
	fmt.Fprintf(CommandLine.Output(), "Usage of %s:\n", filepath.Base(os.Args[0]))
	flag.PrintDefaults()
}

func listDirContents(filepath string) {
	// Open the directory using os.Open
	dir, err := os.Open(filepath)
	if err != nil {
		fmt.Println(err)
		return
	}
	defer dir.Close()

	// Read the contents of the directory
	files, err := dir.Readdir(-1)
	if err != nil {
		fmt.Println("Error occurred while reading the directory:", err)
		return
	}

	type DetailedFile struct {
		Filename string
		Size     int64
		Created  string
	}

	var detailedFiles []DetailedFile
	for _, file := range files {
		detailedFiles = append(detailedFiles, DetailedFile{
			Filename: file.Name(),
			Size:     file.Size(),
			Created:  file.ModTime().String(),
		})
	}

	fmt.Println("Directory contents:")
	for _, file := range detailedFiles {
		fmt.Printf("%s\t%d\t%s\n", file.Filename, file.Size, file.Created)
	}
}

func createDir(filepath string) {
	if _, err := os.Stat(filepath); os.IsNotExist(err) {
		err := os.Mkdir(filepath, 0777)
		if err != nil {
			fmt.Println("Error occurred while creating the directory:", err)
			return
		}
		fmt.Println("The directory has been created successfully")
	}
}

func createFile(filepath string) {
	_, err := os.Create(filepath)
	if err != nil {
		fmt.Println("Error occurred while creating the file:", err)
		return
	}
	fmt.Println("An empty file has been created")
}
