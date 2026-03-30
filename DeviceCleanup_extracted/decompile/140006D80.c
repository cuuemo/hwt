/*
 * func-name: sub_140006D80
 * func-address: 0x140006d80
 * callers: 0x1400052dc
 * callees: 0x140001854, 0x140007428
 */

__int64 __fastcall sub_140006D80(__int64 a1, WCHAR *a2)
{
  HMODULE v4; // rax
  HMODULE v5; // rbx
  BOOL (__stdcall *GetFileVersionInfoW)(LPCWSTR, DWORD, DWORD, LPVOID); // rbp
  BOOL (__stdcall *VerQueryValueW)(LPCVOID, LPCWSTR, LPVOID *, PUINT); // rax
  BOOL (__stdcall *v8)(LPCVOID, LPCWSTR, LPVOID *, PUINT); // rsi
  unsigned __int16 *v9; // rax
  unsigned int v10; // r10d
  unsigned int v11; // r8d
  int v12; // edx
  int v13; // ecx
  int v14; // eax
  __int64 v15; // r9
  __int64 v16; // r8
  unsigned int v17; // eax
  int v19; // [rsp+20h] [rbp-228h]
  __int64 v20; // [rsp+20h] [rbp-228h]
  int v21; // [rsp+28h] [rbp-220h]
  _BYTE v22[512]; // [rsp+30h] [rbp-218h] BYREF
  unsigned int v23; // [rsp+260h] [rbp+18h] BYREF
  unsigned __int16 *v24; // [rsp+268h] [rbp+20h] BYREF

  v4 = (HMODULE)sub_140007428("version.dll");
  v5 = v4;
  if ( !v4 )
    return 0;
  GetFileVersionInfoW = (BOOL (__stdcall *)(LPCWSTR, DWORD, DWORD, LPVOID))GetProcAddress(v4, "GetFileVersionInfoW");
  VerQueryValueW = (BOOL (__stdcall *)(LPCVOID, LPCWSTR, LPVOID *, PUINT))GetProcAddress(v5, "VerQueryValueW");
  v8 = VerQueryValueW;
  if ( !GetFileVersionInfoW )
    return 0;
  if ( !VerQueryValueW )
    return 0;
  v24 = 0;
  sub_140001854(a2, 0, 0x4Cu);
  if ( !((unsigned int (__fastcall *)(__int64, _QWORD, __int64, _BYTE *))GetFileVersionInfoW)(a1, 0, 512, v22)
    || !((unsigned int (__fastcall *)(_BYTE *, const WCHAR *, unsigned __int16 **, unsigned int *))v8)(
          v22,
          L"\\",
          &v24,
          &v23)
    || !v23 )
  {
    return 0;
  }
  v9 = v24;
  v10 = v24[5];
  *a2 = v10;
  v11 = v9[4];
  a2[1] = v11;
  v12 = v9[7];
  a2[2] = v12;
  v13 = v9[6];
  a2[3] = v13;
  *((_DWORD *)a2 + 2) = *((_DWORD *)v9 + 2);
  v21 = v13;
  v19 = v12;
  v14 = wsprintfW(a2 + 6, L"%u.%u.%u.%u", v10, v11, v19, v21);
  v15 = a2[1];
  v16 = *a2;
  v23 = v14;
  LODWORD(v20) = a2[2];
  v17 = wsprintfW(a2 + 22, L"%u.%u.%u", v16, v15, v20);
  v23 = v17;
  if ( (unsigned __int16)word_1400106C6 >= 0xAu )
    wsprintfW(&a2[v17 + 22], L".%u", a2[3]);
  FreeLibrary(v5);
  return 1;
}
