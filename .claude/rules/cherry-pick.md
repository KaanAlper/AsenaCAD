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
   conflicts in favour of our behaviour; the renamed product strings (`CADCraft` → `AsenaCAD`)
   and app paths (`apps/cadcraft*` → `apps/asenacad*`) are the usual conflicts.
4. `cargo xtask ci`, then commit. Update "Upstream reviewed" in `plan/STATUS.md`.
5. Review upstream once a month at most; never block feature work on it.
