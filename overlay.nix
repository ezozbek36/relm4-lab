final: prev: {
  # ofc, latest gtk4 not exist latest nixpkgs
  gtk4 = prev.gtk4.overrideAttrs (old: rec {
    version = "4.22.1";
    src = prev.fetchurl {
      url = "mirror://gnome/sources/gtk/${prev.lib.versions.majorMinor version}/gtk-${version}.tar.xz";
      hash = "sha256-zXtanEESfab7MhxahCrXPGLmmW+c4/GWDKUJ9lWdVfw=";
    };

    nativeBuildInputs = (old.nativeBuildInputs or []) ++ [prev.shared-mime-info];
  });

  # latest libadwaita not exist in nixpkgs
  libadwaita = prev.libadwaita.overrideAttrs (old: rec {
    version = "1.9.0";
    src = prev.fetchFromGitLab {
      domain = "gitlab.gnome.org";
      owner = "GNOME";
      repo = "libadwaita";
      rev = version;
      hash = "sha256-JAKP8CjLCKGZvHoB26ih/J3xAru4wiVf/ObG0L8r4pY=";
    };
  });
}
