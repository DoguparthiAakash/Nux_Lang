# Homebrew Formula for Nux Language
# Install: brew install --formula https://raw.githubusercontent.com/DoguparthiAakash/Nux_Lang/main/nux.rb
# Or from a tap: brew tap DoguparthiAakash/nux && brew install nux

class Nux < Formula
  desc "Universal, cross-hardware systems programming language"
  homepage "https://github.com/DoguparthiAakash/Nux_Lang"
  url "https://github.com/DoguparthiAakash/Nux_Lang/archive/refs/tags/v0.4.0.tar.gz"
  sha256 "PLACEHOLDER_SHA256"  # Update after first release tag is created
  license "MIT"
  head "https://github.com/DoguparthiAakash/Nux_Lang.git", branch: "main"

  # Rust is needed to compile Nux
  depends_on "rust" => :build

  def install
    # Navigate to the portable compiler source
    cd "nux/nux_oleg/nux_portable" do
      system "cargo", "build", "--release", "--locked"
      bin.install "target/release/nux"
    end
  end

  def caveats
    <<~EOS
      Nux has been installed to:
        #{HOMEBREW_PREFIX}/bin/nux

      Quick start:
        nux version                  # Show version
        nux init my_project          # Create a new project
        nux run my_project/main.nux  # Run a Nux source file

      Documentation:
        #{homepage}
    EOS
  end

  test do
    # Create a minimal Nux source and compile it
    (testpath/"test.nux").write <<~NUX
      func main() {
        print(42);
      }
      main();
    NUX
    output = shell_output("#{bin}/nux build test.nux 2>&1")
    assert_match "Finished", output
  end
end
