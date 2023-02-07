import logging
from logging import config

from cyclonedx.constants import PYTHON_LOGGING_CONFIG

# TODO: May need to manually make a root logger if not using AWS
# TODO: Keep these comments for now
# logging.basicConfig()
# logging.root.setLevel(logging.ERROR)
# logging.raiseExceptions = True
# logging.basicConfig(level=logging.ERROR)
# config.fileConfig(PYTHON_LOGGING_CONFIG)
harbor_logger = logging.getLogger()
harbor_logger.setLevel(logging.INFO)
