/*
 * func-name: sub_140008EA0
 * func-address: 0x140008ea0
 * callers: 0x1400052dc
 * callees: 0x140001854, 0x140009212
 */

__int64 __fastcall sub_140008EA0(__int64 a1)
{
  LONG v3; // eax
  unsigned int v4; // ebx
  GUID pgActionID; // [rsp+20h] [rbp-88h] BYREF
  int v6; // [rsp+30h] [rbp-78h] BYREF
  __int64 v7; // [rsp+38h] [rbp-70h]
  __int64 v8; // [rsp+40h] [rbp-68h]
  __int64 v9; // [rsp+48h] [rbp-60h]
  int pWVTData; // [rsp+50h] [rbp-58h] BYREF
  __int64 v11; // [rsp+58h] [rbp-50h]
  __int64 v12; // [rsp+60h] [rbp-48h]
  int v13; // [rsp+68h] [rbp-40h]
  int v14; // [rsp+6Ch] [rbp-3Ch]
  int v15; // [rsp+70h] [rbp-38h]
  int *v16; // [rsp+78h] [rbp-30h]
  int v17; // [rsp+80h] [rbp-28h]
  __int64 v18; // [rsp+88h] [rbp-20h]
  __int64 v19; // [rsp+90h] [rbp-18h]
  int v20; // [rsp+9Ch] [rbp-Ch]

  if ( dword_140010668 < 6 )
    return 0;
  sub_140001854(&v6, 0, 0x20u);
  v8 = 0;
  v9 = 0;
  v7 = a1;
  pgActionID.Data2 = -12988;
  v6 = 32;
  pgActionID.Data3 = 4560;
  pgActionID.Data1 = 11191659;
  pgActionID.Data4[0] = -116;
  pgActionID.Data4[1] = -62;
  pgActionID.Data4[2] = 0;
  pgActionID.Data4[3] = -64;
  pgActionID.Data4[4] = 79;
  pgActionID.Data4[5] = -62;
  pgActionID.Data4[6] = -107;
  pgActionID.Data4[7] = -18;
  sub_140001854(&pWVTData, 0, 0x50u);
  v11 = 0;
  v12 = 0;
  v14 = 0;
  v18 = 0;
  v19 = 0;
  v20 = 0;
  v15 = 1;
  v17 = 1;
  v16 = &v6;
  pWVTData = 80;
  v13 = 2;
  v3 = WinVerifyTrust(0, &pgActionID, &pWVTData);
  v17 = 2;
  v4 = v3;
  WinVerifyTrust(0, &pgActionID, &pWVTData);
  return v4;
}
