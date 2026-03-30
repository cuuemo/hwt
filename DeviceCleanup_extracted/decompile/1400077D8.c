/*
 * func-name: sub_1400077D8
 * func-address: 0x1400077d8
 * callers: 0x1400012fc
 * callees: 0x14000179c, 0x140007700
 */

__int64 __fastcall sub_1400077D8(unsigned int a1, unsigned __int64 *a2)
{
  HMODULE ModuleHandleA; // rax
  CONFIGRET (__stdcall *CM_Get_DevNode_PropertyW)(DEVINST, const DEVPROPKEY *, DEVPROPTYPE *, PBYTE, PULONG, ULONG); // rbx
  _BYTE v7[24]; // [rsp+30h] [rbp-18h] BYREF
  int v8; // [rsp+60h] [rbp+18h] BYREF
  char v9; // [rsp+68h] [rbp+20h] BYREF

  sub_14000179C();
  ModuleHandleA = GetModuleHandleA("cfgmgr32.dll");
  if ( !ModuleHandleA )
    return 0;
  CM_Get_DevNode_PropertyW = (CONFIGRET (__stdcall *)(DEVINST, const DEVPROPKEY *, DEVPROPTYPE *, PBYTE, PULONG, ULONG))GetProcAddress(ModuleHandleA, "CM_Get_DevNode_PropertyW");
  if ( !CM_Get_DevNode_PropertyW )
    return 0;
  v8 = 8;
  if ( ((unsigned int (__fastcall *)(_QWORD, void *, char *, _BYTE *, int *, _DWORD))CM_Get_DevNode_PropertyW)(
         a1,
         &unk_14000BF40,
         &v9,
         v7,
         &v8,
         0) )
  {
    v8 = 8;
    if ( ((unsigned int (__fastcall *)(_QWORD, void *, char *, _BYTE *, int *, _DWORD))CM_Get_DevNode_PropertyW)(
           a1,
           &unk_14000BF28,
           &v9,
           v7,
           &v8,
           0) )
    {
      return 0;
    }
  }
  *a2 = sub_140007700((__int64)v7);
  return 1;
}
