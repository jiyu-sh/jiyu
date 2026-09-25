{
  lib,
  rustPlatform,
  makeWrapper,
  versionCheckHook,
  xray,
  sing-box,
}:
let
  version = "0.1.0";

  binPath = lib.makeBinPath [
    xray
    sing-box
  ];
in

rustPlatform.buildRustPackage {
  pname = "jiyu";

  inherit version;

  src = ./.;

  cargoLock = {
    lockFile = ./Cargo.lock;
  };

  nativeBuildInputs = [ makeWrapper ];

  nativeInstallCheckInputs = [ versionCheckHook ];
  doInstallCheck = true;

  postInstall = ''
    wrapProgram $out/bin/jiyu --prefix PATH : ${binPath}
  '';

  meta = {
    description = "Freedom of ...";
    homepage = "https://jiyu.sh/";
    changelog = "https://releases.jiyu.sh/v${version}";
    platforms = lib.platforms.all;
    license = lib.licenses.mit;
    maintainers = [ lib.maintainers.nekitdev ];
    mainProgram = "jiyu";
  };
}
