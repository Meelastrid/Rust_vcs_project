let SessionLoad = 1
let s:so_save = &g:so | let s:siso_save = &g:siso | setg so=0 siso=0 | setl so=-1 siso=-1
let v:this_session=expand("<sfile>:p")
silent only
silent tabonly
cd ~/code/sister/Rust_vcs_project
if expand('%') == '' && !&modified && line('$') <= 1 && getline(1) == ''
  let s:wipebuf = bufnr('%')
endif
let s:shortmess_save = &shortmess
if &shortmess =~ 'A'
  set shortmess=aoOA
else
  set shortmess=aoO
endif
badd +1 src/util.rs
badd +1 term://~/code/sister/Rust_vcs_project//581183:/home/vd/.local/share/nvim/plugged/vimspector/gadgets/linux/download/CodeLLDB/v1.7.4/root/extension/adapter/codelldb
badd +3 term://~/code/sister/Rust_vcs_project//582114:/home/vd/.local/share/nvim/plugged/vimspector/gadgets/linux/download/CodeLLDB/v1.7.4/root/extension/adapter/codelldb
badd +24 .vimspector.json
badd +1 term://~/code/sister/Rust_vcs_project//585236:/home/vd/.local/share/nvim/plugged/vimspector/gadgets/linux/download/CodeLLDB/v1.7.4/root/extension/adapter/codelldb
badd +1 term://~/code/sister/Rust_vcs_project//587685:/home/vd/.local/share/nvim/plugged/vimspector/gadgets/linux/download/CodeLLDB/v1.7.4/root/extension/adapter/codelldb
badd +1 term://~/code/sister/Rust_vcs_project//608519:/home/vd/.local/share/nvim/plugged/vimspector/gadgets/linux/download/CodeLLDB/v1.7.4/root/extension/adapter/codelldb
badd +1 term://~/code/sister/Rust_vcs_project//609016:/home/vd/.local/share/nvim/plugged/vimspector/gadgets/linux/download/CodeLLDB/v1.7.4/root/extension/adapter/codelldb
badd +1 term://~/code/sister/Rust_vcs_project//609509:/home/vd/.local/share/nvim/plugged/vimspector/gadgets/linux/download/CodeLLDB/v1.7.4/root/extension/adapter/codelldb
badd +2 term://~/code/sister/Rust_vcs_project//610089:/home/vd/.local/share/nvim/plugged/vimspector/gadgets/linux/download/CodeLLDB/v1.7.4/root/extension/adapter/codelldb
badd +1 term://~/code/sister/Rust_vcs_project//610582:/home/vd/.local/share/nvim/plugged/vimspector/gadgets/linux/download/CodeLLDB/v1.7.4/root/extension/adapter/codelldb
badd +1 term://~/code/sister/Rust_vcs_project//611670:/home/vd/.local/share/nvim/plugged/vimspector/gadgets/linux/download/CodeLLDB/v1.7.4/root/extension/adapter/codelldb
badd +1 term://~/code/sister/Rust_vcs_project//629960:/home/vd/.local/share/nvim/plugged/vimspector/gadgets/linux/download/CodeLLDB/v1.7.4/root/extension/adapter/codelldb
badd +1 term://~/code/sister/Rust_vcs_project//630431:/home/vd/.local/share/nvim/plugged/vimspector/gadgets/linux/download/CodeLLDB/v1.7.4/root/extension/adapter/codelldb
badd +1 term://~/code/sister/Rust_vcs_project//630986:/home/vd/.local/share/nvim/plugged/vimspector/gadgets/linux/download/CodeLLDB/v1.7.4/root/extension/adapter/codelldb
badd +1 term://~/code/sister/Rust_vcs_project//631587:/home/vd/.local/share/nvim/plugged/vimspector/gadgets/linux/download/CodeLLDB/v1.7.4/root/extension/adapter/codelldb
badd +17 src/main.rs
badd +18 src/init.rs
badd +1 src/main
badd +1 term://~/code/sister/Rust_vcs_project//652910:/home/vd/.local/share/nvim/plugged/vimspector/gadgets/linux/download/CodeLLDB/v1.7.4/root/extension/adapter/codelldb
badd +1 test/18/166fe33cdd58c771030eb982027783be08b394
badd +1 test/2f/e33745eab60f8fcbbac60ea0eaf2e2eefda5d9
badd +1 test/30/405d4560f06c3dbd457b5f2173c1179845b852
badd +1 test/4b/825dc642cb6eb9a060e54bf8d69288fbee4904
badd +101 src/commands.rs
badd +1 src/commit.rs
badd +1 src/jump_to_branch.rs
badd +1 src/jump_to_commit.rs
badd +1 src/log.rs
badd +1 src/merge.rs
badd +1 src/new_branch.rs
badd +1 src/status.rs
badd +1 src/test.txt
badd +1 term://~/code/sister/Rust_vcs_project//686732:/home/vd/.local/share/nvim/plugged/vimspector/gadgets/linux/download/CodeLLDB/v1.7.4/root/extension/adapter/codelldb
badd +1 term://~/code/sister/Rust_vcs_project//1121510:/home/vd/.local/share/nvim/plugged/vimspector/gadgets/linux/download/CodeLLDB/v1.7.4/root/extension/adapter/codelldb
badd +1 term://~/code/sister/Rust_vcs_project//1122094:/home/vd/.local/share/nvim/plugged/vimspector/gadgets/linux/download/CodeLLDB/v1.7.4/root/extension/adapter/codelldb
badd +1 term://~/code/sister/Rust_vcs_project//1122528:/home/vd/.local/share/nvim/plugged/vimspector/gadgets/linux/download/CodeLLDB/v1.7.4/root/extension/adapter/codelldb
badd +1 term://~/code/sister/Rust_vcs_project//1122881:/home/vd/.local/share/nvim/plugged/vimspector/gadgets/linux/download/CodeLLDB/v1.7.4/root/extension/adapter/codelldb
badd +1 term://~/code/sister/Rust_vcs_project//1125458:/home/vd/.local/share/nvim/plugged/vimspector/gadgets/linux/download/CodeLLDB/v1.7.4/root/extension/adapter/codelldb
badd +1 term://~/code/sister/Rust_vcs_project//1126008:/home/vd/.local/share/nvim/plugged/vimspector/gadgets/linux/download/CodeLLDB/v1.7.4/root/extension/adapter/codelldb
badd +2 test/.vcs/config
badd +1 test/.vcs/description
badd +1 test/.vcs/HEAD
badd +1 term://~/code/sister/Rust_vcs_project//1132269:/home/vd/.local/share/nvim/plugged/vimspector/gadgets/linux/download/CodeLLDB/v1.7.4/root/extension/adapter/codelldb
badd +1 term://~/code/sister/Rust_vcs_project//1133330:/home/vd/.local/share/nvim/plugged/vimspector/gadgets/linux/download/CodeLLDB/v1.7.4/root/extension/adapter/codelldb
badd +1 term://~/code/sister/Rust_vcs_project//1134003:/home/vd/.local/share/nvim/plugged/vimspector/gadgets/linux/download/CodeLLDB/v1.7.4/root/extension/adapter/codelldb
badd +1 term://~/code/sister/Rust_vcs_project//1144600:/home/vd/.local/share/nvim/plugged/vimspector/gadgets/linux/download/CodeLLDB/v1.7.4/root/extension/adapter/codelldb
badd +1 term://~/code/sister/Rust_vcs_project//1144887:/home/vd/.local/share/nvim/plugged/vimspector/gadgets/linux/download/CodeLLDB/v1.7.4/root/extension/adapter/codelldb
badd +1 term://~/code/sister/Rust_vcs_project//1146850:/home/vd/.local/share/nvim/plugged/vimspector/gadgets/linux/download/CodeLLDB/v1.7.4/root/extension/adapter/codelldb
badd +4 term://~/code/sister/Rust_vcs_project//1148732:/home/vd/.local/share/nvim/plugged/vimspector/gadgets/linux/download/CodeLLDB/v1.7.4/root/extension/adapter/codelldb
badd +1 term://~/code/sister/Rust_vcs_project//1157487:/home/vd/.local/share/nvim/plugged/vimspector/gadgets/linux/download/CodeLLDB/v1.7.4/root/extension/adapter/codelldb
argglobal
%argdel
edit src/util.rs
let s:save_splitbelow = &splitbelow
let s:save_splitright = &splitright
set splitbelow splitright
wincmd _ | wincmd |
vsplit
wincmd _ | wincmd |
vsplit
2wincmd h
wincmd w
wincmd w
let &splitbelow = s:save_splitbelow
let &splitright = s:save_splitright
wincmd t
let s:save_winminheight = &winminheight
let s:save_winminwidth = &winminwidth
set winminheight=0
set winheight=1
set winminwidth=0
set winwidth=1
wincmd =
argglobal
balt src/commands.rs
let s:l = 179 - ((15 * winheight(0) + 22) / 45)
if s:l < 1 | let s:l = 1 | endif
keepjumps exe s:l
normal! zt
keepjumps 179
normal! 0
wincmd w
argglobal
if bufexists(fnamemodify("src/init.rs", ":p")) | buffer src/init.rs | else | edit src/init.rs | endif
if &buftype ==# 'terminal'
  silent file src/init.rs
