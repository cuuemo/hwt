/*
 * func-name: sub_140001130
 * func-address: 0x140001130
 * callers: 0x140008320, 0x140009100
 * callees: 0x140001020, 0x140001898, 0x140008158, 0x1400081f4
 */

__int64 __fastcall sub_140001130(__int64 a1)
{
  __int64 result; // rax
  __int64 v3; // rax
  int v4; // r9d
  __int64 v5; // rax
  __int64 v6; // r9
  int v7; // eax
  __int64 v8; // r9
  __int64 v9; // r10
  int i; // r8d
  BOOL v11; // ecx
  SIZE_T dwBytes; // [rsp+48h] [rbp+10h] BYREF

  sub_140001020(a1);
  LODWORD(dwBytes) = 0;
  result = sub_140008158(-2147483646, L"SYSTEM\\CurrentControlSet\\Control\\COM Name Arbiter", L"ComDB", &dwBytes);
  if ( (_DWORD)result )
  {
    v3 = sub_140001898((unsigned int)dwBytes);
    v4 = dwBytes;
    *(_QWORD *)(a1 + 8) = v3;
    v5 = sub_140001898(saturated_mul((unsigned int)(8 * v4), 4u));
    v6 = *(_QWORD *)(a1 + 8);
    *(_QWORD *)a1 = v5;
    v7 = sub_1400081F4(-2147483646, L"SYSTEM\\CurrentControlSet\\Control\\COM Name Arbiter", L"ComDB", v6, dwBytes);
    v8 = 0;
    v9 = 0;
    for ( *(_DWORD *)(a1 + 16) = 8 * v7; v8 < v7; ++v8 )
    {
      for ( i = 0; i < 8; ++i )
      {
        v11 = (*(_BYTE *)(v8 + *(_QWORD *)(a1 + 8)) & (unsigned __int8)(1 << i)) != 0;
        *(_DWORD *)(*(_QWORD *)a1 + 4 * v9++) = v11;
      }
    }
    return *(unsigned int *)(a1 + 16);
  }
  else
  {
    *(_DWORD *)(a1 + 16) = 0;
  }
  return result;
}
