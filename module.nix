{
  config,
  lib,
  pkgs,
  ...
}:

let
  cfg = config.services.jiyu;

  jiyu = lib.getExe' cfg.package "jiyu";

  xray = lib.getExe' cfg.xray.package "xray";

  sing-box = lib.getExe' cfg.sing-box.package "sing-box";

  environment = {
    JIYU_XRAY = xray;
    JIYU_SING_BOX = sing-box;
  };

  format = pkgs.formats.toml { };

  jiyuToml = format.generate "jiyu.toml" cfg.settings;

  serverHost = cfg.server.host;
  serverPort = toString cfg.server.port;

  isJiyu = string: string == "jiyu";

  mkPackageOption =
    name:
    lib.mkPackageOption pkgs name {
      extraDescription = "The `${name}` package to use.";
    };

  empty = "";

  hardening = {
    # it is strict sandbox time indeed
    NoNewPrivileges = true;
    ProtectSystem = "strict";
    ProtectHome = true;
    PrivateTmp = true;
    ProtectKernelTunables = true;
    ProtectKernelModules = true;
    ProtectKernelLogs = true;
    ProtectControlGroups = true;
    ProtectClock = true;
    ProtectHostname = true;
    LockPersonality = true;
    RestrictRealtime = true;
    RestrictSUIDSGID = true;
    RestrictNamespaces = true;
    SystemCallArchitectures = "native";

    # owner has full access, `jiyu` group can read, others have nothing
    UMask = "0027";
  };

in

{
  options.services.jiyu = {
    enable = lib.mkEnableOption "jiyu, freedom of managing proxies";

    package = mkPackageOption "jiyu";

    xray.package = mkPackageOption "xray";

    sing-box.package = mkPackageOption "sing-box";

    user = lib.mkOption {
      type = lib.types.str;
      default = "jiyu";
      description = "The user to run `jiyu` as.";
    };

    group = lib.mkOption {
      type = lib.types.str;
      default = "jiyu";
      description = "The group to run `jiyu` as.";
    };

    web = {
      enable = lib.mkEnableOption "the `jiyu` web UI";

      host = lib.mkOption {
        type = lib.types.str;
        default = "127.0.0.1";
        description = "The host for the web UI to listen on.";
      };

      port = lib.mkOption {
        type = lib.types.port;
        default = 10900;
        description = "The port for the web UI to listen on.";
      };
    };

    server = {
      # TODO: set `default` to `true` later
      enable = lib.mkEnableOption "the `jiyu` API server";

      host = lib.mkOption {
        type = lib.types.str;
        default = "127.0.0.1";
        description = "The host for the API server to listen on.";
      };

      port = lib.mkOption {
        type = lib.types.port;
        default = 10901;
        description = "The port for the API server to listen on.";
      };

      openFirewall = lib.mkOption {
        type = lib.types.bool;
        default = false;
        description = "Whether to open the API port in the firewall.";
      };
    };

    settings = lib.mkOption {
      type = format.type;
      default = {
        # TODO: expand
      };
      example = {
        # TODO: expand
      };
      description = ''
        Configuration of `jiyu` in TOML format, written to `jiyu.toml`
        and passed via `--config`
      '';
    };
  };

  config = lib.mkIf cfg.enable {
    # add `jiyu` to system packages if enabled
    environment.systemPackages = [ cfg.package ];

    # add `jiyu` user and group
    users = {
      users = lib.mkIf (isJiyu cfg.user) {
        jiyu = {
          isSystemUser = true;
          group = cfg.group;
          description = "jiyu services user";
        };
      };

      groups = lib.mkIf (isJiyu cfg.group) {
        jiyu = { };
      };
    };

    # open `jiyu` API port if requested
    networking.firewall.allowedTCPPorts = lib.mkIf cfg.server.openFirewall [ cfg.server.port ];

    systemd.services = {
      jiyu-daemon = {
        description = "jiyu daemon";
        documentation = [ "https://docs.jiyu.sh/" ];

        wantedBy = [ "multi-user.target" ];

        wants = [ "network-online.target" ];
        after = [ "network-online.target" ];

        inherit environment;

        restartTriggers = [ jiyuToml ];

        serviceConfig = hardening // {
          ExecStart = "${jiyu} daemon run --config ${jiyuToml}";

          User = cfg.user;
          Group = cfg.group;

          # `/var/lib/jiyu` (database and others)
          StateDirectory = "jiyu";
          StateDirectoryMode = "0750";

          # `/run/jiyu` (unix socket `jiyu.sock` for communication)
          RuntimeDirectory = "jiyu";
          RuntimeDirectoryMode = "0750";

          # cores such as `xray` and `sing-box` inherit these on spawn
          # (`tun` devices, routes, privileged ports and such)
          AmbientCapabilities = [
            "CAP_NET_ADMIN"
            "CAP_NET_BIND_SERVICE"
          ];
          CapabilityBoundingSet = [
            "CAP_NET_ADMIN"
            "CAP_NET_BIND_SERVICE"
          ];

          # `PrivateDevices` would hide `/dev/net/tun`, therefore use allow-listing here
          DevicePolicy = "closed";
          DeviceAllow = [ "/dev/net/tun rw" ];

          # pretty self-explanatory
          RestrictAddressFamilies = [
            "AF_INET"
            "AF_INET6"
            "AF_UNIX"
            "AF_NETLINK"
          ];

          Restart = "on-failure";
          RestartSec = 5;
        };
      };
      jiyu-server = lib.mkIf cfg.server.enable {
        description = "jiyu server";
        documentation = [ "https://docs.jiyu.sh/" ];

        wantedBy = [ "multi-user.target" ];

        wants = [ "network-online.target" ];
        after = [ "network-online.target" ];

        inherit environment;

        restartTriggers = [ jiyuToml ];

        serviceConfig = hardening // {
          ExecStart = ''
            ${jiyu} server run --config ${jiyuToml} --host ${serverHost} --port ${serverPort}
          '';

          User = cfg.user;
          Group = cfg.group;

          # `/var/lib/jiyu` (database and others)
          StateDirectory = "jiyu";
          StateDirectoryMode = "0750";

          CapabilityBoundingSet = [ empty ];
          PrivateDevices = true;
          MemoryDenyWriteExecute = true;

          RestrictAddressFamilies = [
            "AF_INET"
            "AF_INET6"
            "AF_UNIX"
          ];

          Restart = "on-failure";
          RestartSec = 5;
        };
      };
    };
  };
}
