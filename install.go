package main

import (
	_ "embed"
	"encoding/json"
	"errors"
	"fmt"
	"log"
	"os"
	"os/exec"
	"path/filepath"
	"strings"
	"sync"
)

//go:embed pacman_cuda.json
var pacman_cuda []byte

const TMP_DIR = "/tmp/cuda_installer"
const (
	recent = iota
	compatible
)

type AurPkg struct {
	Version float32 `json:"version"`
	Name    string  `json:"name"`
	Link    string  `json:"link"`
}

type Installer struct {
	Support []string `json:"support"`
	GCC     AurPkg   `json:"gcc"`
	GCCLibs AurPkg   `json:"gcc-libs"`
	Cuda    AurPkg   `json:"cuda"`
	Cudnn   AurPkg   `json:"cudnn"`
}

type JSON struct {
	Deps       string    `json:"deps"`
	Recent     Installer `json:"recent"`
	Compatible Installer `json:"compatible"`
}

func main() {
	var err error

	current := &JSON{}

	err = current.load_json()
	if err != nil {
		log.Fatal(err)
	}

	err = current.deps()
	if err != nil {
		log.Fatal(err)
	}

	installer := &current.Recent
	if current.get_choice(0) == compatible {
		installer = &current.Compatible
	}

	err = installer.install()
	if err != nil {
		log.Fatal(err)
	}

	goodbye()
}

func (j *JSON) load_json() error {
	err := json.Unmarshal(pacman_cuda, &j)
	if err != nil {
		return err
	}

	return nil
}

func (j *JSON) deps() error {
	fmt.Println("Updating system dependencies...")

	return run_cmd("sudo", "pacman", "-Syu", j.Deps)
}

func (j *JSON) get_choice(tries int) int {
	fmt.Printf("Current cuda support choices:\n\tRecent: %v\n\tCompatible: %v\nSelect your Python Version, (0 | Default) Recent, (1) Compatible: ", j.Recent.Support, j.Compatible.Support)
	if tries > 4 {
		fmt.Println("\nToo many tries. Exiting...")
		os.Exit(1)
	}

	version := 0
	fmt.Scanln(&version)
	if version != 0 && version != 1 {
		fmt.Println("Invalid Input. Please try again.")
		version = j.get_choice(tries + 1)
	}

	return version
}

func (i *Installer) install() error {
	fmt.Println("Installing ...")
	var err error

	err = mkdir_p(TMP_DIR)
	if err != nil {
		return err
	}
	defer rmdir_rf(TMP_DIR)

	err = i.download()
	if err != nil {
		return err
	}

	var installs = make([]error, 2)
	installs[0] = run_cmd("pacman", "-U", filepath.Join(TMP_DIR, i.GCC.Name), filepath.Join(TMP_DIR, i.GCCLibs.Name))
	installs[1] = run_cmd("pacman", "-U", filepath.Join(TMP_DIR, i.Cuda.Name), filepath.Join(TMP_DIR, i.Cudnn.Name))

	for _, err := range installs {
		if err != nil {
			return err
		}
	}

	return nil
}

func (i *Installer) download() error {
	pkgs := []*AurPkg{&i.GCC, &i.GCCLibs, &i.Cuda, &i.Cudnn}
	errCh := make(chan error, len(pkgs))

	var wg sync.WaitGroup

	worker := func(ap *AurPkg, wg *sync.WaitGroup) {
		defer wg.Done()
		err := ap.download()

		errCh <- err
	}

	for _, pkg := range pkgs {
		wg.Add(1)
		go worker(pkg, &wg)
	}

	go func() {
		wg.Wait()
		close(errCh)
	}()

	for err := range errCh {
		if err != nil {
			return err
		}
	}

	return nil
}

func (ap *AurPkg) download() error {
	err, installed := ap.check()
	if err != nil {
		return fmt.Errorf("failed to check if %s is installed: %w", ap.Name, err)
	} else if installed {
		return nil
	}

	fmt.Printf("Downloading %v...\n", ap.Name)
	return ap.curl_i_o()
}

func (ap *AurPkg) check() (error, bool) {
	pkg_name := strings.Split(ap.Name, "-")[0]
	fmt.Printf("Checking if %s is installed...\n", pkg_name)

	output, err := get_cmd("pacman", "-Q", pkg_name)
	if err != nil {
		return nil, false
	}

	err = contains(output, fmt.Sprintf("%s %.1f", pkg_name, ap.Version))
	if err == nil { // heh
		fmt.Printf("%s %.1f already installed\n", pkg_name, ap.Version)
		return nil, true
	}

	return nil, false
}

func (ap *AurPkg) curl_i_o() error {
	output, err := get_cmd("curl", "-I", ap.Link)
	if err != nil {
		return err
	}

	err = contains(output, "HTTP/2 200")
	if err != nil {
		return err
	}

	return run_cmd("curl", "-o", filepath.Join(TMP_DIR, ap.Name), ap.Link)
}

func run_cmd(name string, arg ...string) error {
	fs := name + " " + strings.Join(arg, " ")
	fmt.Printf("Running %s\n", fs)

	cmd := exec.Command("bash", "-c", fs)
	cmd.Stdin = os.Stdin
	cmd.Stdout = os.Stdout
	cmd.Stderr = os.Stderr
	err := cmd.Run()
	if err != nil {
		fmt.Println("Error running command:", err)
	}

	return nil
}

func get_cmd(name string, arg ...string) (string, error) {
	cmd := exec.Command(name, arg...)
	if errors.Is(cmd.Err, exec.ErrDot) {
		cmd.Err = nil
	}

	output, err := cmd.CombinedOutput()
	if err != nil {
		return "", err
	}

	return string(output), nil
}

func contains(source, target string) error {
	fmt.Println("Checking for", target)
	length := len(target)
	if length > len(source) {
		return fmt.Errorf("no output from curl")
	}

	for i := 0; i <= len(source)-length; i++ {
		if source[i:i+length] == target {
			fmt.Printf("Found %s\n", target)
			return nil
		}
	}

	return fmt.Errorf("no output from curl")
}

func mkdir_p(dir_path string) error {
	fmt.Printf("Creating %s...\n", dir_path)

	_, err := os.Stat(dir_path)
	if err == nil {
		return nil
	}
	if os.IsNotExist(err) {
		err = mkdir_p(filepath.Dir(dir_path))
		if err != nil {
			return err
		}
		err = os.Mkdir(dir_path, 0777)
		if err != nil {
			return err
		}
	}
	return nil
}

func rmdir_rf(dir_path string) error {
	fmt.Printf("Removing %s...\n", dir_path)

	_, err := os.Stat(dir_path)
	if err != nil {
		return err
	}
	err = os.RemoveAll(dir_path)
	if err != nil {
		return err
	}
	return nil
}

func goodbye() {
	fmt.Println("Installation Complete!")

	fmt.Println("\nTo prevent the system from updating the packages, add the following to /etc/pacman.conf")
	fmt.Println("\tIgnorePkg = cuda cudnn gcc12 gcc12-libs")

	fmt.Println("\nCommon Error: # ERROR: libdevice not found at ./libdevice.10.bc")
	fmt.Println("To fix this error, run the following command:\n\texport XLA_FLAGS=--xla_gpu_cuda_data_dir=/opt/cuda")
}
