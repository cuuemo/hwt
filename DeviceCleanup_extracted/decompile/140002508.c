/*
 * func-name: TimerFunc
 * func-address: 0x140002508
 * callers: 0x14000453c
 * callees: 0x14000246c
 */

void __fastcall TimerFunc(HWND a1, __int64 a2, UINT_PTR a3)
{
  sub_14000246C();
  KillTimer(a1, a3);
}
