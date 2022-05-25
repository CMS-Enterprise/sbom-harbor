from os import getenv
from dotenv import dotenv_values

env_vars = (
  "AWS_ACCOUNT_NUM",
  "AWS_DEFAULT_REGION",
  "AWS_REGION",
)

class Environment():

  def __init__(self) -> dict:

      # load variables from .env file in root directory
      env = {
        **dotenv_values(".env"),
      }

      # load overrides from environment variables
      for key in env_vars:
        value = getenv(key)
        if value is not None:
          env[key] = value

      # set _env to loaded env object
      self._env = env

  def get_all(self) -> dict:
    return self._env

  def get_aws_account(self) -> str:
    return self._env["AWS_ACCOUNT_NUM"]