endif
balt src/util.rs
let s:l = 32 - ((31 * winheight(0) + 22) / 45)
if s:l < 1 | let s:l = 1 | endif
keepjumps exe s:l
normal! zt
keepjumps 32
normal! 074|
wincmd w
argglobal
if bufexists(fnamemodify("src/main.rs", ":p")) | buffer src/main.rs | else | edit src/main.rs | endif
if &buftype ==# 'terminal'
  silent file src/main.rs
endif
balt src/init.rs
let s:l = 19 - ((18 * winheight(0) + 22) / 45)
if s:l < 1 | let s:l = 1 | endif
keepjumps exe s:l
normal! zt
keepjumps 19
normal! 0
wincmd w
wincmd =
tabnext 1
if exists('s:wipebuf') && len(win_findbuf(s:wipebuf)) == 0 && getbufvar(s:wipebuf, '&buftype') isnot# 'terminal'
  silent exe 'bwipe ' . s:wipebuf
endif
unlet! s:wipebuf
set winheight=1 winwidth=20
let &shortmess = s:shortmess_save
let &winminheight = s:save_winminheight
let &winminwidth = s:save_winminwidth
let s:sx = expand("<sfile>:p:r")."x.vim"
if filereadable(s:sx)
  exe "source " . fnameescape(s:sx)
endif
let &g:so = s:so_save | let &g:siso = s:siso_save
set hlsearch
nohlsearch
doautoall SessionLoadPost
unlet SessionLoad
" vim: set ft=vim :
