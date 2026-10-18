param([int]$MaxDepth = 12)
# UI Automation tree of AutoCAD's main window, one element per line, indented by depth:
#   ControlType<TAB>Name<TAB>AutomationId<TAB>ClassName<TAB>x,y,w,h
# Reads only structure (type, name, id, class, rectangle); never HelpText or other help prose.
# Run on the reference PC through AsenaScale (see the autocad-observe skill), e.g.
#   asenascale exec "$ASENACAD_REF" -- "& { $(cat tools/autocad/uia-dump.ps1) } -MaxDepth 8"
# The output is an AutoCAD observation: keep it under plan/autocad/uia/, never commit it.
[Console]::OutputEncoding = [Text.Encoding]::UTF8
Add-Type -AssemblyName UIAutomationClient, UIAutomationTypes
$p = Get-Process acad -ErrorAction Stop | Where-Object { $_.MainWindowHandle -ne 0 } | Select-Object -First 1
$root = [System.Windows.Automation.AutomationElement]::FromHandle($p.MainWindowHandle)
$walker = [System.Windows.Automation.TreeWalker]::ControlViewWalker
function Dump($el, [int]$depth, [int]$max) {
  if ($depth -gt $max) { return }
  try {
    $c = $el.Current; $r = $c.BoundingRectangle
    $t = $c.ControlType.ProgrammaticName -replace '^ControlType\.',''
    $rect = if ($r.IsEmpty) { '-' } else { '{0},{1},{2},{3}' -f [int]$r.X,[int]$r.Y,[int]$r.Width,[int]$r.Height }
    "{0}{1}`t{2}`t{3}`t{4}`t{5}" -f ('  ' * $depth), $t, $c.Name, $c.AutomationId, $c.ClassName, $rect
  } catch { "{0}<error>" -f ('  ' * $depth); return }
  $ch = $walker.GetFirstChild($el)
  while ($ch -ne $null) { Dump $ch ($depth + 1) $max; $ch = $walker.GetNextSibling($ch) }
}
Dump $root 0 $MaxDepth
