/*
 * func-name: sub_140009C34
 * func-address: 0x140009c34
 * callers: 0x140009199, 0x140009224, 0x1400092a9, 0x140009382, 0x14000948b, 0x140009510, 0x14000993d, 0x140009a3a, 0x140009ba3
 * callees: none
 */

FARPROC __fastcall sub_140009C34(_DWORD *a1, char *a2)
{
  __int64 v2; // rax
  int v3; // r9d
  __int64 v4; // r12
  __int64 v5; // r10
  __int64 v7; // rcx
  __int64 v8; // r14
  int v9; // r15d
  volatile __int64 *v11; // r12
  char *v12; // r10
  char *v13; // rcx
  char *v14; // r14
  HMODULE LibraryA; // rdi
  __int64 v17; // rbp
  __int64 (__fastcall *v18)(__int64, int *); // rax
  FARPROC ProcAddress; // rbx
  _QWORD *v20; // rax
  __int64 v21; // rax
  int v22; // [rsp+20h] [rbp-88h] BYREF
  _DWORD *v23; // [rsp+28h] [rbp-80h]
  char *v24; // [rsp+30h] [rbp-78h]
  LPCSTR lpLibFileName; // [rsp+38h] [rbp-70h]
  BOOL v26; // [rsp+40h] [rbp-68h]
  LPCSTR lpProcName; // [rsp+48h] [rbp-60h]
  HMODULE v28; // [rsp+50h] [rbp-58h]
  INT_PTR (__stdcall *v29)(); // [rsp+58h] [rbp-50h]
  DWORD LastError; // [rsp+60h] [rbp-48h]
  int *v31; // [rsp+B0h] [rbp+8h] BYREF
  ULONG_PTR Arguments; // [rsp+B8h] [rbp+10h] BYREF
  ULONG_PTR v33; // [rsp+C0h] [rbp+18h] BYREF

  v2 = (unsigned int)a1[1];
  v3 = *a1;
  v4 = (unsigned int)a1[2];
  v5 = (unsigned int)a1[3];
  v22 = 72;
  v7 = (unsigned int)a1[4];
  v8 = (unsigned int)a1[5];
  v9 = a1[7];
  v23 = a1;
  v24 = a2;
  lpLibFileName = (char *)&_ImageBase + v2;
  v26 = 0;
  v28 = 0;
  v29 = 0;
  lpProcName = 0;
  LastError = 0;
  v11 = (volatile __int64 *)((char *)&_ImageBase + v4);
  v12 = (char *)&_ImageBase + v5;
  v13 = (char *)&_ImageBase + v7;
  v14 = (char *)&_ImageBase + v8;
  if ( (v3 & 1) == 0 )
  {
    v31 = &v22;
    RaiseException(0xC06D0057, 0, 1u, (const ULONG_PTR *)&v31);
    return 0;
  }
  LibraryA = (HMODULE)*v11;
  v17 = (unsigned int)((a2 - v12) >> 3);
  v26 = *(_QWORD *)&v13[8 * v17] >= 0LL;
  if ( v26 )
    lpProcName = (char *)&word_140000002 + *(unsigned int *)&v13[8 * v17];
  else
    LODWORD(lpProcName) = *(unsigned __int16 *)&v13[8 * v17];
  v18 = (__int64 (__fastcall *)(__int64, int *))qword_1400170C0;
  ProcAddress = 0;
  if ( qword_1400170C0 )
  {
    ProcAddress = (FARPROC)qword_1400170C0(0, &v22);
    if ( ProcAddress )
      goto LABEL_37;
    v18 = (__int64 (__fastcall *)(__int64, int *))qword_1400170C0;
  }
  if ( !LibraryA )
  {
    if ( !v18 || (LibraryA = (HMODULE)v18(1, &v22)) == 0 )
    {
      LibraryA = LoadLibraryA(lpLibFileName);
      if ( !LibraryA )
      {
        LastError = GetLastError();
        if ( !qword_1400170B8 || (LibraryA = (HMODULE)qword_1400170B8(3, &v22)) == 0 )
        {
          Arguments = (ULONG_PTR)&v22;
          RaiseException(0xC06D007E, 0, 1u, &Arguments);
          return v29;
        }
      }
    }
    if ( (HMODULE)_InterlockedExchange64(v11, (__int64)LibraryA) == LibraryA )
    {
      FreeLibrary(LibraryA);
    }
    else if ( a1[6] )
    {
      v20 = LocalAlloc(0x40u, 0x10u);
      if ( v20 )
      {
        v20[1] = a1;
        *v20 = qword_1400170B0;
        qword_1400170B0 = (__int64)v20;
      }
    }
    v18 = (__int64 (__fastcall *)(__int64, int *))qword_1400170C0;
  }
  v28 = LibraryA;
  if ( v18 )
    ProcAddress = (FARPROC)v18(2, &v22);
  if ( !ProcAddress )
  {
    if ( !a1[5]
      || !a1[7]
      || (v21 = *((int *)LibraryA + 15), *(_DWORD *)((char *)LibraryA + v21) != 17744)
      || *(_DWORD *)((char *)LibraryA + v21 + 8) != v9
      || LibraryA != *(HMODULE *)((char *)LibraryA + v21 + 48)
      || (ProcAddress = *(FARPROC *)&v14[8 * v17]) == 0 )
    {
      ProcAddress = GetProcAddress(LibraryA, lpProcName);
      if ( !ProcAddress )
      {
        LastError = GetLastError();
        if ( qword_1400170B8 )
          ProcAddress = (FARPROC)qword_1400170B8(4, &v22);
        if ( !ProcAddress )
        {
          v33 = (ULONG_PTR)&v22;
          RaiseException(0xC06D007F, 0, 1u, &v33);
          ProcAddress = v29;
        }
      }
    }
  }
  *(_QWORD *)a2 = ProcAddress;
LABEL_37:
  if ( qword_1400170C0 )
  {
    LastError = 0;
    v28 = LibraryA;
    v29 = ProcAddress;
    qword_1400170C0(5, &v22);
  }
  return ProcAddress;
}
