{
  # libraries
  craneLib,
  lib,
  # metadata
  name,
  version,
  binary,
  # packages
  openssl,
  pkg-config,
  sing-box,
  xray,
  # artifacts
  cargoArtifacts,
  # hooks
  makeWrapper,
  versionCheckHook,
}:
let
  deps = [
    openssl
    pkg-config
  ];

  binPath = lib.makeBinPath [
    xray
    sing-box
  ];

  src = lib.cleanSource ./.;

  extraArgs = "--locked --package ${name} --bin ${binary}";

in

craneLib.buildPackage {
  pname = name;

  inherit version src;

  strictDeps = true;

  buildInputs = deps;

  inherit cargoArtifacts;

  cargoExtraArgs = extraArgs;

  nativeBuildInputs = [ makeWrapper ];

  nativeInstallCheckInputs = [ versionCheckHook ];
  doInstallCheck = true;

  postInstall = ''
    wrapProgram $out/bin/${binary} --prefix PATH : ${binPath}
  '';

  meta = {
    description = "Freedom of managing proxies.";
    homepage = "https://jiyu.sh/";
    changelog = "https://releases.jiyu.sh/v${version}";
    platforms = lib.platforms.all;
    license = lib.licenses.mit;
    maintainers = [ lib.maintainers.nekitdev ];
    mainProgram = binary;
  };
}
