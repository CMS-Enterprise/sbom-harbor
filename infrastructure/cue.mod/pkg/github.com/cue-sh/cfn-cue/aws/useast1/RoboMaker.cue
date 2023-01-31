package useast1

import (
	"github.com/cue-sh/cfn-cue/aws/fn"
	"strings"
)

#RoboMaker: {
	#Fleet: {
		Type: "AWS::RoboMaker::Fleet"
		Properties: {
			Name?: *(strings.MinRunes(1) & strings.MaxRunes(255) & (=~#"[a-zA-Z0-9_\-]{1,255}$"#)) | fn.#Fn
			Tags?: *{
				[string]: *string | fn.#Fn
			} | fn.#If
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#Robot: {
		Type: "AWS::RoboMaker::Robot"
		Properties: {
			Architecture:      *("X86_64" | "ARM64" | "ARMHF") | fn.#Fn
			Fleet?:            *(strings.MinRunes(1) & strings.MaxRunes(1224)) | fn.#Fn
			GreengrassGroupId: *(strings.MinRunes(1) & strings.MaxRunes(1224)) | fn.#Fn
			Name?:             *(strings.MinRunes(1) & strings.MaxRunes(255)) | fn.#Fn
			Tags?:             *{
				[string]: *string | fn.#Fn
			} | fn.#If
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#RobotApplication: {
		Type: "AWS::RoboMaker::RobotApplication"
		Properties: {
			CurrentRevisionId?: *string | fn.#Fn
			Name?:              *string | fn.#Fn
			RobotSoftwareSuite: *{
				Name:    *string | fn.#Fn
				Version: *string | fn.#Fn
			} | fn.#If
			Sources: *[...{
				Architecture: *string | fn.#Fn
				S3Bucket:     *string | fn.#Fn
				S3Key:        *string | fn.#Fn
			}] | fn.#If
			Tags?: *{
				[string]: _
			} | fn.#Fn
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#RobotApplicationVersion: {
		Type: "AWS::RoboMaker::RobotApplicationVersion"
		Properties: {
			Application:        *(=~#"arn:[\w+=/,.@-]+:[\w+=/,.@-]+:[\w+=/,.@-]*:[0-9]*:[\w+=,.@-]+(/[\w+=,.@-]+)*"#) | fn.#Fn
			CurrentRevisionId?: *(strings.MinRunes(1) & strings.MaxRunes(40) & (=~#"[a-zA-Z0-9_.\-]*"#)) | fn.#Fn
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#SimulationApplication: {
		Type: "AWS::RoboMaker::SimulationApplication"
		Properties: {
			CurrentRevisionId?: *string | fn.#Fn
			Environment?:       *string | fn.#Fn
			Name?:              *(strings.MinRunes(1) & strings.MaxRunes(255) & (=~#"[a-zA-Z0-9_\-]*"#)) | fn.#Fn
			RenderingEngine?:   *{
				Name:    *("OGRE") | fn.#Fn
				Version: *(=~#"1.x"#) | fn.#Fn
			} | fn.#If
			RobotSoftwareSuite: *{
				Name:    *("ROS" | "ROS2") | fn.#Fn
				Version: *("Kinetic" | "Melodic" | "Dashing" | "Foxy") | fn.#Fn
			} | fn.#If
			SimulationSoftwareSuite: *{
				Name:    *("Gazebo" | "RosbagPlay") | fn.#Fn
				Version: *("7" | "9" | "11" | "Kinetic" | "Melodic" | "Dashing" | "Foxy") | fn.#Fn
			} | fn.#If
			Sources?: *[...{
				Architecture: *("X86_64" | "ARM64" | "ARMHF") | fn.#Fn
				S3Bucket:     *(=~#"[a-z0-9][a-z0-9.\-]*[a-z0-9]"#) | fn.#Fn
				S3Key:        *(strings.MinRunes(1) & strings.MaxRunes(1024)) | fn.#Fn
			}] | fn.#If
			Tags?: *{
				[string]: *string | fn.#Fn
			} | fn.#If
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
	#SimulationApplicationVersion: {
		Type: "AWS::RoboMaker::SimulationApplicationVersion"
		Properties: {
			Application:        *(=~#"arn:[\w+=/,.@-]+:[\w+=/,.@-]+:[\w+=/,.@-]*:[0-9]*:[\w+=,.@-]+(/[\w+=,.@-]+)*"#) | fn.#Fn
			CurrentRevisionId?: *(strings.MinRunes(1) & strings.MaxRunes(40) & (=~#"[a-zA-Z0-9_.\-]*"#)) | fn.#Fn
		}
		DependsOn?:           string | [...string]
		DeletionPolicy?:      "Delete" | "Retain"
		UpdateReplacePolicy?: "Delete" | "Retain"
		Metadata?: [string]: _
		Condition?: string
	}
}
