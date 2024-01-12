{
  self,
  system,
}: {
  config,
  lib,
  ...
}: let
  cfg = config.programs.alia;
  inherit (lib) mkIf mkEnableOption mkOption mkMerge types;
in {
  options.programs.alia = {
    enable = mkEnableOption "Enable alia";

    package = mkOption {
      type = types.package;
      default = self.packages.${system}.default;
      description = "Package to use for alia (defaults to this flake's).";
    };

    enableHelix = mkOption {
      type = types.bool;
      default = true;
      description = "Add alia to Helix's configuration";
    };
  };

  config = mkIf (cfg.enable) (mkMerge [
    {
      home.packages = [cfg.package];
    }
    (mkIf (cfg.enableHelix) {
      programs.helix.languages = {
        language = [
          {
            name = "alia";
            scope = "source.alia";
            injection-regex = "alia";
            file-types = ["lia"];
            roots = ["make.lia"];
            auto-format = true;
            comment-token = ";";
            language-servers = ["alia-lsp"];
            indent = {
              tab-width = 2;
              unit = "  ";
            };
            diagnostic-severity = "Hint";
            grammar = "clojure";
            text-width = 100;
          }
        ];
        language-server.alia-lsp = {
          command = "alia";
          args = ["lsp"];
        };
      };
    })
  ]);
}
