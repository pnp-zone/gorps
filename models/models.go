package models

import "github.com/myOmikron/echotools/utilitymodels"

type Default struct {
	utilitymodels.CommonSoftDelete
	Name           string `json:"name"`
	Specialization string `json:"specialization"`
	Modifier       int    `json:"modifier"`
}

type Skill struct {
	utilitymodels.CommonSoftDelete
	Name           string     `json:"name"`
	Reference      string     `json:"reference"`
	Difficulty     string     `json:"difficulty"`
	Points         int        `json:"points"`
	Limit          int        `json:"limit"`
	TechLevel      string     `json:"tech_level"`
	Specialization string     `json:"specialization"`
	Notes          string     `json:"notes"`
	Defaults       []*Default `json:"defaults" gorm:"many2many:skill_default;"`
}

type Modifier struct {
	utilitymodels.CommonSoftDelete
	Name          string  `json:"name"`
	BookReference string  `json:"book_reference"`
	Cost          float64 `json:"cost"`
	CostType      string  `json:"cost_type"`
	Affects       string  `json:"affects"`
	Notes         string  `json:"notes"`
}

type Feature struct {
	utilitymodels.CommonSoftDelete
	Name          string  `json:"name"`
	BookReference string  `json:"book_reference"`
	Amount        float64 `json:"amount"`
	PerLevel      bool    `json:"per_level"`
	Attribute     string  `json:"attribute"`
	Situation     string  `json:"situation"`
}

type PreReqs struct {
	utilitymodels.CommonSoftDelete
	Has bool
}

type Advantage struct {
	utilitymodels.CommonSoftDelete
	Name           string      `json:"name"`
	Physical       bool        `json:"physical"`
	Exotic         bool        `json:"exotic"`
	Supernatural   bool        `json:"supernatural"`
	Mental         bool        `json:"mental"`
	Social         bool        `json:"social"`
	BasePoints     float64     `json:"base_points"`
	Levels         int         `json:"levels"`
	PointsPerLevel int         `json:"points_per_level"`
	BookReference  string      `json:"book_reference"`
	Notes          string      `json:"notes"`
	ControlRole    int         `json:"control_role"`
	ControlRoleAdj string      `json:"control_role_adj"`
	Modifier       []*Modifier `json:"modifier" gorm:"many2many:advantage_modifier;"`
	Features       []*Feature  `json:"features" gorm:"many2many:advantage_features;"`
}

type Disadvantage struct {
	utilitymodels.CommonSoftDelete
	Name           string      `json:"name"`
	Physical       bool        `json:"physical"`
	Exotic         bool        `json:"exotic"`
	Supernatural   bool        `json:"supernatural"`
	Mental         bool        `json:"mental"`
	Social         bool        `json:"social"`
	BasePoints     float64     `json:"base_points"`
	Levels         int         `json:"levels"`
	PointsPerLevel int         `json:"points_per_level"`
	BookReference  string      `json:"book_reference"`
	Notes          string      `json:"notes"`
	ControlRole    int         `json:"control_role"`
	ControlRoleAdj string      `json:"control_role_adj"`
	Modifier       []*Modifier `json:"modifier" gorm:"many2many:disadvantage_modifier;"`
	Features       []*Feature  `json:"features" gorm:"many2many:disadvantage_features;"`
}

type Quirk struct {
	utilitymodels.CommonSoftDelete
	Name           string      `json:"name"`
	Physical       bool        `json:"physical"`
	Exotic         bool        `json:"exotic"`
	Supernatural   bool        `json:"supernatural"`
	Mental         bool        `json:"mental"`
	Social         bool        `json:"social"`
	BasePoints     float64     `json:"base_points"`
	Levels         int         `json:"levels"`
	PointsPerLevel int         `json:"points_per_level"`
	BookReference  string      `json:"book_reference"`
	Notes          string      `json:"notes"`
	ControlRole    int         `json:"control_role"`
	ControlRoleAdj string      `json:"control_role_adj"`
	Modifier       []*Modifier `json:"modifier" gorm:"many2many:quirk_modifier;"`
	Features       []*Feature  `json:"features" gorm:"many2many:quirk_features;"`
}

type Perk struct {
	utilitymodels.CommonSoftDelete
	Name           string      `json:"name"`
	Physical       bool        `json:"physical"`
	Exotic         bool        `json:"exotic"`
	Supernatural   bool        `json:"supernatural"`
	Mental         bool        `json:"mental"`
	Social         bool        `json:"social"`
	BasePoints     float64     `json:"base_points"`
	Levels         int         `json:"levels"`
	PointsPerLevel int         `json:"points_per_level"`
	BookReference  string      `json:"book_reference"`
	Notes          string      `json:"notes"`
	ControlRole    int         `json:"control_role"`
	ControlRoleAdj string      `json:"control_role_adj"`
	Modifier       []*Modifier `json:"modifier" gorm:"many2many:perk_modifier;"`
	Features       []*Feature  `json:"features" gorm:"many2many:perk_features;"`
}

type Character struct {
	utilitymodels.CommonSoftDelete
}
