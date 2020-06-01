/*
 * include/controller.hpp
 * tasinput2 - Plugin for creating TAS inputs
 *
 * This file is part of tasinput2.
 *
 * tasinput2 is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * tasinput2 is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with tasinput2.  If not, see <https://www.gnu.org/licenses/>.
 */

#ifndef CONTROLLER_HPP
#define CONTROLLER_HPP

#include <QCheckBox>
#include <QWidget>
#include "inputs.hpp"

enum DirectionalType {DT_C,DT_D};

class DirectionalPanel : public QWidget {
    Q_OBJECT
  
 private:
  QCheckBox *up;
  QCheckBox *down;
  QCheckBox *left;
  QCheckBox *right;
  DirectionalType ty;

  void up_checked();
  void down_checked();
  void left_checked();
  void right_checked();
  Directional *get_directional();

  Inputs *inputs;

 public:
  explicit DirectionalPanel(QWidget *parent, DirectionalType ty, Inputs *inputs);
};

class ButtonPanel : public QWidget {
  Q_OBJECT

  private:
    QCheckBox *a;
    QCheckBox *b;
    
    Inputs *inputs;
  public:
    explicit ButtonPanel(QWidget *parent, Inputs *inputs);
};

class Controller : public QWidget {
  Q_OBJECT

 public:
  Inputs *inputs;

  explicit Controller(QWidget *parent = 0);
  virtual ~Controller();
};

#endif
