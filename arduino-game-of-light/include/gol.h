
class gameOfLifeGrid {
public:
  int _nRow;
  int _nCol;
  bool* _matrix;

  gameOfLifeGrid(int row, int col);
  bool get_cell_at_index(int index);
  void set_cell_at_index(int index,bool new_value);
  void tick();

private:
  int   count_alive_neighboors(int index);
  bool  is_cell_in_bounds(int rowIndex, int colIndex);
  // Utility function
  int   row_from_index(int index);
  int   col_from_index(int index);
  int   index_from_row_and_col(int row, int col);
};


// -------------------------------- //
//  CONSTRUCTOR
// -------------------------------- //
gameOfLifeGrid::gameOfLifeGrid(int row, int col)
  {
    this->_nRow = row;
    this->_nCol = col;
    this->_matrix = new bool[row*col];
    for (int index = 0; index < row*col; ++index){
      this->_matrix[index] = false;
    }
  }

  // TODO: destructor delete [] _matrix;

  // -------------------------------- //
  //  PUBLIC
  // -------------------------------- //
  bool gameOfLifeGrid::get_cell_at_index(int index){
    return _matrix[index];
  }
  void gameOfLifeGrid::set_cell_at_index(int index, bool new_value){
    _matrix[index]=new_value;
  }
  void gameOfLifeGrid::tick(){
    for (int index = 0; index < _nRow * _nCol; ++index){
      // Manage dying cells
      if ((get_cell_at_index(index) && count_alive_neighboors(index) < 2 )||
          (get_cell_at_index(index) && count_alive_neighboors(index) > 3) ){
        set_cell_at_index(index, false);
      }
      // Manage Rising cells
      else if (!get_cell_at_index(index) && count_alive_neighboors(index) == 3 ){
        set_cell_at_index(index, true);
      }
    }
  }




  // -------------------------------- //
  //  PRIVATE
  // -------------------------------- //
  bool gameOfLifeGrid::is_cell_in_bounds(int rowIndex, int colIndex){
    return ((rowIndex >= 0 && rowIndex < _nRow) && (colIndex >= 0 && colIndex < _nCol));
  }
  int gameOfLifeGrid::count_alive_neighboors(int index){
    int live_cell_count = 0;
    int y = row_from_index(index);
    int x = col_from_index(index);

    for (int dx = (x > 0 ? -1 : 0); dx <= (x < _nRow ? 1 : 0); ++dx)
    {
        for (int dy = (y > 0 ? -1 : 0); dy <= (y < _nCol ? 1 : 0); ++dy)
        {
            if (dx != 0 || dy != 0)
              live_cell_count++;
        }
    }
    return live_cell_count;
  }

  // -------------------------------- //
  // Utility
  // -------------------------------- //
  int   gameOfLifeGrid::col_from_index(int index){
    return index % _nCol;
  }
  int   gameOfLifeGrid::row_from_index(int index){
    return index / _nCol;
  }
  int   gameOfLifeGrid::index_from_row_and_col(int row, int col){
    return col + _nCol * row;
  }

// class game_grid
// {
// public:
//   //creates new X*Y game grid
//   //with all players at zero steps
//   game_grid(int x, int y); // x == columns, y == rows

//   //NEW: l-value ref qualifier
//   game_grid set_grid_cell(int index, bool new_val) const &;

//   //NEW: r-value ref qualifier
//   game_grid set_grid_cell(int index, bool new_val) &&;

//   //NEW: getter method
//   int get_grid_cell(int index) const
//   {
//     return grid_cells[index];
//   };

// private:
//   //NEW: mutable & private member
//   bool* grid_cells;

// };

/// ---------------------------///

// typedef struct {
//   int nCol; // X
//   int nRow; // Y
//   bool* matrix; // [...]
// } GameOfLifeGrid;

// GameOfLifeGrid create_all_dead_grid(int col, int row)
// {
//   GameOfLifeGrid all_dead_matrix = {col, row, new bool[row * col]};
//   for (int i = 0; i < row * col; i++)
//     all_dead_matrix.matrix[i] = false;
//   return all_dead_matrix;
// }
/// ---------------------------///