#pragma once

#include <string>
#include <cstdint>
#include <vector>

// this uses some old std:: stuff ,and seems structure is a bit diffrent.
// seems to just be a padding at the start, so not that bad
//
namespace aistd
{

	class _pad
	{
		int _0x0000;
	};

	class string
	    : _pad
	    , public std::string
	{
  public:
	};

	class wstring
	    : _pad
	    , public std::wstring
	{
  public:
	};

  template <typename T>
	class vector 
	    : _pad
	    , public std::vector<T>
	{
  public:
	};

	template <typename TKEY, typename TVALUE>
	class map_entry
	{
  public:
		map_entry<TKEY, TVALUE>* m_pLeft;
		map_entry<TKEY, TVALUE>* m_pRight;
		map_entry<TKEY, TVALUE>* m_pParent;

		char _0x000c[0x4];
		TKEY key;
		TVALUE value;
	};

	template <typename TKEY, typename TVALUE>
	class map 
    : _pad
	{
  public:
		map_entry<TKEY, TVALUE>* m_pRoot;
		int m_Count;
	};

} // namespace aistd
