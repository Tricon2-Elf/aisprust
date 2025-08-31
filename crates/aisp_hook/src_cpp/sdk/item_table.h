#pragma once


#include <cstdint>
#include "aistd.h"


class ITEM_DATA
{
public:
  uint32_t key;
  int sorted_list_priority;
  int item_id;
  uint32_t skill_id;
  char name[97];
  char _0x0069[3];
  uint32_t category_skilleq20; //0x006c
  uint32_t bodyspot1;
  uint32_t bodyspot2_selectable;
  char description[769]; //0x078
  char limit_description[195]; //0x0379
  uint32_t flags; //0x043c
  uint16_t word_448;
  char _0x044A[2];
  uint32_t dword_44c;
  uint32_t dword_450;
  uint32_t dword_454;
  uint32_t dword_458;
}; //0x0045c


class CItemTable
{
public:
  virtual ~CItemTable();


  aistd::map<uint32_t, ITEM_DATA> m_Items; //0x0004
  aistd::vector<ITEM_DATA*> m_CategoryItems[24]; //0x0010
  aistd::vector<ITEM_DATA*> m_SortedCategoryTypes[24]; //0x0190
  uint32_t _0x0310;
  uint32_t _0x0314;
  uint32_t _0x0318;
  uint32_t m_BaseItemId;

  uint32_t _0x0320;
  uint32_t _0x0324;
  uint32_t m_BaseSkillId;
  uint32_t _0x032c;

};
