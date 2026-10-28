// .speclink.yaml 頁（desktop-settings-mode design D8；spec「設定頁圖形化讀寫兩層設定」）：「檔案」列
// （等寬路徑，唯讀）與「AI 工具」卡——內建工具每個一列附開關，自訂工具描述子為不可編輯列。
// remote 專案沒有這一頁（tools 屬本機 checkout 概念）。
import {
  Button,
  ConfirmDialog,
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
  SettingsCard,
  SettingsPageHeader,
  SettingsRow,
  Switch,
  useI18n,
} from "@speclink/ui";

import { BUILTIN_TOOLS, builtinToolLabel } from "../../builtinTools";
import { PageAlerts } from "./parts";
import type { ProjectSettingsState } from "./useProjectSettings";

export function SpeclinkPage({ ps }: { ps: ProjectSettingsState }) {
  const { t } = useI18n();
  const specDir = ps.snap.specDir ?? "speclink";
  const standardMode = specDir === "openspec" || specDir === "speclink";
  const pending = ps.snap.specDirConversion?.pending === true;
  const hasConversion = ps.settings.kind === "local" && ps.settings.convertSpecDirToSpeclink !== undefined;
  const modeValue = standardMode ? specDir : "custom";
  return (
    <>
      <SettingsPageHeader
        title=".speclink.yaml"
        description={t("settings.speclinkPageDesc")}
        note={
          <span data-testid="file-note-speclink" className="font-mono">
            .speclink.yaml · tools
          </span>
        }
      />
      <PageAlerts ps={ps} parseError={ps.snap.app.parseError} />
      <SettingsCard title={t("settings.fileCard")}>
        <SettingsRow
          label={t("settings.fileRow")}
          control={
            <span data-testid="speclink-file-path" className="font-mono text-xs text-muted-foreground">
              .speclink.yaml
            </span>
          }
        />
        <SettingsRow
          label={t("settings.specDirLabel")}
          description={
            ps.toolsDirty
              ? t("settings.specDirSaveToolsFirst")
              : !standardMode
                ? t("settings.specDirCustomHelp")
                : specDir === "speclink" && !pending
                  ? t("settings.specDirIrreversible")
                  : undefined
          }
          control={
            standardMode && hasConversion ? (
              <Select
                value={modeValue}
                disabled={ps.appDisabled || ps.toolsDirty || ps.specDirBusy || specDir === "speclink" || pending}
                onValueChange={(value) => {
                  if (value === "speclink" && specDir === "openspec") ps.setSpecDirConfirmOpen(true);
                }}
              >
                <SelectTrigger id="spec-dir-select" data-testid="spec-dir-select" className="w-56">
                  <SelectValue />
                </SelectTrigger>
                <SelectContent>
                  <SelectItem value="openspec">{t("settings.specDirOpenSpec")}</SelectItem>
                  <SelectItem value="speclink">{t("settings.specDirSpeclink")}</SelectItem>
                </SelectContent>
              </Select>
            ) : (
              <span data-testid={standardMode ? "spec-dir-mode" : "spec-dir-custom-path"} className="font-mono text-xs text-muted-foreground">
                {standardMode ? (specDir === "openspec" ? t("settings.specDirOpenSpec") : t("settings.specDirSpeclink")) : specDir}
              </span>
            )
          }
        >
          {pending && (
            <div className="flex items-center gap-2 px-4 pb-3" role="status">
              <span className="mr-auto text-sm text-destructive">{ps.snap.specDirConversion?.message ?? t("settings.specDirPending")}</span>
              <Button type="button" size="sm" data-testid="retry-spec-dir-conversion" disabled={ps.specDirBusy || ps.appDisabled || ps.toolsDirty} onClick={() => void ps.convertSpecDir()}>
                {ps.specDirBusy ? t("settings.specDirConverting") : t("settings.specDirRetry")}
              </Button>
            </div>
          )}
          {ps.specDirMsg && <p className="px-4 pb-3 text-sm text-destructive" role="alert">{ps.specDirMsg}</p>}
        </SettingsRow>
      </SettingsCard>
      <div data-testid="tools-card">
        <SettingsCard title={t("settings.toolsLabel")}>
          {BUILTIN_TOOLS.map((tool) => (
            <SettingsRow
              key={builtinToolLabel(tool)}
              label={builtinToolLabel(tool)}
              htmlFor={`tool-${tool}`}
              control={
                <Switch
                  id={`tool-${tool}`}
                  checked={ps.tools.includes(tool)}
                  disabled={ps.appDisabled}
                  onCheckedChange={(on) => ps.toggleTool(tool, on)}
                />
              }
            />
          ))}
          {ps.snap.app.customTools.map((name) => (
            <SettingsRow
              key={name}
              label={name}
              description={t("settings.customToolsLabel")}
              control={<span className="text-xs text-muted-foreground">{t("settings.readOnly")}</span>}
            />
          ))}
          <div className="flex items-center justify-end gap-2 px-4 py-3">
            <span className="mr-auto text-xs text-muted-foreground">{ps.appMsg ?? t("settings.toolsHelp")}</span>
            <Button type="button" size="sm" data-testid="save-app" disabled={ps.appDisabled} onClick={() => void ps.saveApp()}>
              {t("settings.save")}
            </Button>
          </div>
        </SettingsCard>
      </div>
      <ConfirmDialog
        open={ps.specDirConfirmOpen}
        onOpenChange={ps.setSpecDirConfirmOpen}
        title={t("settings.specDirConfirmTitle")}
        description={t("settings.specDirConfirmDescription")}
        cancelLabel={t("common.cancel")}
        confirmLabel={t("settings.specDirConfirm")}
        busy={ps.specDirBusy}
        onConfirm={ps.convertSpecDir}
      />
    </>
  );
}
