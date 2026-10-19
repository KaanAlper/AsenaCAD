# Taking commits from upstream CADCraft

We forked `storytold/cadcraft` and diverge; there are no merges. A useful upstream fix comes in
as a cherry-pick.

0. Once per clone, make pushing to upstream impossible:
   `git remote set-url --push upstream no-push`.
1. `git fetch upstream` and list what is new: `git log --oneline <last-reviewed>..upstream/main`
   (the last reviewed upstream commit is recorded in `plan/STATUS.md` under "Upstream reviewed").
2. Take only commits that fix bugs or crashes, or add AutoCAD behaviour we have not built
   differently. Skip Mac-UI work that conflicts with our Windows ribbon layout.
3. `git cherry-pick -x <sha>` (keeps "cherry picked from commit …" in the message). Resolve
   conflicts in favour of our behaviour; the renamed product strings (`CADCraft` → `AsenaCAD`),
   app paths, crate names, environment variables and drawing names are the usual conflicts.
   Upstream's crate and path prefix is its project name in lower case, its environment
   variables and drawing names use it in upper case; rename them to `asenacad`/`ASENACAD` while
   resolving, e.g. on the files the pick touched:
   ```sh
   up=$(echo CADCraft | tr A-Z a-z); UP=$(echo CADCraft | tr a-z A-Z)
   git diff --name-only HEAD | xargs sed -i -e "s/${up}\([-_]\)/asenacad\1/g" -e "s/${UP}/ASENACAD/g"
   ```
   AsenaCAD reads only its own `ASENACAD` names; keep `storytold/cadcraft` URLs.
4. `cargo xtask ci`, then commit. Update "Upstream reviewed" in `plan/STATUS.md`.
5. Review upstream once a month at most; never block feature work on it.
